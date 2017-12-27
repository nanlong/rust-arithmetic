use std::cmp::Ordering;

type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    val: V,
    n: usize,
    h: isize,
    left: Link<K, V>,
    right: Link<K, V>,
}

trait LinkMethods<K, V> {
    fn new(key: K, val: V) -> Link<K, V>;
    fn put(&mut self, key: K, val: V);
    fn size(&self) -> usize;
    fn height(&self) -> isize;
    fn update_size(&mut self);
    fn update_height(&mut self);
    fn compare_key(key: &K, link: &Link<K, V>) -> Option<Ordering>;
    fn left(&self) -> &Link<K, V>;
    fn right(&self) -> &Link<K, V>;
    fn left_mut(&mut self) -> &mut Link<K, V>;
    fn right_mut(&mut self) -> &mut Link<K, V>;
    fn rotate_left(&mut self);
    fn rotate_right(&mut self);
    fn balance(&mut self);
    fn is_balance(&self) -> bool;
}

impl<K : PartialOrd, V> LinkMethods<K, V> for Link<K, V> {
    fn new(key: K, val: V) -> Self {
        let boxed_node = Box::new(Node {
            key,
            val,
            n: 1,
            h: 1,
            left: None,
            right: None,
        });

        Some(boxed_node)
    }

    fn put(&mut self, key: K, val: V) {
        match Self::compare_key(&key, &self) {
            Some(Ordering::Less) => self.as_mut().unwrap().left.put(key, val),
            Some(Ordering::Greater) => self.as_mut().unwrap().right.put(key, val),
            Some(Ordering::Equal) => {
                self.as_mut().map(|node| node.val = val);
            },
            None => *self = Self::new(key, val),
        }

        self.balance();
    }

    fn size(&self) -> usize {
        match *self {
            Some(ref boxed_node) => boxed_node.n,
            None => 0,
        }
    }

    fn height(&self) -> isize {
        match *self {
            Some(ref boxed_node) => boxed_node.h,
            None => 0,
        }
    }

    fn update_size(&mut self) {
        self.as_mut().map(|node| {
            node.n = node.left.size() + node.right.size() + 1
        });
    }

    fn update_height(&mut self) {
        self.as_mut().map(|node| {
            let left_height = node.left.height();
            let right_height = node.right.height();

            node.h = left_height.max(right_height) + 1;
        });
    }

    fn compare_key(key: &K, link: &Self) -> Option<Ordering> {
        match *link {
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

    fn left(&self) -> &Self {
        &self.as_ref().unwrap().left
    }

    fn right(&self) -> &Self {
        &self.as_ref().unwrap().right
    }

    fn left_mut(&mut self) -> &mut Self {
        &mut self.as_mut().unwrap().left
    }

    fn right_mut(&mut self) -> &mut Self {
        &mut self.as_mut().unwrap().right
    }

    fn rotate_left(&mut self) {
        let mut h = self.take();
        let mut x = h.right_mut().take();

        x.as_mut().map(|node| {
            node.n = h.size();
            node.h = h.height();
        });

        h.as_mut().map(|node| {
            node.right = x.left_mut().take();
        });
        h.update_size();
        h.update_height();

        x.as_mut().map(|node| node.left = h);

        *self = x;
    }

    fn rotate_right(&mut self) {
        let mut h = self.take();
        let mut x = h.left_mut().take();

        x.as_mut().map(|node| {
            node.n = h.size();
            node.h = h.height();
        });

        h.as_mut().map(|node| {
            node.left = x.right_mut().take();
        });
        h.update_size();
        h.update_height();

        x.as_mut().map(|node| node.right = h);

        *self = x;
    }

    fn balance(&mut self) {
        if self.is_some() {
            let diff = self.left().height() - self.right().height();

            if diff < 2 && diff > -2 {
                // 1 0 -1
                self.left_mut().balance();
                self.right_mut().balance();
            }
            else if diff > 1 {
                if self.left().left().height() - self.left().right().height() == -1 {
                    self.left_mut().rotate_left();
                }
                self.rotate_right();
                self.balance();
            }
            else if diff < -1 {
                if self.right().left().height() - self.right().right().height() == 1 {
                    self.right_mut().rotate_right();
                }
                self.rotate_left();
                self.balance();
            }

            self.update_size();
            self.update_height();
        }
    }

    fn is_balance(&self) -> bool {
        let diff = self.left().height() - self.right().height();
        diff < 2 && diff > -2
    }
}

#[derive(Debug)]
pub struct AvlTree<K, V> {
    root: Link<K, V>,
}

impl<K : PartialOrd, V> AvlTree<K, V> {
    pub fn new() -> Self {
        AvlTree { root: None }
    }

    pub fn put(&mut self, key: K, val: V) {
        self.root.put(key, val);
    }

    pub fn is_balance(&self) -> bool {
        self.root.is_balance()
    }
}

#[test]
fn test() {
    extern crate rand;
    let mut tree = AvlTree::<u16, usize>::new();

    for i in 1..2000 {
        tree.put(rand::random::<u16>(), i);
    }

    assert!(tree.is_balance());
}