use std::{borrow::Borrow, cmp::Ordering};

pub(crate) struct Node<K, V> {
    key: K,
    val: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    priority: u64,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, val: V, priority: u64) -> Self {
        Self {
            key,
            val,
            left: None,
            right: None,
            priority,
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        use std::cmp::Ordering;

        match key.cmp(self.key.borrow()) {
            Ordering::Equal => Some(&self.val),
            Ordering::Less => self.left.as_ref().and_then(|n| n.get(key)),
            Ordering::Greater => self.right.as_ref().and_then(|n| n.get(key)),
        }
    }

    pub fn insert(&mut self, key: K, val: V, priority: u64) -> Option<V>
    where
        K: Ord,
    {
        match key.cmp(&self.key) {
            Ordering::Equal => {
                if self.priority < priority {
                    self.priority = priority;
                }

                // we don't update the key
                // See rationale in std::collections::BtreeMap docs.
                Some(std::mem::replace(&mut self.val, val))
            }
            Ordering::Less => {
                let old_val = if let Some(ref mut left) = self.left {
                    left.insert(key, val, priority)
                } else {
                    self.left = Some(Box::new(Node::new(key, val, priority)));
                    None
                };

                if self.is_heap_property_violated(&self.left) {
                    self.rotate_right();
                }

                old_val
            }
            Ordering::Greater => {
                let old_val = if let Some(ref mut right) = self.right {
                    right.insert(key, val, priority)
                } else {
                    self.right = Some(Box::new(Node::new(key, val, priority)));
                    None
                };

                // TODO: restore heap invariant
                if self.is_heap_property_violated(&self.right) {
                    self.rotate_left();
                }

                old_val
            }
        }
    }

    fn is_heap_property_violated(&self, subtree: &Option<Box<Node<K, V>>>) -> bool {
        if let Some(child) = subtree.as_ref() {
            self.priority < child.priority
        } else {
            false
        }
    }

    //        y             x
    //       / \           / \
    //      x   c -->    a    y
    //    / \                / \
    //  a    b              b   c
    fn rotate_right(&mut self) {
        use std::mem;

        let x = self.left.take();

        if let Some(mut x) = x {
            mem::swap(self, &mut x);
            mem::swap(&mut self.right, &mut x.left);
            let _ = mem::replace(&mut self.right, Some(x));
        }
    }

    //        y             x
    //       / \           / \
    //      a   x -->    y    c
    //         / \      / \
    //        b   c    a   b
    fn rotate_left(&mut self) {
        use std::mem;

        let x = self.right.take();

        if let Some(mut x) = x {
            mem::swap(self, &mut x);
            mem::swap(&mut self.left, &mut x.right);
            let _ = mem::replace(&mut self.left, Some(x));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_right() {
        let a = Box::new(Node {
            key: b'a',
            val: (),
            left: None,
            right: None,
            priority: 0,
        });
        let b = Box::new(Node {
            key: b'b',
            val: (),
            left: None,
            right: None,
            priority: 0,
        });
        let c = Box::new(Node {
            key: b'c',
            val: (),
            left: None,
            right: None,
            priority: 0,
        });

        let x = Box::new(Node {
            key: b'x',
            val: (),
            left: Some(a),
            right: Some(b),
            priority: 0,
        });

        let mut y = Box::new(Node {
            key: b'y',
            val: (),
            priority: 0,
            left: Some(x),
            right: Some(c),
        });

        y.rotate_right();

        assert_eq!(y.key, b'x');
        assert_eq!(y.left.unwrap().key, b'a');

        {
            let y = y.right.unwrap();
            assert_eq!(y.key, b'y');

            assert_eq!(y.left.unwrap().key, b'b');
            assert_eq!(y.right.unwrap().key, b'c');
        }
    }

    #[test]
    fn rotate_left() {
        let a = Box::new(Node {
            key: b'a',
            val: (),
            left: None,
            right: None,
            priority: 0,
        });
        let b = Box::new(Node {
            key: b'b',
            val: (),
            left: None,
            right: None,
            priority: 0,
        });
        let c = Box::new(Node {
            key: b'c',
            val: (),
            left: None,
            right: None,
            priority: 0,
        });

        let x = Box::new(Node {
            key: b'x',
            val: (),
            left: Some(b),
            right: Some(c),
            priority: 0,
        });

        let mut y = Box::new(Node {
            key: b'y',
            val: (),
            priority: 0,
            left: Some(a),
            right: Some(x),
        });

        y.rotate_left();

        assert_eq!(y.key, b'x');
        assert_eq!(y.right.unwrap().key, b'c');

        {
            let y = y.left.unwrap();
            assert_eq!(y.key, b'y');

            assert_eq!(y.left.unwrap().key, b'a');
            assert_eq!(y.right.unwrap().key, b'b');
        }
    }
}
