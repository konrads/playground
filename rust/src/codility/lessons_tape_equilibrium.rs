/*
TapeEquilibrium

Source:
https://github.com/jonasraoni/codility/blob/master/Lesson%203%20-%20Time%20Complexity/TapeEquilibrium.md

Task

Any P with 0 < P < N splits A into two non-empty parts, A[0..P] and A[P..N]. The difference
of a split is the absolute difference between the sums of the two parts. Return the minimal
difference over all splits.

Example

For A = [3, 1, 2, 4, 3], the splits give 7, 5, 1 and 7, so return 1.

Input and performance requirements

- N is in 2..=100_000.
- Each value is in -1_000..=1_000.
- Expected worst-case time: O(N).
- Expected auxiliary space: O(N), excluding the input.
*/

fn solution(arr: &[i32]) -> i32 {
    (1..arr.len())
        .map(|x| {
            let (left, right) = arr.split_at(x);
            // println!("### x {x}, left {left:?}, right {right:?}");
            (left.iter().sum::<i32>() - right.iter().sum::<i32>()).abs()
        })
        .min()
        .unwrap_or_default()
}

#[test]
fn test_edge_splits() {
    assert_eq!(solution(&[1, -1]), 2);
    assert_eq!(solution(&[1, 1, 1, 3]), 0);
    assert_eq!(solution(&[-1000, 1000]), 2000);
}
