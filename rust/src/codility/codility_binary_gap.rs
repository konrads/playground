/*
BinaryGap: given a positive i32 N (1..=2_147_483_647), return the length of the longest
run of zeros surrounded by ones on both sides in the binary form of N, or 0 if there is none.
Examples: 9 (1001) -> 2, 529 (1000010001) -> 4, 20 (10100) -> 1, 15 (1111) -> 0,
32 (100000) -> 0, 1041 (10000010001) -> 5.
*/

pub fn solution_v0(n: i32) -> i32 {
    let bits = to_bits(n);
    let mut result = 0;

    let one_indices = (0..bits.len())
        .filter(|i| bits[*i] == 1)
        .collect::<Vec<_>>();

    let one_index_gaps = one_indices.windows(2);

    let result = one_index_gaps
        .map(|window| window[1] - window[0] - 1)
        .max()
        .unwrap_or_default() as i32;

    result
}

// least significant bit first
fn to_bits(mut n: i32) -> Vec<u8> {
    let mut bits = Vec::new();
    while n > 0 {
        bits.push((n & 1) as u8);
        n >>= 1;
    }
    bits
}

fn all_zeros(bits: &[u8]) -> bool {
    bits.iter().all(|&b| b == 0)
}

/*
Optimisation plan:

v1: the inner `j` loop tries every j and rechecks the zeros with `all_zeros`. Start j only
at a one and stop it at the first one after i, so each gap is visited once and `all_zeros` goes.

v2: drop `to_bits` and the pair loops: walk the bits of n once, from the lowest one upward,
counting zeros since the last one and keeping the max when the next one is reached.

v3: skip the zero runs with `trailing_zeros` on n instead of shifting one bit at a time.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v0_examples() {
        assert_eq!(solution_v0(9), 2);
        assert_eq!(solution_v0(529), 4);
        assert_eq!(solution_v0(20), 1);
        assert_eq!(solution_v0(15), 0);
        assert_eq!(solution_v0(32), 0);
        assert_eq!(solution_v0(1041), 5);
    }
}
