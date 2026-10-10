/*
Given a date pattern "MM-DD" where any digit may be "?",
return the latest date in a non-leap year that matches it,
or "xx-xx" if none does.
*/

use std::{collections::BTreeMap, sync::LazyLock};

static ALL_DATES: LazyLock<Vec<String>> = LazyLock::new(|| {
    let mut max_dates = BTreeMap::new();
    max_dates.insert("01", 31);
    max_dates.insert("02", 28);
    max_dates.insert("03", 31);
    max_dates.insert("04", 30);
    max_dates.insert("05", 31);
    max_dates.insert("06", 30);
    max_dates.insert("07", 31);
    max_dates.insert("08", 31);
    max_dates.insert("09", 30);
    max_dates.insert("10", 31);
    max_dates.insert("11", 30);
    max_dates.insert("12", 31);

    max_dates
        .into_iter()
        .flat_map(|(mm, d)| (1..=d).map(move |dd| format!("{mm}-{dd:02}")))
        .collect()
});

fn match_wildcards(pattern: &str, x: &str) -> bool {
    pattern
        .chars()
        .zip(x.chars())
        .all(|(p_char, x_char)| p_char == x_char || p_char == '?')
}

fn find_latest_date(pattern: &str) -> String {
    ALL_DATES
        .iter()
        .rev()
        .find(|date| match_wildcards(pattern, date))
        .cloned()
        .unwrap_or("xx-xx".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_dates() {
        assert_eq!(ALL_DATES.len(), 365);
        assert_eq!(ALL_DATES[0], "01-01");
        assert_eq!(ALL_DATES[364], "12-31");
    }

    #[test]
    fn test_match_wildcards() {
        assert!(match_wildcards("abc", "abc"));
        assert!(match_wildcards("?bc", "abc"));
        assert!(match_wildcards("a??", "abc"));
        assert!(!match_wildcards("abcd", "abc?"));
    }

    #[test]
    fn test_find_latest_date() {
        assert_eq!(find_latest_date("01-15"), "01-15");
        assert_eq!(find_latest_date("?1-?5"), "11-25");
        assert_eq!(find_latest_date("02-3?"), "xx-xx");
    }
}
