/*
OddOccurrencesInArray

Source:
https://github.com/jonasraoni/codility/blob/master/Lesson%202%20-%20Arrays/OddOccurrencesInArray.md

Task

A has an odd number of elements. Every element can be paired with another element of the
same value, except one. Return the value of the unpaired element.

Example

For A = [9, 3, 9, 3, 9, 7, 9], return 7.

Input and performance requirements

- N is odd and in 1..=1_000_000.
- Each value is in 1..=1_000_000_000.
- Expected worst-case time: O(N).
- Expected auxiliary space: O(1), excluding the input.
*/

use std::collections::HashMap;

fn solution(arr: &[i32]) -> i32 {
    // find the number with odd frequency
    let mut map: HashMap<i32, usize> = HashMap::new();
    for a in arr {
        *map.entry(*a).or_default() += 1;
    }
    map.iter()
        .find_map(|(&a, &size)| (size % 2 == 1).then_some(a))
        .unwrap_or_default()
}
