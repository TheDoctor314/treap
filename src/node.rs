use std::borrow::Borrow;

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
