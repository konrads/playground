/*
Given a date pattern "MM-DD" where any digit may be "?", return the latest
date in a non-leap year that matches it, or "xx-xx" if none does.

Examples:
  "?1-?4" -> "11-24"
  "0?-31" -> "08-31"
  "??-??" -> "12-31"
  "02-3?" -> "xx-xx"
*/

const DAYS_IN_MONTH: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn pattern_match(date: &str, pattern: &str) -> bool {
    date.chars().zip(pattern.chars()).all(|(c, p)| p == '?' || p == c)
}

pub fn solution_v0(s: &str) -> String {
    let mut result: Option<String> = None;

    // expand: every date of the year, as month and day (e.g. 11, 24) and as text in date (e.g. "11-24")
    for month in 1..=12 {
        for day in 1..=DAYS_IN_MONTH[month as usize - 1] {
            let date = format!("{month:02}-{day:02}");
            // filter: date matches the pattern (rejects e.g. date = "11-25")
            if pattern_match(&date, s) {
                // reduce: keep the latest candidate (e.g. "11-24") in result
                if result.as_ref().is_none_or(|r| date > *r) {
                    result = Some(date);
                }
            }
        }
    }

    result.unwrap_or_else(|| "xx-xx".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v0_examples() {
        assert_eq!(solution_v0("?1-?4"), "11-24");
        assert_eq!(solution_v0("0?-31"), "08-31");
        assert_eq!(solution_v0("??-??"), "12-31");
        assert_eq!(solution_v0("02-3?"), "xx-xx");
    }
}
