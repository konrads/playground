/*
PermMissingElem

Source:
https://github.com/jonasraoni/codility/blob/master/Lesson%203%20-%20Time%20Complexity/PermMissingElem.md

Task

A holds N distinct integers from 1..=N + 1, so exactly one value in that range is missing.
Return the missing value.

Example

For A = [2, 3, 1, 5], return 4.

Input and performance requirements

- N is in 0..=100_000.
- Values are distinct and in 1..=N + 1.
- Expected worst-case time: O(N).
- Expected auxiliary space: O(1), excluding the input.
*/

use std::collections::HashSet;

fn solution(arr: &[i32]) -> i32 {
    let mut unique = (1_i32..=arr.len() as i32 + 1).collect::<HashSet<_>>();
    for a in arr {
        unique.remove(a);
    }
    unique.into_iter().last().unwrap()
}
