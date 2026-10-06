#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    #[test]
    fn test_concat_str() {
        fn double(x: &str) -> String {
            let mut res = "".to_owned();
            res.push_str(&x);
            res.push_str(&x);
            res
        }
        assert_eq!(double("abc"), "abcabc".to_owned());
    }

    #[test]
    fn test_ref_usage() {
        #[derive(Eq, PartialEq, Ord, PartialOrd)]
        struct X<'a> {
            s: &'a str
        }
        #[derive(Eq, PartialEq, Ord, PartialOrd)]
        struct Y<'a> {
            ss: &'a str
        }

        let s = "abc";
        let x1 = X { s: s };
        let x2 = X { s: s };

        let mut map = BTreeMap::new();
        map.insert(Y { ss: "aaa"}, x1);
        map.insert(Y { ss: "bbb"}, x2);
    }
}