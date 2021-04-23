use std::collections::BTreeMap;

pub trait PriorityQueue<Priority, Element> {
    /// create a new priority queue.
    fn new() -> Self;
    /// check whether the queue has no elements.
    fn is_empty(&self) -> bool;
    /// returns the highest-priority element but does not modify the queue.
    fn peek(&self) -> Option<&Element>;
    /// add an element to the queue with an associated priority.
    fn insert(&mut self, element: Element, priority: Priority);
    /// remove the element from the queue that has the highest priority, and return it.
    fn pop(&mut self) -> Option<Element>;
}


pub struct PriorityQueueImpl<P, E>(BTreeMap<P, Vec<E>>);

/// Implementation utilizing `BTreeMap` instead of the `HashMap`. This `BTreeMap` has the advantage of
/// recording the order of keys (priorities) and keeping multiple generic elements.
impl<P: Copy + Ord, E> PriorityQueue<P, E> for PriorityQueueImpl<P, E> {
    fn new() -> Self {
        Self(BTreeMap::new())
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Note: returning an Option of a ref!
    fn peek(&self) -> Option<&E> {
        self.0.iter().next_back().and_then(|(_, v)| v.last().map(|x| x))
    }

    fn insert(&mut self, element: E, priority: P) {
        self.0.entry(priority).or_insert(Vec::new()).insert(0, element);
    }

    fn pop(&mut self) -> Option<E> {
        let k_vlen_opt = self.0.iter_mut().next_back().map(|(k, v)| (*k, v.len()));
        k_vlen_opt.and_then(|(k, vlen)| {
            if vlen == 1 {
                self.0.remove(&k).as_mut().and_then(|x| x.pop())
            } else {
                self.0.get_mut(&k).and_then(|x| x.pop())
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut queue = PriorityQueueImpl::<u64, Vec<u8>>::new();
        assert!(queue.is_empty());

        queue.insert(vec![0], 5);
        assert!(!queue.is_empty());
        assert_eq!(queue.peek(), Some(&vec![0]));

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

    #[test]
    fn it_works_on_empty() {
        let mut queue = PriorityQueueImpl::<u64, Vec<u8>>::new();
        assert!(queue.is_empty());
        assert_eq!(queue.peek(), None);
        assert_eq!(queue.pop(), None);
    }

    /// Note: multiple elements of the same priority are supported, test have been enabled.
    #[test]
    fn it_works_on_duplicate_priorities() {
        let mut queue = PriorityQueueImpl::<u64, Vec<u8>>::new();
        queue.insert(vec![22], 1);
        assert_eq!(queue.peek(), Some(&vec![22]));
        queue.insert(vec![11], 1);
        assert_eq!(queue.peek(), Some(&vec![22]));

        assert_eq!(queue.pop(), Some(vec![22]));
        assert_eq!(queue.pop(), Some(vec![11]));
    }

}
