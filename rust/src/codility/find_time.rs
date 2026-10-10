/*

FindTime. It's in jonasraoni/codility, the most-starred repo under that GitHub topic;

Given six digits 0–9, return the earliest valid 24-hour time "hh:mm:ss" that uses all six,
or "NOT POSSIBLE".

1, 8, 3, 2, 6, 4 gives "12:36:48"
0, 0, 0, 7, 8, 9 gives "07:08:09"
2, 4, 5, 9, 5, 9 gives "NOT POSSIBLE"
It's the same kind of problem as your last_calendar_date: a small, fixed set of valid values
and a constraint to satisfy. Your approach carries over directly.
Generate all 86,400 times in order and return the first one whose digits, sorted,
equal the input digits sorted. The other option is to try the 720 orderings of the six digits.

For your first-and-last-digit pairs task, the matching one is PassingCars from Lesson 5
of the same repo. It also counts pairs, and the step from the O(n²) brute force to an O(n)
count is the same.

Brute force strategy:
- create all possible options from valid options:
  - hh: 23-00
  - mm: 60-00
  - ss: 60-00
*/

fn maybe_solution(digits: &[u8]) -> Option<String> {
    let mut best: Option<String> = None;

    for h1_i in 0..digits.len() {
        for h2_i in 0..digits.len() {
            if h1_i == h2_i {
                continue;
            }
            for m1_i in 0..digits.len() {
                if [h1_i, h2_i].contains(&m1_i) {
                    continue;
                }
                for m2_i in 0..digits.len() {
                    if [h1_i, h2_i, m1_i].contains(&m2_i) {
                        continue;
                    }
                    for s1_i in 0..digits.len() {
                        if [h1_i, h2_i, m1_i, m2_i].contains(&s1_i) {
                            continue;
                        }
                        for s2_i in 0..digits.len() {
                            if [h1_i, h2_i, m1_i, m2_i, s1_i].contains(&s2_i) {
                                continue;
                            }
                            // let have_unique_indices = [h1_i, h2_i, m1_i, m2_i, s1_i, s2_i]
                            //     .into_iter()
                            //   l  .collect::<HashSet<_>>()
                            //     .len()
                            //     == 6;
                            // if have_unique_indices {
                            match (
                                digits[h1_i],
                                digits[h2_i],
                                digits[m1_i],
                                digits[m2_i],
                                digits[s1_i],
                                digits[s2_i],
                            ) {
                                (h1 @ (0..=1), h2, m1 @ (..=5), m2, s1 @ (..=5), s2)
                                | (h1 @ 2, h2 @ (0..=3), m1 @ (..=5), m2, s1 @ (..=5), s2) => {
                                    let candidate = format!("{h1}{h2}:{m1}{m2}:{s1}{s2}");
                                    if best.as_ref().is_none_or(|b| &candidate < b) {
                                        best = Some(candidate);
                                    }
                                }
                                _ => {}
                            }
                            // }
                        }
                    }
                }
            }
        }
    }

    best
}

fn solution(digits: &[u8]) -> String {
    maybe_solution(digits).unwrap_or("NOT POSSIBLE".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(&[1, 8, 3, 2, 6, 4]), "12:36:48");
        assert_eq!(solution(&[0, 0, 0, 7, 8, 9]), "07:08:09");
        assert_eq!(solution(&[2, 4, 5, 9, 5, 9]), "NOT POSSIBLE");
    }
}
