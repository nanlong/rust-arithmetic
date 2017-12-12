use rbt::node::{Link, ST};

#[derive(Debug)]
pub struct RBT<K, V> {
    root: Link<K, V>,
}

impl<K: PartialOrd, V> RBT<K, V> {
    pub fn new() -> Self {
        RBT { root: None }
    }

    pub fn get(&self, key: K) -> &Link<K, V> {
        self.root.get(key)
    }

    pub fn put(&mut self, key: K, val: V) {
        self.root.put(key, val);
        self.root.change_black();
    }

    pub fn size(&self) -> usize {
        self.root.size()
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

#[test]
fn test() {
    let mut rbt = RBT::<&str, isize>::new();

    rbt.put("S", 1);
    rbt.put("E", 2);
    rbt.put("X", 3);
    rbt.put("A", 4);
    rbt.put("R", 5);
    rbt.put("C", 6);
    rbt.put("H", 7);
    rbt.put("M", 8);

    rbt.delete_min();
    assert!(rbt.get("A").is_none());
    assert_eq!(rbt.size(), 7);

    rbt.delete_max();
    assert!(rbt.get("X").is_none());
    assert_eq!(rbt.size(), 6);

    rbt.delete("S");
    assert!(rbt.get("S").is_none());
    assert_eq!(rbt.size(), 5);

    rbt.delete("C");
    assert!(rbt.get("C").is_none());
    assert_eq!(rbt.size(), 4);
}