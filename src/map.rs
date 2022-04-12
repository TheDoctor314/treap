use std::borrow::Borrow;

use crate::node::Node;

pub struct TreapMap<K, V> {
    root: Option<Node<K, V>>,
    len: usize,
}

impl<K, V> TreapMap<K, V> {
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V>
    where
        K: Ord,
    {
        let priority = rand::random();

        if let Some(root) = &mut self.root {
            let res = root.insert(key, val, priority);

            if res.is_none() {
                self.len += 1;
            }

            res
        } else {
            self.root = Some(Node::new(key, val, priority));
            self.len += 1;

            None
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.root.as_ref().and_then(|n| n.get(key))
    }
}

impl<K, V> Default for TreapMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
