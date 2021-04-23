use std::collections::HashMap;

pub trait PriorityQueue<Element> {
    /// create a new priority queue.
    fn new() -> Self;
    /// check whether the queue has no elements.
    fn is_empty(&self) -> bool;
    /// returns the highest-priority element but does not modify the queue.
    fn peek(&self) -> Option<Element>;
    /// add an element to the queue with an associated priority.
    fn insert(&mut self, element: Element, priority: u64);
    /// remove the element from the queue that has the highest priority, and return it.
    fn pop(&mut self) -> Option<Element>;
}

pub struct PriorityQueueImpl(HashMap<Vec<u8>, Vec<u8>>);

// Do not modify anything above ^^^
use std::convert::TryFrom;

/// Limited and inefficient `PriorityQueue` implementation that nonetheless fulfills the requirements.
/// It needs to heep awareness of both priorities and elements, and given that struct `PriorityQueueImpl`
/// only keeps a single data structure, a HashMap, in needs to map both priorities and elements onto it.
/// It does so by using keys to store priorities, values to store elements.
///
/// Restrictions of this implementation are:
/// - priorities must be castable to u8
/// - only single element can be store for given priority
/// - `pop()` and `peek()` are inefficient in lookup of highest priority
impl PriorityQueueImpl {
    /// fetches highest priority key, or None if empty
    fn highest_priority_key(&self) -> Option<Vec<u8>> {
        self.0.keys().map(|x| x[0]).max().map(|x| vec![x])
    }
}

impl PriorityQueue<Vec<u8>> for PriorityQueueImpl {
    // TODO: finish the implementation

    fn new() -> Self {
        Self(HashMap::new())
    }

    /// If underlying data structure is empty, we're empty
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Needs to clone result as interfaces dictates usage of owned value
    fn peek(&self) -> Option<Vec<u8>> {
        self.highest_priority_key().map(|key| self.0[&key].clone())
    }

    /// Assumes priority can be `squeezed` into u8, panics otherwise.
    fn insert(&mut self, element: Vec<u8>, priority: u64) {
        let key = vec![
            u8::try_from(priority).expect(&format!("this implementation cannot handle priority ({}) > 255", priority))
        ];
        self.0.insert(key, element);
    }

    fn pop(&mut self) -> Option<Vec<u8>> {
        self.highest_priority_key().map(|key| self.0.remove(&key).unwrap())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut queue = PriorityQueueImpl::new();
        assert!(queue.is_empty());

        queue.insert(vec![0], 5);
        assert!(!queue.is_empty());
        assert_eq!(queue.peek(), Some(vec![0]));

        queue.insert(vec![1], 10);
        queue.insert(vec![2], 3);
        queue.insert(vec![3], 4);
        queue.insert(vec![4], 6);

        assert_eq!(queue.pop(), Some(vec![1]));
        assert_eq!(queue.pop(), Some(vec![4]));
        assert_eq!(queue.pop(), Some(vec![0]));
        assert_eq!(queue.pop(), Some(vec![3]));
        assert_eq!(queue.pop(), Some(vec![2]));

        assert!(queue.is_empty());
    }

    // TODO: add more tests as appropriate

    #[test]
    fn it_works_on_empty() {
        let mut queue = PriorityQueueImpl::new();
        assert!(queue.is_empty());
        assert_eq!(queue.peek(), None);
        assert_eq!(queue.pop(), None);
    }

    /// Exhibits limitation of the current implementation - cannot add multiple elements of the same priority
    #[ignore]
    #[test]
    fn it_works_on_duplicate_priorities() {
        let mut queue = PriorityQueueImpl::new();
        queue.insert(vec![22], 1);
        assert_eq!(queue.peek(), Some(vec![22]));
        queue.insert(vec![11], 1);
        assert_eq!(queue.peek(), Some(vec![22]));

        assert_eq!(queue.pop(), Some(vec![22]));
        assert_eq!(queue.pop(), Some(vec![11]));
    }
}