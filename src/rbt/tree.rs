use rbt::node::{Link, ST, Colors};

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
        if let Some(ref mut boxed_node) = self.root {
            if ! boxed_node.left.is_red() && ! boxed_node.right.is_red() {
                boxed_node.color = Colors::RED;
            }
        }

        self.root.delete_min();

        if self.root.size() > 0 {
            self.root.change_black();
        }
    }

    pub fn delete_max(&mut self) {
        if let Some(ref mut boxed_node) = self.root {
            if ! boxed_node.left.is_red() && ! boxed_node.right.is_red() {
                boxed_node.color = Colors::RED;
            }
        }

        self.root.delete_max();

        if self.root.size() > 0 {
            self.root.change_black();
        }
    }
}