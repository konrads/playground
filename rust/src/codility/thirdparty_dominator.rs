/*
Dominator

Source:
https://github.com/jonasraoni/codility/blob/master/Lesson%208%20-%20Leader/Dominator.md

Task

The dominator of an array is a value appearing in strictly more than half of its elements.
Given A, return any index whose value is the dominator. Return -1 if no dominator exists.

Example

For A = [3, 4, 3, 2, 3, -1, 3, 3], value 3 occurs five times out of eight. Any of the
indices 0, 2, 4, 6, or 7 is therefore a valid result.

Input and performance requirements

- N is in 0..=100_000.
- Values span the complete signed 32-bit integer range.
- Expected worst-case time: O(N).
- Expected auxiliary space: O(1), excluding the input.
*/

use std::collections::HashMap;

fn solution(arr: &[i32]) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    let min_freq = arr.len() as i32 / 2 + 1;
    for (i, a) in arr.iter().enumerate() {
        let count = counts.entry(*a).or_default();
        *count += 1;
        if *count >= min_freq {
            return i as i32;
        }
    }

    -1
}

// advanced...
// fn solution(arr: &[i32]) -> i32 {
//     let mut candidate = 0;
//     let mut count = 0;
//     for &a in arr {
//         if count == 0 {
//             candidate = a;
//         }
//         count += if a == candidate { 1 } else { -1 };
//     }

//     let occurrences = arr.iter().filter(|&&a| a == candidate).count();
//     if occurrences * 2 > arr.len() {
//         arr.iter().position(|&a| a == candidate).unwrap() as i32
//     } else {
//         -1
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dominator_found() {
        assert!([0, 2, 4, 6, 7].contains(&solution(&[3, 4, 3, 2, 3, -1, 3, 3])));
        assert_eq!(solution(&[5]), 0);
        assert!([0, 1].contains(&solution(&[1, 1])));
        assert!([1, 2].contains(&solution(&[2, 1, 1])));
        assert!([0, 2].contains(&solution(&[i32::MIN, i32::MAX, i32::MIN])));
    }

    #[test]
    fn no_dominator() {
        assert_eq!(solution(&[]), -1);
        assert_eq!(solution(&[1, 2]), -1);
        assert_eq!(solution(&[1, 1, 2, 2]), -1);
        assert_eq!(solution(&[1, 2, 3]), -1);
    }
}
