use rbt::node::{Link, ST};

#[derive(Debug)]
pub struct RBT<K, V> {
    root: Link<K, V>,
}

impl<K: PartialOrd, V> RBT<K, V> {
    pub fn new() -> Self {
        RBT { root: None }
    }

    pub fn put(&mut self, key: K, val: V) {
        self.root.put(key, val);
        self.root.change_black();
    }
}