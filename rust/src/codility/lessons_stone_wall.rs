/*
StoneWall

Source:
https://github.com/jonasraoni/codility/blob/master/Lesson%207%20-%20Stacks%20and%20Queues/StoneWall.md

Task

A straight wall N meters long has height H[I] between I and I + 1 meters from its left end.
It is built from rectangular stone blocks. Return the minimal number of blocks needed.

Example

For H = [8, 8, 5, 7, 9, 8, 7, 4, 8], return 7.
   inds: 0  1  2  3  4  5  6  7  8

                     █
      8  █  █        █  █        █
      7  █  █     █  █  █  █     █
      6  █  █     █  █  █  █     █
      5  █  █  █  █  █  █  █     █
      4  █  █  █  █  █  █  █  █  █
      3  █  █  █  █  █  █  █  █  █
      2  █  █  █  █  █  █  █  █  █
      1  █  █  █  █  █  █  █  █  █

Input and performance requirements

- N is in 1..=100_000.
- Each height is in 1..=1_000_000_000.
- Expected worst-case time: O(N).
- Expected auxiliary space: O(N), excluding the input.
*/

fn solution(h: &[u32]) -> usize {
    let Some(&min) = h.iter().min() else {
        return 0;
    };
    // one block up to min, then the parts between columns of height min
    1 + h.split(|&x| x == min).map(solution).sum::<usize>()
}

#[test]
fn test_simple() {
    assert_eq!(solution(&[8, 8, 5, 7, 9, 8, 7, 4, 8]), 7);
    assert_eq!(solution(&[2, 3, 2]), 2);
    assert_eq!(solution(&[1]), 1);
    assert_eq!(solution(&[1, 2, 3]), 3);
}
