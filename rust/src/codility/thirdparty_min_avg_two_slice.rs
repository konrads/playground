/*
MinAvgTwoSlice

Source:
https://github.com/jonasraoni/codility/blob/master/Lesson%205%20-%20Prefix%20Sums/MinAvgTwoSlice.md

Task

A slice is an inclusive range (P, Q) with 0 <= P < Q < N, so it always contains at
least two elements. Its average is the sum from A[P] through A[Q], divided by its length.
Return the starting and ending indices (P, Q) of a slice having the smallest average. If
several slices share that average, return the smallest P, then the smallest Q.

Example

For A = [4, 2, 2, 5, 1, 5, 8], the result is (1, 2). The slice from index 1 through 2 has
average 2, which is minimal; the longer slice from 1 through 4 has average 2.5.

Input and performance requirements

- N is in 2..=100_000.
- Each value is in -10_000..=10_000.
- Expected worst-case time: O(N).
- Expected auxiliary space: O(N), excluding the input.
*/

fn solution(arr: &[i32]) -> (usize, usize) {
    // brute force
    let mut best_avg = f64::MAX;
    let mut best_p: usize = 0;
    let mut best_q: usize = 0;
    for p in 0..arr.len() - 1 {
        let mut sum = arr[p];
        for q in p + 1..arr.len() {
            sum += arr[q];
            // let candidate_avg = arr[p..=q].iter().sum::<i32>() as f64 / (q - p + 1) as f64;
            let candidate_avg = sum as f64 / (q - p + 1) as f64;
            if candidate_avg < best_avg {
                best_avg = candidate_avg;
                best_p = p;
                best_q = q;
            }
        }
    }

    (best_p, best_q)
}
