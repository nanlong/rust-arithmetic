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

    pub fn delete_min(&mut self) {
        if ! self.root.left().is_red() && ! self.root.right().is_red() {
            self.root.change_red();
        }

        self.root.delete_min();

        if ! self.root.is_empty() {
            self.root.change_black();
        }
    }

    pub fn delete_max(&mut self) {
        if ! self.root.left().is_red() && ! self.root.right().is_red() {
            self.root.change_red();
        }

        self.root.delete_max();

        if ! self.root.is_empty() {
            self.root.change_black();
        }
    }

    pub fn delete(&mut self, key: K) {
        if ! self.root.left().is_red() && ! self.root.right().is_red() {
            self.root.change_red();
        }

        self.root.delete(key);

        if ! self.root.is_empty() {
            self.root.change_black();
        }
    }
}