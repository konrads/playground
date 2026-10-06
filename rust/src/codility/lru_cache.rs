/*

LRUCache

A cache holds at most capacity entries. Each entry has a key and a value. When a new entry is added to a full cache, the least recently used entry is evicted first. An entry counts as used when it is added, updated, or read by a successful GET.

You are given N operations, described by three arrays of N integers each. Operation I is:

PUT (O[I] = 0): store value V[I] under key K[I]. If the key already exists, replace its value.
GET (O[I] = 1): look up key K[I]. V[I] is ignored.

Write a functions:
```rust```
fn solution(capacity: i32, o: &mut Vec<i32>, k: &mut Vec<i32>, v: &mut Vec<i32>) -> Vec<i32>
```

that returns the result of each GET in order: the value found, or −1 if the key is not in the cache.

Example

Given capacity = 2 and the operations:

Op	K	V
PUT	1	10
PUT	2	20
GET	1	–
PUT	3	30
GET	2	–
PUT	1	11
GET	1	–
GET	3	–

the function should return [10, −1, 11, 30].

Adding key 3 evicts key 2, because key 1 was read more recently.
PUT 1, 11 updates key 1 rather than inserting it, so nothing is evicted.
Assumptions

Write an efficient algorithm for the following assumptions:

N is an integer within the range [1..200,000];
capacity is an integer within the range [1..200,000];
each element of array O is either 0 or 1;
each element of arrays K and V is an integer within the range [0..1,000,000,000].

*/

use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
};

type OpId = u64;
type Key = String;
type Value = String;

#[derive(Default)]
struct LruCacheState {
    cache: HashMap<Key, (OpId, Value)>,
    by_op_id: BTreeMap<OpId, Key>,
    op_count: OpId,
}

struct LruCache {
    state: RefCell<LruCacheState>,
    capacity: usize,
}

impl LruCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            state: Default::default(),
            capacity,
        }
    }

    pub fn get(&self, k: Key) -> Option<Value> {
        let state = &mut *self.state.borrow_mut();

        let (op_id, value) = state.cache.get_mut(&k)?;
        let value = value.clone();
        state.op_count += 1;
        let new_op_id = state.op_count;
        // let old_op_id = std::mem::replace(op_id, new_op_id);
        let old_op_id = *op_id;
        *op_id = new_op_id;

        state.by_op_id.remove(&old_op_id);
        state.by_op_id.insert(new_op_id, k);
        Some(value)
    }

    pub fn put(&self, k: Key, v: Value) -> Option<Value> {
        let mut state = self.state.borrow_mut();

        // new op_id
        state.op_count += 1;
        let new_op_id = state.op_count;

        // compact first
        if state.cache.len() >= self.capacity
            && !state.cache.contains_key(&k)
            && let Some((_, old_key)) = state.by_op_id.pop_first()
        {
            state.cache.remove(&old_key);
        }

        // upsert
        state.by_op_id.insert(new_op_id, k.clone());
        let (old_op_id, old_v) = state.cache.insert(k, (new_op_id, v))?;
        state.by_op_id.remove(&old_op_id);
        Some(old_v)
    }

    pub fn delete(&self, k: &Key) -> Option<Value> {
        let mut state = self.state.borrow_mut();

        let (op_id, value) = state.cache.remove(k)?;
        state.by_op_id.remove(&op_id);
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_get_update_delete_evict() {
        let c = LruCache::new(2);

        // add
        assert_eq!(c.put("a".to_string(), "1".to_string()), None);
        assert_eq!(c.put("b".to_string(), "2".to_string()), None);
        assert_eq!(c.get("a".to_string()), Some("1".to_string()));

        // update returns old value, doesn't evict
        assert_eq!(
            c.put("a".to_string(), "10".to_string()),
            Some("1".to_string())
        );
        assert_eq!(c.get("a".to_string()), Some("10".to_string()));
        assert_eq!(c.get("b".to_string()), Some("2".to_string()));

        // delete
        assert_eq!(c.delete(&"b".to_string()), Some("2".to_string()));
        assert_eq!(c.get("b".to_string()), None);
        assert_eq!(c.delete(&"b".to_string()), None);

        // fill to capacity, then breach: "a" is least recently used
        assert_eq!(c.put("c".to_string(), "3".to_string()), None);
        assert_eq!(c.get("c".to_string()), Some("3".to_string()));
        assert_eq!(c.put("d".to_string(), "4".to_string()), None);
        assert_eq!(c.get("a".to_string()), None);
        assert_eq!(c.get("c".to_string()), Some("3".to_string()));
        assert_eq!(c.get("d".to_string()), Some("4".to_string()));
    }
}
