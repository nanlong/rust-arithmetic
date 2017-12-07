pub type TreeNode<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
pub struct Node<K, V> {
    pub key: K,
    pub val: V,
    left: TreeNode<K, V>,
    right: TreeNode<K, V>,
    n: usize,
}

pub trait ST<K, V> {
    fn new(key: K, val: V) -> TreeNode<K, V>;
    fn size(&self) -> usize;
    fn get(&self, key: K) -> &TreeNode<K, V>;
    fn put(&mut self, key: K, val: V);
    fn min(&self) -> &TreeNode<K, V>;
    fn max(&self) -> &TreeNode<K, V>;
    fn ceiling(&self, key: K) -> &TreeNode<K, V>;
    fn floor(&self, key: K) -> &TreeNode<K, V>;
    fn select(&self, k: usize) -> &TreeNode<K, V>;
    fn rank(&self, key: K) -> usize;
    fn delete_min(&mut self);
    fn delete_max(&mut self);
}


impl<K: PartialOrd, V> ST<K, V> for TreeNode<K, V> {
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
        match *self {
            Some(ref node) => {
                if key < node.key {
                    node.left.get(key)
                }
                else if key > node.key {
                    node.right.get(key)
                }
                else {
                    &self
                }
            },
            None => &self,
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
        match *self {
            Some(ref node) => {
                if node.left.is_none() {
                    &self
                }
                else {
                    node.left.min()
                }
            },
            None => &self,
        }
    }

    fn max(&self) -> &Self {
        match *self {
            Some(ref node) => {
                if node.right.is_none() {
                    &self
                }
                else {
                    node.right.max()
                }
            },
            None => &self,
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
}

#[test]
fn test() {
    let mut tree_node = TreeNode::new("S", 1);
    assert_eq!(tree_node.size(), 1);

    tree_node.put("S", 1);
    tree_node.put("E", 2);
    tree_node.put("X", 3);
    tree_node.put("A", 4);
    tree_node.put("R", 5);
    tree_node.put("C", 6);
    tree_node.put("H", 7);
    tree_node.put("M", 8);

    assert_eq!(tree_node.size(), 8);

    match *tree_node.get("A") {
        Some(ref node) => assert_eq!(node.val, 4),
        None => assert!(false),
    }

    match *tree_node.min() {
        Some(ref node) => assert_eq!(node.key, "A"),
        None => assert!(false),
    }

    match *tree_node.max() {
        Some(ref node) => assert_eq!(node.key, "X"),
        None => assert!(false),
    }

    match *tree_node.ceiling("G") {
        Some(ref node) => assert_eq!(node.key, "H"),
        None => assert!(false),
    }

    match *tree_node.floor("G") {
        Some(ref node) => assert_eq!(node.key, "E"),
        None => assert!(false),
    }

    match *tree_node.select(5) {
        Some(ref node) => assert_eq!(node.key, "R"),
        None => assert!(false),
    }

    assert_eq!(tree_node.rank("R"), 5);

    tree_node.delete_min();
    assert_eq!(tree_node.size(), 7);

    match *tree_node.min() {
        Some(ref node) => assert_eq!(node.key, "C"),
        None => assert!(false),
    }

    tree_node.delete_max();
    assert_eq!(tree_node.size(), 6);

    match * tree_node.max() {
        Some(ref node) => assert_eq!(node.key, "S"),
        None => assert!(false),
    }
}