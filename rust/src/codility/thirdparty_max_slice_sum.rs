/*
MaxSliceSum

Source:
https://github.com/jonasraoni/codility/blob/master/Lesson%209%20-%20Maximum%20slice%20problem/MaxSliceSum.md

Task

A slice is a non-empty inclusive range (P, Q) satisfying 0 <= P <= Q < N. Its sum is the
total of A[P] through A[Q]. Return the greatest sum of any slice in A.

Example

For A = [3, 2, -6, 4, 0], return 5. The slice from index 0 through 1 sums to 5, which is
greater than every other slice sum.

Input and performance requirements

- N is in 1..=1_000_000.
- Each value is in -1_000_000..=1_000_000.
- The result fits in a signed 32-bit integer.
- Expected worst-case time: O(N).
- Expected auxiliary space: O(N), excluding the input.
*/

use std::collections::btree_set;

fn solution(a: &[i32]) -> i32 {
    // brute force, n2:
    // let mut result = i32::MIN;
    // for p in 0..a.len() {
    //     let mut sum = 0;
    //     for q in p..a.len() {
    //         sum += a[q];
    //         result = result.max(sum);
    //     }
    // }

    // result

    let mut current_sum = a[0];
    let mut result = i32::MIN;
    for &x in a {
        current_sum = x.max(current_sum + x);
        result = result.max(current_sum);
    }
    result
}
