use std::borrow::Cow;

fn fizzbuzz(n: i32) -> Cow<'static, str> {
    match n {
        _ if n % 15 == 0 => "fizzbuzz".into(),
        _ if n % 3 == 0  => "fizz".into(),
        _ if n % 5 == 0  => "buzz".into(),
        _                => n.to_string().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_test() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(2), "2");
        assert_eq!(fizzbuzz(3), "fizz");
        assert_eq!(fizzbuzz(4), "4");
        assert_eq!(fizzbuzz(5), "buzz");
        assert_eq!(fizzbuzz(6), "fizz");
        assert_eq!(fizzbuzz(30), "fizzbuzz");
    }
}