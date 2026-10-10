/*
You are given an array A of N positive integers. Count the ordered pairs
(i, j), i != j, such that the first digit of A[i] equals the last digit of A[j].

Examples:
  A = [123, 34]      -> 1   (first of 34 = 3 = last of 123)
  A = [123, 31]      -> 2   (1 = last of 31, 3 = last of 123)
  A = [123, 31, 34]  -> 3
  A = []             -> 0

Assume:
  N is an integer within the range [0..100,000];
  each element of A is an integer within the range [1..1,000,000,000].
*/

fn as_first_last(mut n: i32) -> (i32, i32) {
    let last = n % 10;

    let mut first = last;
    while n > 0 {
        first = n % 10;
        n /= 10;
    }
    (first, last)
}

fn find_first_last_pair_counts(n: &[i32]) -> i32 {
    let n_first_last = n.iter().map(|x| as_first_last(*x)).collect::<Vec<_>>();

    let mut result = 0;

    for i in 0..n_first_last.len() {
        let (i_first, i_last) = &n_first_last[i];
        for j in i + 1..n_first_last.len() {
            let (j_first, j_last) = &n_first_last[j];

            if i_first == j_last {
                result += 1
            }

            if j_first == i_last {
                result += 1
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_first_last() {
        assert_eq!(as_first_last(1), (1, 1));
        assert_eq!(as_first_last(12), (1, 2));
        assert_eq!(as_first_last(123), (1, 3));
    }

    #[test]
    fn test_find_first_last_pair_counts() {
        assert_eq!(find_first_last_pair_counts(&vec![]), 0);
        assert_eq!(find_first_last_pair_counts(&vec![123, 34]), 1);
        assert_eq!(find_first_last_pair_counts(&vec![123, 31]), 2);
        assert_eq!(find_first_last_pair_counts(&vec![123, 31, 34]), 3);
    }
}
