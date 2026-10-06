//! key-value store that supports creating and retrieving snapshots, as well as looking up a key’s value in a specific snapshot

use std::{collections::HashMap, hash::Hash};

pub enum Value<V> {
    New { value: V },
    Tombstone,
}

#[derive(Default)]
pub struct KVSnapshot<K, V> {
    inner: HashMap<K, Vec<(u64, Value<V>)>>,
    snapshot_cnt: u64,
}

impl<K: Eq + Hash, V> KVSnapshot<K, V> {
    pub fn upsert(&mut self, k: K, v: V) {
        self.upsert_inner(k, Value::New { value: v });
    }

    pub fn remove(&mut self, k: K) {
        self.upsert_inner(k, Value::Tombstone);
    }

    pub fn get(&self, k: &K, snapshot: Option<u64>) -> Option<&V> {
        let snapshot = snapshot.unwrap_or(self.snapshot_cnt);
        let vs = self.inner.get(k)?;
        let ind = vs.partition_point(|(version, _v)| *version <= snapshot);
        if let Some((_, Value::New { value })) = vs.get(ind.checked_sub(1)?) {
            return Some(value);
        }
        None
    }

    pub fn snapshot(&mut self) {
        self.snapshot_cnt += 1;
    }

    fn upsert_inner(&mut self, k: K, v: Value<V>) {
        let vs = self.inner.entry(k).or_default();
        match vs.last_mut() {
            Some((version, value)) if *version == self.snapshot_cnt => {
                *value = v;
            }
            _ => {
                vs.push((self.snapshot_cnt, v));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kv() -> KVSnapshot<&'static str, i32> {
        KVSnapshot {
            inner: HashMap::new(),
            snapshot_cnt: 0,
        }
    }

    #[test]
    fn get_missing_key() {
        let s = kv();
        assert_eq!(s.get(&"a", None), None);
    }

    #[test]
    fn insert_then_get() {
        let mut s = kv();
        s.upsert("a", 1);
        assert_eq!(s.get(&"a", None), Some(&1));
    }

    #[test]
    fn update_overwrites() {
        let mut s = kv();
        s.upsert("a", 1);
        s.upsert("a", 2);
        assert_eq!(s.get(&"a", None), Some(&2));
    }

    #[test]
    fn remove_then_get() {
        let mut s = kv();
        s.upsert("a", 1);
        s.remove("a");
        assert_eq!(s.get(&"a", None), None);
    }

    #[test]
    fn remove_then_reinsert() {
        let mut s = kv();
        s.upsert("a", 1);
        s.remove("a");
        s.upsert("a", 3);
        assert_eq!(s.get(&"a", None), Some(&3));
    }

    #[test]
    fn keys_are_independent() {
        let mut s = kv();
        s.upsert("a", 1);
        s.upsert("b", 2);
        s.remove("a");
        assert_eq!(s.get(&"a", None), None);
        assert_eq!(s.get(&"b", None), Some(&2));
    }

    #[test]
    fn snapshot_keeps_old_value() {
        let mut s = kv();
        s.upsert("a", 1);
        s.snapshot();
        s.upsert("a", 2);
        assert_eq!(s.get(&"a", Some(0)), Some(&1));
        assert_eq!(s.get(&"a", Some(1)), Some(&2));
        assert_eq!(s.get(&"a", None), Some(&2));
    }

    #[test]
    fn snapshot_before_key_existed() {
        let mut s = kv();
        s.snapshot();
        s.upsert("a", 1);
        assert_eq!(s.get(&"a", Some(0)), None);
        assert_eq!(s.get(&"a", Some(1)), Some(&1));
    }

    #[test]
    fn snapshot_without_changes_sees_earlier_value() {
        let mut s = kv();
        s.upsert("a", 1);
        s.snapshot();
        s.snapshot();
        assert_eq!(s.get(&"a", Some(2)), Some(&1));
    }

    #[test]
    fn remove_after_snapshot_keeps_old_value() {
        let mut s = kv();
        s.upsert("a", 1);
        s.snapshot();
        s.remove("a");
        assert_eq!(s.get(&"a", Some(0)), Some(&1));
        assert_eq!(s.get(&"a", Some(1)), None);
    }

    #[test]
    fn multiple_writes_in_one_snapshot_keep_last() {
        let mut s = kv();
        s.upsert("a", 1);
        s.upsert("a", 2);
        s.snapshot();
        s.upsert("a", 3);
        assert_eq!(s.get(&"a", Some(0)), Some(&2));
    }
}
