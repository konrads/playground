/*
Nesting

Source:
https://github.com/jonasraoni/codility/blob/master/Lesson%207%20-%20Stacks%20and%20Queues/Nesting.md

Task

S is properly nested if it is empty, has the form "(U)" where U is properly nested, or has
the form "VW" where V and W are properly nested. Return 1 if S is properly nested and 0
otherwise.

Example

For S = "(()(())())", return 1. For S = "())", return 0.

Input and performance requirements

- N is in 0..=1_000_000.
- S contains only ( and ).
- Expected worst-case time: O(N).
- Expected auxiliary space: O(1), excluding the input.
*/

use std::{iter::Peekable, str::Chars};

fn solution(s: &str) -> u8 {
    let mut feed = s.chars().peekable();
    if visit_siblings(&mut feed).is_err() {
        return 0;
    }
    if feed.next().is_some() {
        return 0;
    }
    1
}

// does not consume!
fn visit_siblings(feed: &mut Peekable<Chars>) -> Result<(), String> {
    while let Some(&c) = feed.peek() {
        if c != '(' {
            break;
        }
        visit_nested(feed)?;
    }
    Ok(())
}

// does consume!
fn visit_nested(feed: &mut Peekable<Chars>) -> Result<(), String> {
    let open_c = feed.next().expect("no open");
    if open_c != '(' {
        return Err(format!("unexpcted open_c {open_c}"));
    }
    visit_siblings(feed)?;
    if let Some(close_c) = feed.next() {
        if close_c != ')' {
            return Err(format!("wrong close_c {close_c}"));
        }
    } else {
        return Err("unexpected end of feed".to_string());
    }

    Ok(())
}
