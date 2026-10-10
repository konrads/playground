/*
CountFactors

Source:
https://github.com/jonasraoni/codility/blob/master/Lesson%2010%20-%20Prime%20and%20composite%20numbers/CountFactors.md

Task

A positive integer D is a factor of positive integer N when another integer M exists such
that N = D * M. Return the number of distinct positive factors of N.

Example

For N = 24, return 8. Its factors are 1, 2, 3, 4, 6, 8, 12, and 24.

Input and performance requirements

- N is in 1..=2_147_483_647.
- Expected worst-case time: O(sqrt N).
- Expected auxiliary space: O(1).
*/

use std::collections::HashSet;

// fn solution(n: u32) -> HashSet<u32> {
//     let sqrt = n.isqrt();

//     let mut divisors = HashSet::new();

//     for i in 1..=sqrt {
//         if n % i == 0 {
//             divisors.insert(i);
//             divisors.insert(n / i);
//         }
//     }

//     divisors

// }

fn solution(n: u32) -> u32 {
    let sqrt = n.isqrt();

    let mut divisors = 0;

    for i in 1..=sqrt {
        if n % i == 0 {
            if i == n / i {
                divisors += 1;
            } else {
                divisors += 2;
            }
        }
    }

    divisors
}
