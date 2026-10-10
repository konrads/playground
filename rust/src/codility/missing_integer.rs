//! In-memory DynamoDB-style store. Build put(pk, sk, item), query(pk, sk_begins_with / between),
//! and a secondary index on one attribute that stays consistent when items are updated or deleted.
//! This tests BTreeMap<(pk, sk), _> range queries and index maintenance.
//! Stale GSI entries left behind after an update are a common hidden-test failure.

use std::collections::{BTreeMap, HashMap, HashSet};

/*
This is a demo task.

Write a function:

pub fn solution(A: &mut Vec<i32>) -> i32
content_copy

that, given an array A of N integers, returns the smallest positive integer (greater than 0) that does not occur in A.

For example, given A = [1, 3, 6, 4, 1, 2], the function should return 5.

Given A = [1, 2, 3], the function should return 4.

Given A = [−1, −3], the function should return 1.

Write an efficient algorithm for thme following assumptions:

N is an integer within the range [1..100,000];
each element of array A is an integer within the range [−1,000,000..1,000,000].
*/

pub fn solution(xs: &mut Vec<i32>) -> i32 {
    // put in a hashset
    let xs = xs.iter().collect::<HashSet<_>>();
    for x in 1.. {
        if !xs.contains(&x) {
            return x;
        }
    }
    unreachable!()
}

pub fn solution2(xs: &mut Vec<i32>) -> i32 {
    xs.sort();
    // put in a hashset
    for x in 1.. {
        if !xs.contains(&x) {
            return x;
        }
    }
    unreachable!()
}

#[test]
fn test_solution1() {
    let mut xs = vec![1, 3, 6, 4, 1, 2];
    assert_eq!(solution(&mut xs), 5);

    let mut xs = vec![1, 2, 3];
    assert_eq!(solution(&mut xs), 4);

    let mut xs = vec![-1, -3];
    assert_eq!(solution(&mut xs), 1);

    let mut xs = vec![];
    assert_eq!(solution(&mut xs), 1);
}

pub type Item = HashMap<String, String>;

pub enum SortKeyQuery {
    BeginsWith(String),
    Between(String, String),
}

/// Using BTreesMap as it's ordered and has range() method
pub struct Table {
    items: BTreeMap<(String, String), Item>,
    index_pk: String,
    index_sk: String,
    index: BTreeMap<(String, String), (String, String)>,
}

impl Table {
    fn put(&mut self, pk: String, sk: String, item: Item) {
        if let Some(v_pk) = item.get(&self.index_pk)
            && let Some(v_sk) = item.get(&self.index_sk)
        {
            self.index
                .insert((v_pk.clone(), v_sk.clone()), (pk.clone(), sk.clone()));
        }

        self.items.insert((pk, sk), item);
    }

    fn query(&self, pk: String, sk: Option<SortKeyQuery>) -> Vec<&Item> {
        match sk {
            Some(SortKeyQuery::BeginsWith(prefix)) => self
                .items
                .range((pk.clone(), prefix.clone())..)
                .take_while(|((pk2, sk2), _)| &pk == pk2 && sk2.starts_with(&prefix))
                .map(|(_, v)| v)
                .collect(),
            Some(SortKeyQuery::Between(start, end)) => self
                .items
                .range((pk.clone(), start)..=(pk, end))
                .map(|(_, v)| v)
                .collect(),
            None => self
                .items
                .range((pk.clone(), "".to_owned())..)
                .take_while(|((pk2, _), _)| &pk == pk2)
                .map(|(_, v)| v)
                .collect(),
        }
    }

    fn query_index(&self, pk: String, sk: Option<SortKeyQuery>) -> Vec<&Item> {
        match sk {
            Some(SortKeyQuery::BeginsWith(prefix)) => self
                .index
                .range((pk.clone(), prefix.clone())..)
                .take_while(|((pk2, sk2), _)| &pk == pk2 && sk2.starts_with(&prefix))
                .filter_map(|(_, base_pk_sk)| self.items.get(base_pk_sk))
                .collect(),
            Some(SortKeyQuery::Between(start, end)) => self
                .index
                .range((pk.clone(), start)..=(pk, end))
                .filter_map(|(_, base_pk_sk)| self.items.get(base_pk_sk))
                .collect(),
            None => self
                .index
                .range((pk.clone(), "".to_owned())..)
                .take_while(|((pk2, _), _)| &pk == pk2)
                .filter_map(|(_, base_pk_sk)| self.items.get(base_pk_sk))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(n: i64, status: &str) -> Item {
        HashMap::from([
            ("n".to_owned(), n.to_string()),
            ("status".to_owned(), status.to_owned()),
        ])
    }

    fn as_ns(items: Vec<&Item>) -> Vec<&str> {
        items.iter().map(|item| item["n"].as_str()).collect()
    }

    // u0 and u2 are neighbours of u1 and must never leak into u1 queries
    fn fixture() -> Table {
        let mut t = Table {
            items: BTreeMap::new(),
            index_pk: "status".to_owned(),
            index_sk: "n".to_owned(),
            index: BTreeMap::new(),
        };
        t.put("u0".into(), "2024-01-01".into(), item(0, "open"));
        t.put("u1".into(), "2024".into(), item(1, "open"));
        t.put("u1".into(), "2024-01-05".into(), item(2, "closed"));
        t.put("u1".into(), "2024-02-10".into(), item(3, "open"));
        t.put("u1".into(), "2025-01-01".into(), item(4, "closed"));
        t.put("u2".into(), "2024-01-01".into(), item(5, "open"));
        t
    }

    #[test]
    fn test_query_and_index() {
        let t = fixture();

        assert_eq!(as_ns(t.query("u1".into(), None)), ["1", "2", "3", "4"]);
        assert_eq!(as_ns(t.query("u0".into(), None)), ["0"]);
        assert!(t.query("u3".into(), None).is_empty());

        assert_eq!(
            as_ns(t.query_index("open".to_string(), None)),
            ["0", "1", "3", "5"]
        );
        assert_eq!(as_ns(t.query_index("closed".to_string(), None)), ["2", "4"]);
        assert!(t.query_index("missing".to_string(), None).is_empty());

        let begins_with = |pk: &str, prefix: &str| {
            as_ns(t.query_index(pk.into(), Some(SortKeyQuery::BeginsWith(prefix.into()))))
        };
        assert_eq!(begins_with("open", "3"), ["3"]);
        assert_eq!(begins_with("open", ""), ["0", "1", "3", "5"]);
        assert!(begins_with("closed", "3").is_empty());

        let between = |pk: &str, start: &str, end: &str| {
            as_ns(t.query_index(
                pk.into(),
                Some(SortKeyQuery::Between(start.into(), end.into())),
            ))
        };
        assert_eq!(between("open", "1", "3"), ["1", "3"]);
        assert_eq!(between("closed", "2", "4"), ["2", "4"]);
        assert!(between("open", "6", "9").is_empty());
    }

    #[test]
    fn test_begins_with() {
        let t = fixture();
        let begins_with = |prefix: &str| {
            as_ns(t.query("u1".into(), Some(SortKeyQuery::BeginsWith(prefix.into()))))
        };

        // sk equal to the prefix counts as a match
        assert_eq!(begins_with("2024"), ["1", "2", "3"]);
        assert_eq!(begins_with("2024-01"), ["2"]);
        assert_eq!(begins_with("2025"), ["4"]);
        assert_eq!(begins_with(""), ["1", "2", "3", "4"]);
        assert!(begins_with("2023").is_empty());
        assert!(begins_with("2026").is_empty());
    }

    #[test]
    fn test_between() {
        let t = fixture();
        let between = |start: &str, end: &str| {
            as_ns(t.query(
                "u1".into(),
                Some(SortKeyQuery::Between(start.into(), end.into())),
            ))
        };

        // both bounds inclusive
        assert_eq!(between("2024-01-05", "2024-02-10"), ["2", "3"]);
        assert_eq!(between("2024-02-10", "2024-02-10"), ["3"]);
        // bounds need not be existing keys
        assert_eq!(between("2024-01", "2024-12"), ["2", "3"]);
        assert_eq!(between("", "9999"), ["1", "2", "3", "4"]);
        assert!(between("2024-03", "2024-12").is_empty());
    }
}
