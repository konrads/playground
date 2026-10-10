use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::sync::LazyLock;

fn to_dec_digits(mut n: i32) -> Vec<u8> {
    let mut result = vec![];
    while n > 0 {
        result.push(n as u8 % 10);
        n /= 10;
    }
    result.into_iter().rev().collect()
}

fn to_bin_digits(mut n: i32) -> Vec<u8> {
    let mut result = vec![];
    while n > 0 {
        result.push(n as u8 % 2);
        n >>= 1;
    }
    result.into_iter().rev().collect()
}

fn pattern_match(pattern: &str, s: &str) -> bool {
    pattern
        .chars()
        .zip(s.chars())
        .all(|(p, c)| p == c || p == '?')
}

static MONTH_DATES: LazyLock<BTreeMap<&str, u8>> = LazyLock::new(|| {
    let mut m = BTreeMap::new();
    m.insert("01", 31);
    m.insert("02", 28);
    m.insert("03", 31);
    m.insert("04", 30);
    m.insert("05", 31);
    m.insert("06", 30);
    m.insert("07", 31);
    m.insert("08", 31);
    m.insert("09", 30);
    m.insert("10", 31);
    m.insert("11", 30);
    m.insert("12", 31);
    m
});

fn all_dates() -> Vec<String> {
    MONTH_DATES
        .iter()
        .flat_map(|(&k, &v)| (1..=v as usize).map(move |v| format!("{k}-{v:02}")))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bin_digits() {
        assert_eq!(to_bin_digits(123), vec![1, 1, 1, 1, 0, 1, 1]);
    }

    #[test]
    fn test_to_dec_digits() {
        assert_eq!(to_dec_digits(123), vec![1, 2, 3]);
    }

    #[test]
    fn test_pattern_match() {
        assert!(pattern_match("abc", "abc"));
        assert!(pattern_match("?bc", "abc"));
        assert!(pattern_match("?bc", "bbc"));
    }

    #[test]
    fn test_all_dates() {
        let dates = all_dates();
        assert_eq!(dates.len(), 365);
        assert_eq!(dates[5], "01-06");
        assert_eq!(dates[364], "12-31");
    }
}
