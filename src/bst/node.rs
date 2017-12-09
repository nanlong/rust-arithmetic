use std::mem;

pub type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug, Clone)]
pub struct Node<K, V> {
    pub key: K,
    pub val: V,
    left: Link<K, V>,
    right: Link<K, V>,
    n: usize,
}

pub trait ST<K, V> {
    fn new(key: K, val: V) -> Link<K, V>;
    fn size(&self) -> usize;
    fn get(&self, key: K) -> &Link<K, V>;
    fn get_mut(&mut self, key: K) -> &mut Link<K, V>;
    fn put(&mut self, key: K, val: V);
    fn min(&self) -> &Link<K, V>;
    fn min_mut(&mut self) -> &mut Link<K, V>;
    fn max(&self) -> &Link<K, V>;
    fn ceiling(&self, key: K) -> &Link<K, V>;
    fn floor(&self, key: K) -> &Link<K, V>;
    fn select(&self, k: usize) -> &Link<K, V>;
    fn rank(&self, key: K) -> usize;
    fn delete_min(&mut self);
    fn delete_max(&mut self);
    fn delete(&mut self, key: K);
    fn delete_self(&mut self);
}


impl<K: PartialOrd, V> ST<K, V> for Link<K, V> {
    fn new(key: K, val: V) -> Self {
        let node = Box::new(Node {
            key,
            val,
            left: None,
            right: None,
            n: 1,
        });

        Some(node)
    }

    fn size(&self) -> usize {
        match *self {
            Some(ref node) => node.n,
            None => 0,
        }
    }

    fn get(&self, key: K) -> &Self {
        match {self} {
            &Some(ref node) if key != node.key => {
                if key < node.key {
                    node.left.get(key)
                }
                else {
                    node.right.get(key)
                }
            },
            node @ &Some(_) | node @ &None => node,
        }
    }

    fn get_mut(&mut self, key: K) -> &mut Self {
        match {self} {
            &mut Some(ref mut node) if key != node.key => {
                if key < node.key {
                    node.left.get_mut(key)
                }
                else {
                    node.right.get_mut(key)
                }
            },
            node @ &mut Some(_) | node @ &mut None => node,
        }
    }

    fn put(&mut self, key: K, val: V) {
        match *self {
            Some(ref mut node) => {
                if key < node.key {
                    node.left.put(key, val)
                }
                else if key > node.key {
                    node.right.put(key, val)
                }
                else {
                    node.val = val
                }

                node.n = node.left.size() + node.right.size() + 1
            },
            None => {
                let node = Box::new(Node {
                    key,
                    val,
                    left: None,
                    right: None,
                    n: 1,
                });

                *self = Some(node);
            },
        }
    }

    fn min(&self) -> &Self {
        match {self} {
            &Some(ref node) if node.left.is_some() => {
                node.left.min()
            },
            node @ &Some(_) | node @ &None => node,
        }
    }

    fn min_mut(&mut self) -> &mut Self {
        match {self} {
            &mut Some(ref mut node) if node.left.is_some() => {
                node.left.min_mut()
            },
            other @ &mut Some(_) | other @ &mut None => other,
        }
    }

    fn max(&self) -> &Self {
        match {self} {
            &Some(ref node) if node.right.is_some() => {
                node.right.max()
            },
            node @ &Some(_) | node @ &None => node,
        }
    }

    fn ceiling(&self, key: K) -> &Self {
        match *self {
            Some(ref node) => {
                if key < node.key {
                    let tree_node = node.left.ceiling(key);

                    if tree_node.is_none() {
                        &self
                    }
                    else {
                        tree_node
                    }
                }
                else if key > node.key {
                    node.right.ceiling(key)
                }
                else {
                    &self
                }
            },
            None => &self,
        }
    }

    fn floor(&self, key: K) -> &Self {
        match *self {
            Some(ref node) => {
                if key < node.key {
                    node.left.floor(key)
                }
                else if key > node.key {
                    let tree_node = node.right.floor(key);

                    if tree_node.is_none() {
                        &self
                    }
                    else {
                        tree_node
                    }
                }
                else {
                    &self
                }
            },
            None => &self,
        }
    }

    fn select(&self, k: usize) -> &Self {
        match *self {
            Some(ref node) => {
                let t = node.left.size();

                if t < k {
                    node.right.select(k - t - 1)
                }
                else if t > k {
                    node.left.select(k)
                }
                else {
                    &self
                }
            },
            None => &self,
        }
    }

    fn rank(&self, key: K) -> usize {
        match *self {
            Some(ref node) => {
                if key < node.key {
                    node.left.rank(key)
                }
                else if key > node.key {
                    1 + node.left.size() + node.right.rank(key)
                }
                else {
                    node.left.size()
                }
            },
            None => 0,
        }
    }

    fn delete_min(&mut self) {
        let mut has_left = true;

        match *self {
            Some(ref mut node) => {
                if node.left.is_none() {
                    has_left = false;
                }
                else {
                    node.left.delete_min();
                    node.n = node.left.size() + node.right.size() + 1;
                }
            }
            None => {},
        }

        if ! has_left {
            *self = self.take().unwrap().right;
        }
    }

    fn delete_max(&mut self) {
        let mut has_right = true;

        match *self {
            Some(ref mut node) => {
                if node.right.is_none() {
                    has_right = false;
                }
                else {
                    node.right.delete_max();
                    node.n = node.left.size() + node.right.size() + 1;
                }
            },
            None => {},
        }

        if ! has_right {
            *self = self.take().unwrap().left;
        }
    }

    fn delete(&mut self, key: K) {
        let mut is_self = false;

        if let &mut Some(ref mut node) = self {
            if key < node.key {
                node.left.delete(key);
            }
            else if key > node.key {
                node.right.delete(key);
            }
            else {
                is_self = true;
            }

            node.n = node.left.size() + node.right.size() + 1;
        }

        if is_self {
            self.delete_self();
        }
    }

    fn delete_self(&mut self) {
        if let Some(mut boxed_node) = self.take() {
            match (boxed_node.left.take(), boxed_node.right.take()) {
                (None, None) => {},
                (leaf @ Some(_), None) | (None, leaf @ Some(_)) => *self = leaf,
                (left, right) => {

                    boxed_node.left = left;
                    boxed_node.right = right;

                    {
                        let node = &mut *boxed_node;
                        let next = node.right.min_mut();

                        mem::swap(&mut node.key, &mut next.as_mut().unwrap().key);
                        mem::swap(&mut node.val, &mut next.as_mut().unwrap().val);
                    }

                    boxed_node.right.delete_min();
                    boxed_node.n = boxed_node.left.size() + boxed_node.right.size() + 1;
                    *self = Some(boxed_node)
                }
            }
        }
    }
}


#[test]
fn test() {
    let mut node = Link::new("S", 1);
    assert_eq!(node.size(), 1);

    node.put("S", 1);
    node.put("E", 2);
    node.put("X", 3);
    node.put("A", 4);
    node.put("R", 5);
    node.put("C", 6);
    node.put("H", 7);
    node.put("M", 8);

    assert_eq!(node.size(), 8);

    match *node.get("A") {
        Some(ref node) => assert_eq!(node.val, 4),
        None => assert!(false),
    }

    match *node.min() {
        Some(ref node) => assert_eq!(node.key, "A"),
        None => assert!(false),
    }

    match *node.max() {
        Some(ref node) => assert_eq!(node.key, "X"),
        None => assert!(false),
    }

    match *node.ceiling("G") {
        Some(ref node) => assert_eq!(node.key, "H"),
        None => assert!(false),
    }

    match *node.floor("G") {
        Some(ref node) => assert_eq!(node.key, "E"),
        None => assert!(false),
    }

    match *node.select(5) {
        Some(ref node) => assert_eq!(node.key, "R"),
        None => assert!(false),
    }

    assert_eq!(node.rank("R"), 5);

    node.delete_min();
    assert_eq!(node.size(), 7);

    match *node.min() {
        Some(ref node) => assert_eq!(node.key, "C"),
        None => assert!(false),
    }

    node.delete_max();
    assert_eq!(node.size(), 6);

    match *node.max() {
        Some(ref node) => assert_eq!(node.key, "S"),
        None => assert!(false),
    }

    node.delete("E");
    match *node.get("E") {
        Some(_) => assert!(false),
        None => assert!(true),
    }

    assert_eq!(node.size(), 5);
}