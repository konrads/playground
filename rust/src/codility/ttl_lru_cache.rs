//! TTL + LRU cache with tenant fairness. Build get and put with capacity, per-entry TTL, and a cap per tenant so no tenant can evict everyone else's entries.
//! This tests O(1) LRU (hashmap plus linked list, or an indexed slab in Rust to avoid Rc<RefCell> pain) and lazy versus eager expiry. It maps to "caching strategies as tenant count grows."
