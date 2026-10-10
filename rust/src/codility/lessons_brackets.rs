/*
Brackets

Source:
https://github.com/jonasraoni/codility/blob/master/Lesson%207%20-%20Stacks%20and%20Queues/Brackets.md

Task

S is properly nested if it is empty, has the form "(U)", "[U]" or "{U}" where U is properly
nested, or has the form "VW" where V and W are properly nested. Return 1 if S is properly
nested and 0 otherwise.

Example

For S = "{[()()]}", return 1. For S = "([)()]", return 0.

Input and performance requirements

- N is in 0..=200_000.
- S contains only the characters ( ) [ ] { }.
- Expected worst-case time: O(N).
- Expected auxiliary space: O(N), excluding the input.
*/

use std::{iter::Peekable, str::Chars};

fn solution(s: &str) -> u8 {
    let mut feed = s.chars().peekable();
    if let Err(err) = visit_siblings(&mut feed) {
        println!("Error: {err}");
        return 0;
    }
    // should be empty
    if feed.peek().is_some() {
        println!("Error: Feed not empty");
        return 0;
    }
    1
}

// does not consume, just peeks
fn visit_siblings(feed: &mut Peekable<Chars>) -> Result<(), String> {
    // replace following with:
    // while let Some(open_c) = feed.next_if(is_open) {
    while let Some(&open_c) = feed.peek() {
        if !is_open(&open_c) {
            break;
        }
        feed.next();

        // while feed.peek().is_some_and(is_open) {
        // while let Some(c) = feed.peek() {
        //     if !is_open(c) {
        //         break;
        //     }
        //visit_nested(feed)?;

        visit_nested(feed, open_c)?;
    }
    Ok(())
}

// consumes the feed
fn visit_nested(feed: &mut Peekable<Chars>, open_c: char) -> Result<(), String> {
    // let open_c = feed.next().expect("should always be there");
    // expect open_c verified!

    visit_siblings(feed)?;

    if let Some(close_c) = feed.next() {
        if !open_close_matches(&open_c, &close_c) {
            return Err(format!("open/close mismatch: {open_c}/{close_c}"));
        }
    } else {
        return Err("premature feed end".to_string());
    }
    Ok(())
}

// fn visit(feed: &mut Peekable<Chars>) -> Result<(), String> {
//     // accept 1 or more fully enclosed
//     while let Some(&open_c) = feed.peek() {
//         if !is_open(&open_c) {
//             // presume it belongs to the caller, eg. closing bracket
//             return Ok(());
//         }
//         feed.next();

//         let Some(close_c) = feed.peek() else {
//             return Err("no more closing char".to_string());
//         };

//         if !open_close_matches(&open_c, close_c) {
//             visit(feed)?;
//         }

//         // consume of the feed
//         let Some(close_c) = feed.next() else {
//             return Err(format!("no closing char for {open_c}"));
//         };
//         if !open_close_matches(&open_c, &close_c) {
//             return Err(format!(
//                 "unexpected open/close mismatch: {open_c}/{close_c}"
//             ));
//         }
//     }

//     Ok(())
// }

fn is_open(c: &char) -> bool {
    ['(', '[', '{'].contains(c)
}

fn open_close_matches(open_c: &char, close_c: &char) -> bool {
    matches!((open_c, close_c), ('(', ')') | ('[', ']') | ('{', '}'))
}

#[test]
fn test_recursive() {
    assert_eq!(solution("[()"), 0);
    assert_eq!(solution("{[()()]}"), 1);
    assert_eq!(solution("([)()]"), 0);
    assert_eq!(solution(""), 1);
    assert_eq!(solution("()[]{}"), 1);
    assert_eq!(solution(")("), 0);
    assert_eq!(solution("(("), 0);
    assert_eq!(solution("())"), 0);
}
