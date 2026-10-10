/*
A gateway receives N requests. Request i arrives at time T[i] from tenant C[i].
Each tenant may have at most K accepted requests in any window of length W.
A request at time t is accepted if the tenant has fewer than K accepted requests with time in (t − W, t]
Otherwise it is rejected.
Rejected requests do not count toward the limit.

Return the number of rejected requests.
*/

use std::collections::{HashMap, VecDeque};

/// Examples
///
/// T=[1,2,3,11,12], C=[5,5,5,5,5], W=10, K=2 → 1 (only the request at t=3 is rejected)
/// T=[0,0,0,5], C=[1,2,1,1], W=5, K=1 → 1
///
/// Assumptions
///
/// N in [0..100,000]
/// T is sorted non-decreasing; each T[i] in [0..1,000,000,000]
/// C[i] in [0..1,000,000,000]
/// W, K in [1..1,000,000,000]
fn solution(t: &mut Vec<i32>, c: &mut Vec<i32>, w: i32, k: i32) -> i32 {
    // iterate through z.zip(c), with a window per c
    let mut windows = HashMap::<i32, VecDeque<i32>>::new();
    let mut rejected = 0;
    for (t, c) in t.iter().zip(c) {
        let window = windows.entry(*c).or_default();

        // BIT SLOWER
        // let i = window.partition_point(|window_t| t - window_t >= w);
        // window.drain(..i);

        // while window.front().is_some_and(|front| t - front >= w) {
        //     window.pop_front();
        // }

        // while w > window.front().map(|front| t - front).unwrap_or_default() {
        //     window.pop_front();
        // }

        // trim the window if needed
        loop {
            let time_since_first_w = window
                .front()
                .map(|w_front| t - w_front)
                .unwrap_or_default();
            let satisfies_w = w > time_since_first_w;
            if satisfies_w {
                break;
            }
            window.pop_front();
        }

        let satisfies_k = k > window.len() as i32;
        if satisfies_k {
            window.push_back(*t);
        } else {
            rejected += 1;
        }
    }

    rejected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_client() {
        let mut t = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut c = [1; 10].to_vec();

        assert_eq!(solution(&mut t, &mut c, 10, 5), 5);
        assert_eq!(solution(&mut t, &mut c, 10, 7), 3);

        // shows a gap
        let mut t = vec![1, 2, 3, 4, 5, 6, 10, 11, 12, 13];
        assert_eq!(solution(&mut t, &mut c, 10, 5), 1);
    }
}
