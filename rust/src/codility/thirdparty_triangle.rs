/*
Triangle

Source:
https://github.com/jonasraoni/codility/blob/master/Lesson%206%20-%20Sorting/Triangle.md

Task

Given an integer array A, determine whether some indices P < Q < R form a triangular
triplet. Their three values must satisfy every triangle inequality:

- A[P] + A[Q] > A[R]
- A[Q] + A[R] > A[P]
- A[R] + A[P] > A[Q]

Return 1 when at least one such triplet exists; otherwise return 0.

Examples

- [10, 2, 5, 1, 8, 20] returns 1; values 10, 5, and 8 form a triangle.
- [10, 50, 5, 1] returns 0.

Input and performance requirements

- N is in 0..=100_000.
- Values span the complete signed 32-bit integer range.
- Expected worst-case time: O(N log N).
- Expected auxiliary space: O(N), excluding the input.
- Addition must be performed without signed overflow.
*/

fn solution(arr: &[i64]) -> u8 {
    for p in 0..arr.len() {
        for q in p + 1..arr.len() {
            for r in q + 1..arr.len() {
                if arr[p] + arr[q] > arr[r] && arr[q] + arr[r] > arr[p] && arr[r] + arr[p] > arr[q]
                {
                    return 1;
                }
            }
        }
    }
    0
}
