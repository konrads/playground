use std::collections::{HashSet, HashMap};
use std::fmt::Debug;

static NO_MATCHING_PARENTHESIS_ERR: &str = "no matching parenthesis";

// Note, cannot extend Copy without Clone, as Copy: Clone, although I dunno why
#[derive(Debug, Copy, Clone)]
struct X(i32);
fn refs_and_slices() {
    let mut a = [X(1), X(2), X(3) ,X(4), X(5)];
    let v = vec![X(1), X(2), X(3), X(4) ,X(5)];
    let a_slice = &a[..];
    let v_slice = &v[..];
    let mut a_mapped= a.iter().map(|&x| X(x.0 * 2)).collect::<Vec<X>>();
    // let (mut a1_mut, mut a2_mut): (Vec<&X>, Vec<&X>) = a.iter().drain().partition(|x| x.0 % 2 == 0);
    // let (a1, a2): (Vec<X>, Vec<X>) = a.iter().partition(|&&mut x| x.0 % 2 == 0);
    let (mut a1, mut a2): (Vec<X>, Vec<X>) = a.iter().partition(|&&x| x.0 % 2 == 0);
    a1[1] = X(111);
    println!("
a        : {:?}
v        : {:?}
a_slice  : {:?}
v_slice  : {:?}
a_mapped : {:?}
a1       : {:?}, a2: {:?}
    ", a, v, a_slice, v_slice, a_mapped, a1, a2);
}


const HH_PARSE_ERR: &str = "Failed to parse HH";
const MM_PARSE_ERR: &str = "Failed to parse MM";
const SS_PARSE_ERR: &str = "Failed to parse SS";
const INVALID_FORMAT_ERR: &str = "Invalid format";
fn digital_clock_time_2_secs(s: &str) -> Result<i32, &str> {
    let split = s.split(":").collect::<Vec<_>>();
    match split[..] {
        [hh, mm, ss] => {
            let hh_parsed = hh.parse::<i32>().map_err(|x| HH_PARSE_ERR)?;
            let mm_parsed = mm.parse::<i32>().map_err(|x| MM_PARSE_ERR)?;
            let ss_parsed = ss.parse::<i32>().map_err(|x| SS_PARSE_ERR)?;
            Ok(hh_parsed * 3600 + mm_parsed * 60 + ss_parsed)
        },
        _ => Err(INVALID_FORMAT_ERR)
    }
}

fn fib(x: i32) -> i32 {
    fn fib_(i1: i32, i2: i32, cnt: i32) -> i32 {
        if cnt > 0 {
            return fib_(i2, i1+i2, cnt-1)
        }
        return i1;
    }
    fib_(0, 1, x)
}

// NOTE: needs itertools!!!
use itertools::Itertools;
fn first_non_repeated_char(s: &str) -> Option<char> {
    let counts = s.chars()
        .map(|x| (x, 1))// map
        .into_group_map()
        .into_iter().map(|(k, v)| (k, v.len()))// reduce...
        .collect::<HashMap<char, usize>>();
    for c in s.chars() {
        if let Some(1) = counts.get(&c) {
            return Some(c);
        }
    }
    return None;
}


use regex::Regex;
fn count_words(s: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::<&str, usize>::new();
    let regex = Regex::new(r"\s").unwrap();
    for w in regex.split(s) {
        *counts.entry(w).or_insert(0) += 1
    }
    counts
}

fn first_non_repeated_char__no_itertools(s: &str) -> Option<char> {
    // from https://stackoverflow.com/questions/41417660/how-does-one-create-a-hashmap-with-a-default-value-in-rust
    let mut counts = HashMap::<char, i32>::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1
    }

    for c in s.chars() {
        if counts[&c] == 1 {
            return Some(c);
        }
    }
    return None;
}

fn has_redundant_brackets(s: &str) -> Result<bool, &str> {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '+' | '-' | '*' | '/' => stack.push(c),
            ')' => {
                if let Some('(') = stack.pop() {
                    return Ok(true)
                } else {
                    loop {
                        match stack.pop() {
                            None => return Err(NO_MATCHING_PARENTHESIS_ERR),
                            Some('(') => break,
                            Some(_) => ()
                        }
                    }
                    // while ! stack.is_empty() {
                    //     if let Some('(') = stack.pop() {
                    //         break;
                    //     }
                    // }
                }
            }
            _ => ()
        }
    }
    Ok(false)
}

fn quick_sort<T: Ord + Copy + Debug>(xs: &mut [T]) {
    if xs.len() > 1 {
        let pivot = xs[0];
        // partition
        let (mut pre, mut pivots_post): (Vec<T>, Vec<T>) = xs.iter().partition(|&&x| x < pivot);
        let (pivots, mut post): (Vec<T>, Vec<T>) = pivots_post.iter().partition(|&&x| x == pivot);

        println!("pivot: {:?}, pre: {:?}, pivots: {:?}, post: {:?}", pivot, pre, pivots, post);
        quick_sort(&mut pre[..]);
        quick_sort(&mut post[..]);

        // copy, from https://users.rust-lang.org/t/idomatic-partial-copy-with-offset/35435/2
        let pivots_offset = pre.len();
        let post_offset = pivots_offset + pivots.len();
        for (dst, src) in xs.iter_mut().zip(pre.iter()) {
            *dst = *src;
        }
        for (dst, src) in xs[pivots_offset..].iter_mut().zip(pivots.iter()) {
            *dst = *src;
        }
        for (dst, src) in xs[post_offset..].iter_mut().zip(post.iter()) {
            *dst = *src;
        }

        // for (i, &x) in pre.iter().enumerate() {
        //     xs[i] = x;
        // }
        // for (i, &x) in pivots.iter().enumerate() {
        //     xs[i+pivots_offset] = x;
        // }
        // for (i, &x) in post.iter().enumerate() {
        //     xs[i+post_offset] = x;
        // }
    }
}

fn is_anagram(s1: &str, s2: &str) -> bool {
    s1.chars().collect::<HashSet<_>>() == s2.chars().collect::<HashSet<_>>()
}

fn find_duplicates(s: &str) -> String {
    let mut uniques = HashSet::new();
    let mut duplicates = HashSet::new();
    for c in s.chars() {
        if uniques.contains(&c) {
            duplicates.insert(c);
        } else {
            uniques.insert(c);
        }
    }
    duplicates.iter().collect::<String>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digital_clock_time_2_secs_test() {
        assert_eq!(digital_clock_time_2_secs("01:10:20"), Ok(3600+600+20));
        assert_eq!(digital_clock_time_2_secs("balls:balls:balls"), Err(HH_PARSE_ERR));
        assert_eq!(digital_clock_time_2_secs("balls"), Err(INVALID_FORMAT_ERR));
    }

    #[test]
    fn word_count_test() {
        // Note: need map macro!
        let exp: HashMap<&str, usize> = [("hello", 1usize), ("you", 2usize), ("monkey", 1usize), ("balls", 3)].iter().map(|&x| x).collect();
        assert_eq!(count_words("hello you monkey you balls balls balls"), exp);
    }

    #[test]
    fn fib_test() {
        for i in 0..10 {
            println!("i: {}, fib: {}", i, fib(i));
        }
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
    }

    #[test]
    fn first_non_repeated_char_test() {
        let f = first_non_repeated_char;
        assert_eq!(f("asd"), Some('a'));
        assert_eq!(f("asda"), Some('s'));
        assert_eq!(f("asdas"), Some('d'));
        assert_eq!(f("asdasd"), None);
        let f = first_non_repeated_char__no_itertools;
        assert_eq!(f("asd"), Some('a'));
        assert_eq!(f("asda"), Some('s'));
        assert_eq!(f("asdas"), Some('d'));
        assert_eq!(f("asdasd"), None);
    }

    #[test]
    fn refs_and_slices_test() {
        refs_and_slices();
    }

    #[test]
    fn has_redundant_brackets_test() {
        assert_eq!(has_redundant_brackets("(( a - b ))"), Ok(true));
        assert_eq!(has_redundant_brackets("(( a * b ) - (c) + d)"), Ok(true));
        assert_eq!(has_redundant_brackets("( a - b )"), Ok(false));
        assert_eq!(has_redundant_brackets("(( a * b ) - c + d)"), Ok(false));
        assert_eq!(has_redundant_brackets("( a - b ))"), Err(NO_MATCHING_PARENTHESIS_ERR));  // brackets don't match even though invalid expr
    }

    #[test]
    fn quick_sort_test() {
        let mut xs = Vec::<i32>::new();
        quick_sort(&mut xs[..]);
        assert_eq!(xs, &vec![][..]);

        let mut xs = vec![1];
        quick_sort(&mut xs[..]);
        assert_eq!(xs, &vec![1][..]);

        let mut xs = vec![1, 2];
        quick_sort(&mut xs[..]);
        assert_eq!(xs, &vec![1, 2][..]);

        let mut xs = vec![2, 1];
        quick_sort(&mut xs[..]);
        assert_eq!(xs, &vec![1, 2][..]);

        let mut xs = vec![4, 10, 9, 7, 5, 6];
        quick_sort(&mut xs[..]);
        assert_eq!(xs, &vec![4, 5, 6, 7, 9, 10][..]);
    }

    #[test]
    fn find_duplicates_test() {
        assert_eq!(find_duplicates("asdaafaaf"), "af");
    }

    #[test]
    fn is_anagram_test() {
        assert!(is_anagram("cinema", "iceman"));
    }
}