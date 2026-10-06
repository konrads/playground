/*
InheritedAccess

There are N principals in an organisation, numbered from 0 to N−1. A principal may be a person or a group, and groups may contain other principals, including other groups.

You are given M membership relations, described by two arrays A and B of M integers each. For each J (0 ≤ J < M), principal A[J] is a member of principal B[J].

You are also given an array G listing the principals to whom a resource has been shared directly.

A principal has access to the resource if:

it appears in G, or
it is a member of a principal that has access.

Membership is transitive. If X is a member of Y, and Y is a member of Z, and Z has access, then X has access too.

Write a function:

```rust
fn solution(n: i32, a: &mut Vec<i32>, b: &mut Vec<i32>, g: &mut Vec<i32>) -> i32
```

that, given an integer N and arrays A, B and G as described above, returns the number of principals that have access to the resource.

Examples:

Given N = 5, A = [0, 1, 3], B = [1, 2, 2] and G = [2], the function should return 4. Principal 2 has direct access. Principals 1 and 3 are members of 2. Principal 0 is a member of 1. Principal 4 has no access.
2 -+
   |
   +- 1
   |   |
   |   +- 0
   +- 3
ie.: 0, 1, 2, 3

Given N = 3, A = [0, 1], B = [1, 0] and G = [], the function should return 0. No principal has direct access.
G = [] - no access

Given N = 4, A = [1, 2, 3, 3], B = [0, 1, 2, 2] and G = [0, 0], the function should return 4. Principal 0 has direct access, and principals 1, 2 and 3 inherit it through a chain of memberships.
0 -+
   |
   +- 1
      |
      +- 2
         |
         +- 3
ie. 0, 1, 2, 3

Write an efficient algorithm for the following assumptions:

N is an integer within the range [1..100,000];
M is an integer within the range [0..200,000];
arrays A and B have the same length M;
array G has length within the range [0..N];
each element of arrays A, B and G is an integer within the range [0..N−1];
arrays A and B may contain duplicate pairs and pairs where A[J] = B[J];
membership relations may contain cycles;
array G may contain duplicates.

////////////////////////////////////////////////////////////


*/

use std::collections::{HashMap, HashSet, VecDeque};

fn solution(n: i32, r#as: &mut Vec<i32>, bs: &mut Vec<i32>, gs: &mut Vec<i32>) -> i32 {
    //
    let mut queued = HashSet::new(); // list of nodes already queued, to prevent duplicates
    let mut to_visit = VecDeque::new(); // list of nodes yet to be visited
    for g in gs {
        if queued.insert(*g) {
            to_visit.push_back(*g);
        }
    }

    // important - establish membership via: parent -> children (vec)

    let mut membership: HashMap<i32, HashSet<i32>> = Default::default();
    for (a, b) in r#as.iter().zip(bs) {
        membership.entry(*b).or_default().insert(*a);
    }

    // pop_front & push_back = BFS
    while let Some(g) = to_visit.pop_front() {
        if let Some(r#as) = membership.get(&g) {
            for &a in r#as {
                if queued.insert(a) {
                    to_visit.push_back(a);
                }
            }
        }
    }

    queued.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    // Given N = 5, A = [0, 1, 3], B = [1, 2, 2] and G = [2], the function should return 4. Principal 2 has direct access. Principals 1 and 3 are members of 2. Principal 0 is a member of 1. Principal 4 has no access.
    #[test]
    fn test_1() {
        assert_eq!(
            solution(5, &mut vec![0, 1, 3], &mut vec![1, 2, 2], &mut vec![2]),
            4
        );
    }

    // Given N = 3, A = [0, 1], B = [1, 0] and G = [], the function should return 0. No principal has direct access.
    #[test]
    fn test_2() {
        assert_eq!(
            solution(3, &mut vec![0, 1], &mut vec![1, 0], &mut vec![]),
            0
        );
    }

    // Given N = 4, A = [1, 2, 3, 3], B = [0, 1, 2, 2] and G = [0, 0], the function should return 4. Principal 0 has direct access, and principals 1, 2 and 3 inherit it through a chain of memberships.
    #[test]
    fn test_3() {
        assert_eq!(
            solution(
                4,
                &mut vec![1, 2, 3, 3],
                &mut vec![0, 1, 2, 2],
                &mut vec![0, 0]
            ),
            4
        );
    }

    // test_3's G=[0,0] with no edges. Would fail if duplicate grants were counted twice.
    #[test]
    fn test_3_dup_g_only() {
        assert_eq!(solution(4, &mut vec![], &mut vec![], &mut vec![0, 0]), 1);
    }
}
