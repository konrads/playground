/*
BinaryGap

Source:
https://github.com/jonasraoni/codility/blob/master/Lesson%201%20-%20Iterations/BinaryGap.md

Task

For a positive integer N, inspect its base-two representation. A binary gap is a maximal
run of zero bits with a one bit on both its left and right sides. Return the length of the
longest such run. Return 0 when no bounded zero run exists.

Examples

- 9 is 1001 in binary, so the result is 2.
- 529 is 1000010001, so the result is 4.
- 20 is 10100, so the result is 1; the trailing zeros do not form a gap.
- 15 is 1111, so the result is 0.
- 1041 is 10000010001, so the result is 5.

Input and performance requirements

- N is in 1..=2_147_483_647.
- Expected worst-case time: O(log N).
- Expected auxiliary space: O(1).
*/

// done elsewhere
