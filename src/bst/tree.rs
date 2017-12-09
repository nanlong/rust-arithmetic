use bst::tree_node::{TreeNode, ST};

pub struct BST<K, V> {
    root: TreeNode<K, V>,
}

impl<K: PartialOrd, V> BST<K, V> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn size(&self) -> usize {
        self.root.size()
    }

    pub fn put(&mut self, key: K, val: V) {
        self.root.put(key, val)
    }

    pub fn get(&self, key: K) -> &TreeNode<K, V> {
        self.root.get(key)
    }

    pub fn min(&self) -> &TreeNode<K, V> {
        self.root.min()
    }

    pub fn max(&self) -> &TreeNode<K, V> {
        self.root.max()
    }

    pub fn floor(&self, key: K) -> &TreeNode<K, V> {
        self.root.floor(key)
    }

    pub fn ceiling(&self, key: K) -> &TreeNode<K, V> {
        self.root.ceiling(key)
    }

    pub fn select(&self, k: usize) -> &TreeNode<K, V> {
        self.root.select(k)
    }

    pub fn rank(&self, key: K) -> usize {
        self.root.rank(key)
    }

    pub fn delete_min(&mut self) {
        self.root.delete_min()
    }

    pub fn delete_max(&mut self) {
        self.root.delete_max()
    }

    pub fn delete(&mut self, key: K) {
        self.root.delete(key)
    }
}


#[test]
fn test() {
    let mut bst = BST::<&str, isize>::new();
    assert_eq!(bst.size(), 0);

    bst.put("S", 1);
    bst.put("E", 2);
    bst.put("X", 3);
    bst.put("A", 4);
    bst.put("R", 5);
    bst.put("C", 6);
    bst.put("H", 7);
    bst.put("M", 8);

    assert_eq!(bst.size(), 8);


    match *bst.get("C") {
        Some(ref node) => assert_eq!(node.val, 6),
        None => assert!(false),
    }

    match *bst.min() {
        Some(ref node) => assert_eq!(node.key, "A"),
        None => assert!(false),
    }

    match *bst.max() {
        Some(ref node) => assert_eq!(node.key, "X"),
        None => assert!(false),
    }

    match *bst.floor("G") {
        Some(ref node) => assert_eq!(node.key, "E"),
        None => assert!(false),
    }

    match *bst.ceiling("G") {
        Some(ref node) => assert_eq!(node.key, "H"),
        None => assert!(false),
    }

    match *bst.select(5) {
        Some(ref node) => assert_eq!(node.key, "R"),
        None => assert!(false),
    }

    assert_eq!(bst.rank("R"), 5);

    bst.delete_min();
    assert_eq!(bst.size(), 7);

    match *bst.min() {
        Some(ref node) => assert_eq!(node.key, "C"),
        None => assert!(false),
    }

    bst.delete_max();
    assert_eq!(bst.size(), 6);

    match *bst.max() {
        Some(ref node) => assert_eq!(node.key, "S"),
        None => assert!(false),
    }

    bst.delete("E");
    match *bst.get("E") {
        Some(_) => assert!(false),
        None => assert!(true),
    }

    assert_eq!(bst.size(), 5);
}