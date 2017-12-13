use std::mem;
use std::cmp::Ordering;

pub type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
pub struct Node<K, V> {
    key: K,
    val: V,
    n: usize,
    color: Colors,
    left: Link<K, V>,
    right: Link<K, V>,
}

#[derive(Debug)]
enum Colors {
    RED,
    BLACK,
}

trait LinkMethods<K, V> {
    fn new(key: K, val: V) -> Link<K, V>;
    fn put(&mut self, key: K, val: V);
    fn get(&self, key: K) -> &Link<K, V>;
    fn delete(&mut self, key: K);
    fn delete_min(&mut self);
    fn delete_max(&mut self);
    fn size(&self) -> usize;
    fn is_red(&self) -> bool;
    fn left(&self) -> &Link<K, V>;
    fn left_mut(&mut self) -> &mut Link<K, V>;
    fn right(&self) -> &Link<K, V>;
    fn right_mut(&mut self) -> &mut Link<K, V>;
    fn min(&self) -> &Link<K, V>;
    fn min_mut(&mut self) -> &mut Link<K, V>;
    fn max(&self) -> &Link<K, V>;
    fn rotate_left(&mut self);
    fn rotate_right(&mut self);
    fn flip_colors(&mut self);
    fn balance(&mut self);
    fn compare_key(&self, key: &K) -> Option<Ordering>;
    fn move_red_left(&mut self);
    fn move_red_right(&mut self);
}

impl<K: PartialOrd, V> LinkMethods<K, V> for Link<K, V> {
    fn new(key: K, val: V) -> Self {
        let boxed_node = Box::new(Node {
            key,
            val,
            n: 1,
            color: Colors::RED,
            left: None,
            right: None,
        });

        Some(boxed_node)
    }

    fn put(&mut self, key: K, val: V) {
        match self.compare_key(&key) {
            Some(Ordering::Less) => self.left_mut().put(key, val),
            Some(Ordering::Greater) => self.right_mut().put(key, val),
            Some(Ordering::Equal) => {
                self.as_mut().map(|node| node.val = val);
            },
            None => *self = Self::new(key, val),
        };

        self.balance();
    }

    fn get(&self, key: K) -> &Self {
        match self.compare_key(&key) {
            Some(Ordering::Less) => self.left().get(key),
            Some(Ordering::Greater) => self.right().get(key),
            Some(Ordering::Equal) | None => &self,
        }
    }

    fn delete(&mut self, key: K) {
        match self.compare_key(&key) {
            Some(Ordering::Less) => {
                if ! self.left().is_red() && ! self.left().left().is_red() {
                    self.move_red_left();
                }

                self.left_mut().delete(key);
            },
            Some(Ordering::Greater) | Some(Ordering::Equal) => {
                if self.left().is_red() {
                    self.rotate_right();
                }

                if let Some(Ordering::Equal) = self.compare_key(&key) {
                    if self.right().is_none() {
                        *self = None;
                        return
                    }
                }

                if ! self.right().is_red() && ! self.right().left().is_red() {
                    self.move_red_right();
                }

                if let Some(Ordering::Equal) = self.compare_key(&key) {
                    if let Some(mut boxed_node) = self.take() {
                        {
                            let node = &mut *boxed_node;
                            let next = node.right.min_mut();
                            mem::swap(&mut node.key, &mut next.as_mut().unwrap().key);
                            mem::swap(&mut node.val, &mut next.as_mut().unwrap().val);
                        }

                        boxed_node.right.delete_min();

                        *self = Some(boxed_node);
                    }
                }
                else {
                    self.right_mut().delete(key);
                }
            },
            None => {},
        }

        if self.right().is_red() {
            self.rotate_left();
        }

        self.balance();
    }

    fn delete_min(&mut self) {
        if self.left().is_none() {
            *self = None;
            return
        }

        if ! self.left().is_red() && ! self.left().left().is_red() {
            self.move_red_left();
        }

        self.left_mut().delete_min();

        if self.right().is_red() {
            self.rotate_left();
        }

        self.balance();
    }

    fn delete_max(&mut self) {
        if self.left().is_red() {
            self.rotate_right();
        }

        if self.right().is_none() {
            *self = None;
            return
        }

        if ! self.right().is_red() && ! self.right().left().is_red() {
            self.move_red_right();
        }

        self.right_mut().delete_max();

        if self.right().is_red() {
            self.rotate_left();
        }

        self.balance();
    }

    fn size(&self) -> usize {
        match *self {
            Some(ref boxed_node) => boxed_node.n,
            None => 0,
        }
    }

    fn is_red(&self) -> bool {
        match *self {
            Some(ref boxed_node) => {
                match boxed_node.color {
                    Colors::RED => true,
                    Colors::BLACK => false,
                }
            },
            None => false,
        }
    }

    fn left(&self) -> &Self {
        &self.as_ref().unwrap().left
    }

    fn left_mut(&mut self) -> &mut Self {
        &mut self.as_mut().unwrap().left
    }

    fn right(&self) -> &Self {
        &self.as_ref().unwrap().right
    }

    fn right_mut(&mut self) -> &mut Self {
        &mut self.as_mut().unwrap().right
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
            node @ &mut Some(_) | node @ &mut None => node,
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

    fn rotate_left(&mut self) {
        let mut h = self.take();
        let mut x = h.right_mut().take();

        x.as_mut().map(|node| {
            node.color = match &h.as_ref().unwrap().color {
                &Colors::RED => Colors::RED,
                &Colors::BLACK => Colors::BLACK,
            };
            node.n = h.as_ref().unwrap().n;
        });

        h.as_mut().map(|node| {
            node.color = Colors::RED;
            node.right = x.left_mut().take();
            node.n = node.left.size() + node.right.size() + 1;
        });

        x.as_mut().map(|node| node.left = h);

        *self = x;
    }

    fn rotate_right(&mut self) {
        let mut h = self.take();
        let mut x = h.left_mut().take();

        x.as_mut().map(|node| {
            node.color = match &h.as_ref().unwrap().color {
                &Colors::RED => Colors::RED,
                &Colors::BLACK => Colors::BLACK,
            };
            node.n = h.as_ref().unwrap().n;
        });

        h.as_mut().map(|node| {
            node.color = Colors::RED;
            node.left = x.right_mut().take();
            node.n = node.left.size() + node.right.size() + 1;
        });

        x.as_mut().map(|node| node.right = h);

        *self = x;
    }

    fn flip_colors(&mut self) {
        self.as_mut().map(|node| {
            node.color = Colors::RED;
            node.left.as_mut().map(|node| node.color = Colors::BLACK);
            node.right.as_mut().map(|node| node.color = Colors::BLACK);
        });
    }

    fn balance(&mut self) {
        if ! self.left().is_red() && self.right().is_red() {
            self.rotate_left();
        }

        if self.left().is_red() && self.left().left().is_red() {
            self.rotate_right();
        }

        if self.left().is_red() && self.right().is_red() {
            self.flip_colors();
        }

        self.as_mut().map(|node| {
            node.n = node.left.size() + node.right.size() + 1;
        });
    }

    fn compare_key(&self, key: &K) -> Option<Ordering> {
        match *self {
            Some(ref boxed_node) => {
                if key < &boxed_node.key {
                    Some(Ordering::Less)
                }
                else if key > &boxed_node.key {
                    Some(Ordering::Greater)
                }
                else {
                    Some(Ordering::Equal)
                }
            },
            None => None,
        }
    }

    fn move_red_left(&mut self) {
        self.as_mut().map(|node| {
            node.color = Colors::BLACK;
            node.left.as_mut().map(|node| node.color = Colors::RED);
            node.right.as_mut().map(|node| node.color = Colors::RED);
        });

        if self.right().left().is_red() {
            self.right_mut().rotate_left();
            self.rotate_right();
        }
    }

    fn move_red_right(&mut self) {
        self.as_mut().map(|node| {
            node.color = Colors::BLACK;
            node.left.as_mut().map(|node| node.color = Colors::RED);
            node.right.as_mut().map(|node| node.color = Colors::RED);
        });

        if self.left().left().is_red() {
            self.rotate_right();
        }
    }
}

#[derive(Debug)]
pub struct RedBlackBST<K, V> {
    root: Link<K, V>,
}

impl<K: PartialOrd, V> RedBlackBST<K, V> {
    pub fn new() -> Self {
        RedBlackBST { root: None }
    }

    pub fn put(&mut self, key: K, val: V) {
        self.root.put(key, val);
    }

    pub fn get(&self, key: K) -> &Link<K, V> {
        self.root.get(key)
    }

    pub fn delete(&mut self, key: K) {
        if ! self.root.left().is_red() && ! self.root.right().is_red() {
            self.root.as_mut().map(|node| node.color = Colors::RED);
        }

        self.root.delete(key);

        if self.root.size() > 0 {
            self.root.as_mut().map(|node| node.color = Colors::BLACK);
        }
    }

    pub fn delete_min(&mut self) {
        if ! self.root.left().is_red() && ! self.root.right().is_red() {
            self.root.as_mut().map(|node| node.color = Colors::RED);
        }

        self.root.delete_min();

        if self.root.size() > 0 {
            self.root.as_mut().map(|node| node.color = Colors::BLACK);
        }
    }

    pub fn delete_max(&mut self) {
        if ! self.root.left().is_red() && ! self.root.right().is_red() {
            self.root.as_mut().map(|node| node.color = Colors::RED);
        }

        self.root.delete_max();

        if self.root.size() > 0 {
            self.root.as_mut().map(|node| node.color = Colors::BLACK);
        }
    }

    pub fn size(&self) -> usize {
        self.root.size()
    }

    pub fn min(&self) -> &Link<K, V> {
        self.root.min()
    }

    pub fn max(&self) -> &Link<K, V> {
        self.root.max()
    }
}


#[test]
fn test() {
    let mut tree = RedBlackBST::<&str, isize>::new();

    tree.put("S", 1);
    tree.put("E", 2);
    tree.put("X", 3);
    tree.put("A", 4);
    tree.put("R", 5);
    tree.put("C", 6);
    tree.put("H", 7);
    tree.put("M", 8);

    assert_eq!(tree.min().as_ref().unwrap().key, "A");
    assert_eq!(tree.max().as_ref().unwrap().key, "X");

    assert_eq!(tree.size(), 8);
    assert!(tree.get("S").is_some());

    tree.delete_min();
    assert_eq!(tree.size(), 7);
    assert!(tree.get("A").is_none());

    tree.delete_max();
    assert_eq!(tree.size(), 6);
    assert!(tree.get("X").is_none());

    tree.delete("S");
    assert_eq!(tree.size(), 5);
    assert!(tree.get("S").is_none());
}