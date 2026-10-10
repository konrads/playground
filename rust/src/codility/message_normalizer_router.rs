//! Message normaliser and router. Parse inbound messages from two or three formats (e.g. JSON, a key-value text protocol, a length-prefixed format) into one canonical struct, then route by rules (tenant, channel, priority).
//! This tests parsing robustness: malformed input, missing fields, Unicode, and returning Result instead of panicking. It maps to "message routing and protocol translation across channels."

use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, Clone, Copy, PartialEq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    #[default]
    Low,
    High,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Message {
    pub tenant: String,
    #[serde(default)]
    pub priority: Priority,
    pub body: String,
}

pub fn parse_json(input: &str) -> Result<Message, String> {
    serde_json::from_str(input).map_err(|e| e.to_string())
}

// One `key=value` per line, turned into a JSON object so serde does the validation.
pub fn parse_key_value(input: &str) -> Result<Message, String> {
    let mut map = Map::new();
    for line in input.lines() {
        let (k, v) = line.split_once('=').ok_or(format!("malformed line: {line}"))?;
        map.insert(k.to_string(), Value::String(v.to_string()));
    }
    serde_json::from_value(Value::Object(map)).map_err(|e| e.to_string())
}

// `None` matches anything; the first matching rule wins.
#[derive(Deserialize)]
pub struct Rule {
    tenant: Option<String>,
    priority: Option<Priority>,
    destination: String,
}

pub fn route<'a>(rules: &'a [Rule], m: &Message) -> Option<&'a str> {
    rules
        .iter()
        .find(|r| {
            r.tenant.as_ref().is_none_or(|t| *t == m.tenant)
                && r.priority.is_none_or(|p| m.priority == p)
        })
        .map(|r| r.destination.as_str())
}

#[test]
fn parse_and_route() {
    let m = parse_key_value("tenant=acme\npriority=high\nbody=zażółć 🚀");
    assert_eq!(m, parse_json(r#"{"tenant":"acme","priority":"high","body":"zażółć 🚀"}"#));
    assert!(parse_json("{not json").is_err());
    assert!(parse_key_value("body=x").is_err());
    assert!(parse_key_value("nonsense").is_err());

    let rules: Vec<Rule> = serde_json::from_str(
        r#"[{"tenant": "acme", "priority": "high", "destination": "urgent"},
            {"destination": "default"}]"#,
    )
    .unwrap();
    let m = m.unwrap();
    assert_eq!(route(&rules, &m), Some("urgent"));
    assert_eq!(route(&rules, &Message { priority: Priority::Low, ..m }), Some("default"));
}
