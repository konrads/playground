/*
Given six digits A, B, C, D, E, F (each 0-9), return the earliest valid
24-hour time "hh:mm:ss" that uses all six digits, or "NOT POSSIBLE".

Examples:
  1, 8, 3, 2, 6, 4 -> "12:36:48"
  0, 0, 0, 7, 8, 9 -> "07:08:09"
  2, 4, 5, 9, 5, 9 -> "NOT POSSIBLE"
*/

use std::collections::HashSet;

fn is_valid_time(t: &[i32; 6]) -> bool {
    matches!(
        t,
        [0..=1, _, 0..=5, _, 0..=5, _] | [2, 0..=3, 0..=5, _, 0..=5, _]
    )
}

pub fn solution_v0(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> String {
    let digits = [a, b, c, d, e, f];
    let mut result: Option<String> = None;

    // expand: every placement of the digits, as positions in indices (e.g. [0, 3, 2, 4, 5, 1])
    // and as digits in t (e.g. [1, 2, 3, 6, 4, 8])
    for i0 in 0..6 {
        for i1 in 0..6 {
            for i2 in 0..6 {
                for i3 in 0..6 {
                    for i4 in 0..6 {
                        for i5 in 0..6 {
                            let indices = [i0, i1, i2, i3, i4, i5];
                            let t = indices.map(|i| digits[i]);
                            // filter: each digit used once, and a valid time (rejects e.g. t = [8, 1, 3, 2, 6, 4])
                            if indices.iter().collect::<HashSet<_>>().len() == 6 && is_valid_time(&t) {
                                // reduce: keep the earliest candidate (e.g. "12:36:48") in result
                                let candidate = format!(
                                    "{}{}:{}{}:{}{}",
                                    t[0], t[1], t[2], t[3], t[4], t[5]
                                );
                                if result.as_ref().is_none_or(|r| candidate < *r) {
                                    result = Some(candidate);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    result.unwrap_or_else(|| "NOT POSSIBLE".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v0_examples() {
        assert_eq!(solution_v0(1, 8, 3, 2, 6, 4), "12:36:48");
        assert_eq!(solution_v0(0, 0, 0, 7, 8, 9), "07:08:09");
        assert_eq!(solution_v0(2, 4, 5, 9, 5, 9), "NOT POSSIBLE");
    }
}
