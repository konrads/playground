/*
Fish

Source:
https://github.com/jonasraoni/codility/blob/master/Lesson%207%20-%20Stacks%20and%20Queues/Fish.md

Task

N fish swim in a river, ordered from upstream (index 0) to downstream. A[P] is the size of
fish P, and all sizes are distinct. B[P] is its direction: 0 means upstream, 1 means
downstream. All fish swim at the same speed. Fish P and Q meet when P < Q, B[P] = 1,
B[Q] = 0, and no living fish is between them; the larger one eats the smaller and keeps
swimming. Return the number of fish that stay alive.

Example

For A = [4, 3, 2, 1, 5] and B = [0, 1, 0, 0, 0], return 2. Fish 1 eats fish 2 and 3, then
fish 4 eats it. Fish 0 and 4 never meet anyone.

Input and performance requirements

- N is in 1..=100_000.
- Each A value is in 0..=1_000_000_000, all distinct.
- Each B value is 0 or 1.
- Expected worst-case time: O(N).
- Expected auxiliary space: O(N), excluding the input.
*/

fn solution(a: Vec<u32>, b: Vec<u32>) -> usize {
    // use stack
    let fish_feed = a.into_iter().zip(b);
    let mut stack = vec![];
    let mut survived = 0;

    for (size, dir) in fish_feed {
        if dir == 1 {
            // downstream
            stack.push((size, dir))
        } else {
            // upstream
            while let Some((down_size, _)) = stack.last()
                && *down_size < size
            {
                stack.pop();
            }
            if stack.is_empty() {
                survived += 1;
            }
        }
    }

    survived + stack.len()
}

#[test]
fn test_simple() {
    assert_eq!(solution(vec![4, 3, 2, 1, 5], vec![0, 1, 0, 0, 0]), 2);
    assert_eq!(solution(vec![1, 3, 4, 2, 0, 5], vec![1, 0, 0, 0, 0, 0]), 5);
}
