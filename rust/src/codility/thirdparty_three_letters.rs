/*
ThreeLetters

Repository source:
https://github.com/RosenbergYehuda/Codility-Exercises-Solutions/blob/main/Coding%20skills/three_letters.py

Task source:
https://app.codility.com/programmers/trainings/5/three_letters/

Task

Given non-negative integers A and B, construct any string containing exactly A copies of
'a' and exactly B copies of 'b'. The result must contain neither "aaa" nor "bbb".
Return None if no such string exists.

Examples

- A = 5 and B = 3 may produce Some("aabaabab") or any other valid arrangement.
- A = 3 and B = 3 may produce Some("ababab"), among several valid answers.
- A = 1 and B = 4 must produce Some("bbabb").
- A = 5 and B = 1 produces None.

Input requirements

- A and B are each in 0..=100.
- Correctness, rather than performance, is the assessment focus.
*/

fn solution(mut a: usize, mut b: usize) -> Option<String> {
    // options: aa | bb | a | b
    let mut process_a = a > b;
    let mut acc = "".to_string();
    while a != 0 || b != 0 {
        match (process_a, a, b) {
            (true, _, _) if a > b && a > 1 => {
                a -= 2;
                acc.push_str("aa");
                process_a = false;
            }
            (true, _, _) if a > 0 => {
                a -= 1;
                acc.push_str("a");
                process_a = false;
            }
            //
            (false, _, _) if b > a && b > 1 => {
                b -= 2;
                acc.push_str("bb");
                process_a = true;
            }
            (false, _, _) if b > 0 => {
                b -= 1;
                acc.push_str("b");
                process_a = true;
            }
            _ => return None,
        }
    }

    (a == 0 && b == 0).then_some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nones() {
        assert_eq!(solution(10, 1), None);
    }

    #[test]
    fn test_oks() {
        assert_eq!(solution(5, 3), Some("aabaabba".to_string()));
    }
}
