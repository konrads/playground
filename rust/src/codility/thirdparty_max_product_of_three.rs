/*
MaxProductOfThree

Source:
https://github.com/jonasraoni/codility/blob/master/Lesson%206%20-%20Sorting/MaxProductOfThree.md

Task

Given an array A with N integers, choose three distinct indices P, Q, and R satisfying
P < Q < R. Return the greatest possible value of A[P] * A[Q] * A[R]. Negative values
matter because two negative factors can produce a positive product.

Example

For A = [-3, 1, 2, -2, 5, 6], the maximum product is 60, obtained from 2 * 5 * 6.

Input and performance requirements

- N is in 3..=100_000.
- Each value is in -1_000..=1_000.
- Expected worst-case time: O(N log N).
- Expected auxiliary space: O(1), excluding the input.
*/

fn solution(arr: &[i32]) -> i32 {
    // brute force: 3 nested loops, each picks a number, find max
    // let mut result = i32::MIN;
    // for x in 0..arr.len() {
    //     for y in x + 1..arr.len() {
    //         for z in y + 1..arr.len() {
    //             result = result.max(arr[x] * arr[y] * arr[z]);
    //         }
    //     }
    // }

    // result

    // not-brute:
    // - 3 +ve - mult 3 largest
    // - 2 +ve, 1 -ve - mult 2 smallest, 1 largest
    // - 1 +ve, 2 -ve - mult 3 largest
    // - 3 -ve - mult 3 largest
    // - 0s?
    let mut arr = arr.to_vec();
    arr.sort_unstable();
    let n = arr.len();
    let result = (arr[n - 1] * arr[n - 2] * arr[n - 3]).max(arr[0] * arr[1] * arr[n - 1]);
    result
}
