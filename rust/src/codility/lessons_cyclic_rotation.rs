/*
CyclicRotation

Source:
https://github.com/jonasraoni/codility/blob/master/Lesson%202%20-%20Arrays/CyclicRotation.md

Task

Rotating an array moves every element one index to the right, and the last element
becomes the first. Given A and K, return A rotated K times.

Example

For A = [3, 8, 9, 7, 6] and K = 3, return [9, 7, 6, 3, 8].
For A = [0, 0, 0] and K = 1, return [0, 0, 0].
For A = [1, 2, 3, 4] and K = 4, return [1, 2, 3, 4].

Input and performance requirements

- N and K are in 0..=100.
- Each value is in -1_000..=1_000.
- Correctness is assessed, not performance.
*/

fn solution(arr: &[i32], k: usize) -> Vec<i32> {
    let mut result = vec![0; arr.len()];
    for (i, x) in arr.iter().enumerate() {
        let new_i = (i + k) % arr.len();
        result[new_i] = *x;
    }
    result
}
