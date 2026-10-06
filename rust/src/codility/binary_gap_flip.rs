/*

BinaryGapFlip

A binary gap within a positive integer N is a maximal sequence of consecutive zeros in the binary representation of N that is surrounded by ones at both ends.

You may choose at most K of the ones in N's binary representation and change them to zeros. Your goal is to make the longest binary gap as long as possible. The result must still contain a gap bounded by ones on both ends, so you can't flip away every boundary.

Write a function that, given positive integers N and K, returns the length of the longest binary gap achievable after at most K flips. If no binary gap can be formed, the function should return 0.

Examples:

N = 1041 (binary 10000010001), K = 0: the function should return 5. No flips are allowed, and the gaps are 5 and 3.
N = 1041, K = 1: the function should return 9. Flip the middle one to get 10000000001.
N = 305 (binary 100110001), K = 1: the function should return 4. Flip the lower one of the 11 pair to get 100100001.
N = 305, K = 2: the function should return 7. Flip both middle ones to get 100000001.
N = 32 (binary 100000), K = 5: the function should return 0. There is only a single one, so no gap can be formed.
N = 15 (binary 1111), K = 2: the function should return 2. Flip the two inner ones to get 1001.

Write an efficient algorithm for the following assumptions:

N is an integer within the range [1..2,147,483,647];
K is an integer within the range [0..31].

Hidden-test traps:

Trailing zeros, as in N = 32 or N = 328 (101001000). They aren't bounded on the right, so they never count.
Runs of adjacent ones. Each one flipped costs 1, and a flip may eat into a boundary run as long as a one remains on each side.
A K larger than the number of ones available to flip.
N = 1, and N = 2,147,483,647 (all ones).

*/

/*

My interpretation:

100110001
 |    |
 2    3

for K = 1:
100010001 - gaps are 3 & 3
                     _   _

100100001 - gaps are 2 & 4
                         _

for K = 2:
100000001 - gap is 7
                   _

so for   100110001:
indexes: 012345678
- convert number to binary
- list 0 start-end indexes: 1..3, 6..8
- with lens                  2     3
- looping K, K -= 1; K<=0:
  - can I reduce, ie. is there more than 1 gap? if so, expand the greatest: 1..3, 4..8
                                                                              2    4
  - can I reduce, ie. is there more than 1 gap? if so, expand the greatest: 1..8
                                                                              7

what if 10010000110001
        01234567890123
     1..3, 4..8, 10..13
       2    4      3
Q: need to expand on 4..8 - but which way?

for K=1, flip ind[3],   gap = 1..8 = 7
for K=2, flip ind[8-9], gap = 3..13 = 10



100110001



*/

use std::collections::HashSet;

fn solution(n: i32, k: i32) -> i32 {
    let bytes = format!("{n:b}")
        .bytes()
        .map(|b| b - b'0')
        .rev()
        .collect::<Vec<_>>();

    let ones_inds = bytes
        .iter()
        .enumerate()
        .filter_map(|(i, x)| (*x == 1).then_some(i))
        .collect::<Vec<_>>();

    println!("### ones_inds {ones_inds:?}, k {k}");

    let mut max_gap = 0;
    for (start_1_ind_ind, start_1_ind) in ones_inds.iter().enumerate() {
        let end_1_ind_ind = (start_1_ind_ind + k as usize + 1).min(ones_inds.len() - 1);
        println!(
            "##### ones_inds {ones_inds:?}, start_1_ind_ind: {start_1_ind_ind}, end_1_ind_ind: {end_1_ind_ind}"
        );
        if end_1_ind_ind > start_1_ind_ind {
            let end_1_ind = ones_inds[end_1_ind_ind];
            let gap = end_1_ind - start_1_ind - 1;
            max_gap = max_gap.max(gap);
        }
    }

    max_gap as i32

    // let how_many_1s = bytes.iter().filter(|b| **b == 1).count();
    // println!("### how_many_1s {how_many_1s}, k {k}");

    // if how_many_1s <= 1 {
    //     // no gaps present
    //     0
    // } else if how_many_1s - 2 <= k as usize {
    //     // can simply eliminate all gaps and return end-to-end gap
    //     bytes.len() as i32 - 2
    // } else {
    //     // find all 0 seqs with k 1s
    //     // find all start indices
    //     let mut gap_starts = vec![];
    //     let mut prev_wasnt_0 = true;

    //     for (i, b) in bytes.iter().enumerate() {
    //         if prev_wasnt_0 && *b == 0 {
    //             gap_starts.push(i);
    //         }
    //         prev_wasnt_0 = *b != 0;
    //     }

    //     println!("#### gap_starts {gap_starts:?}");
    //     let mut gap_lens = HashSet::new();
    //     for gap_start in gap_starts {
    //         let gap_candidate = &bytes[gap_start..bytes.len() - 1];
    //         println!("#### gap_candidate {gap_candidate:?}");
    //         let mut cnt_1s = k;
    //         let mut gap_len = 0;
    //         for b in gap_candidate {
    //             if *b == 1 {
    //                 if cnt_1s == 0 {
    //                     break;
    //                 }
    //                 cnt_1s -= 1;
    //             }
    //             gap_len += 1;
    //         }
    //         gap_lens.insert(gap_len);
    //         println!("##### gap_lens {gap_lens:?}")
    //     }

    //     let ones: Vec<i32> = (0..31).filter(|i| n >> i & 1 == 1).collect();
    //     let xxx = (0..ones.len())
    //         .map(|i| {
    //             let j = (i + k as usize + 1).min(ones.len() - 1);
    //             (ones[j] - ones[i] - 1).max(0)
    //         })
    //         .max()
    //         .unwrap_or(0);

    //     gap_lens.into_iter().max().unwrap_or(0)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10000010001() {
        let n = i32::from_str_radix("10000010001", 2).unwrap();
        assert_eq!(solution(n, 0), 5);
        assert_eq!(solution(n, 1), 9);
        assert_eq!(solution(n, 2), 9);
    }

    #[test]
    fn test_100110001() {
        let n = i32::from_str_radix("100110001", 2).unwrap();
        assert_eq!(solution(n, 0), 3);
        assert_eq!(solution(n, 1), 4);
        assert_eq!(solution(n, 2), 7);
        assert_eq!(solution(n, 3), 7);
    }
}
