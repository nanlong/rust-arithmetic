use std::mem;
use std::cmp::Ordering;

pub type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
pub enum Colors {
    RED,
    BLACK,
}

pub trait ST<K, V> {
    fn new(key: K, val: V) -> Link<K, V>;
    fn get(&self, key: K) -> &Link<K, V>;
    fn size(&self) -> usize;
    fn update_n(&mut self);
    fn is_red(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn left(&self) -> &Link<K, V>;
    fn left_mut(&mut self) -> &mut Link<K, V>;
    fn right(&self) -> &Link<K, V>;
    fn right_mut(&mut self) -> &mut Link<K, V>;
    fn min_mut(&mut self) -> &mut Link<K, V>;
    fn change_color(&mut self, color: Colors);
    fn change_red(&mut self);
    fn change_black(&mut self);
    fn rotate_left(&mut self);
    fn rotate_right(&mut self);
    fn flip_colors(&mut self);
    fn put(&mut self, key: K, val: V);
    fn flip_colors_inverse(&mut self);
    fn balance(&mut self);
    fn move_red_left(&mut self);
    fn delete_min(&mut self);
    fn move_red_right(&mut self);
    fn delete_max(&mut self);
    fn compare(&self, key: &K) -> Option<Ordering>;
    fn delete(&mut self, key: K);
}

#[derive(Debug)]
pub struct Node<K, V> {
    pub key: K,
    pub val: V,
    pub n: usize,
    pub color: Colors,
    pub left: Link<K, V>,
    pub right: Link<K, V>,
}

impl<K: PartialOrd, V> ST<K, V> for Link<K, V> {
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

    fn get(&self, key: K) -> &Self {
        match {self} {
            &Some(ref boxed_node) => {
                if key < boxed_node.key {
                    boxed_node.left.get(key)
                }
                else if key > boxed_node.key {
                    boxed_node.right.get(key)
                }
                else {
                    &self
                }
            },
            other => &other,
        }
    }

    fn size(&self) -> usize {
        match *self {
            Some(ref boxed_node) => boxed_node.n,
            None => 0,
        }
    }

    fn update_n(&mut self) {
        match *self {
            Some(ref mut boxed_node) => {
                boxed_node.n = boxed_node.left.size() + boxed_node.right.size() + 1;
            },
            None => {},
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

    fn is_empty(&self) -> bool {
        self.size() <= 0
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

    fn min_mut(&mut self) -> &mut Self {
        match {self} {
            &mut Some(ref mut node) if node.left.is_some() => {
                node.left.min_mut()
            },
            node @ &mut Some(_) | node @ &mut None => node,
        }
    }

    fn change_color(&mut self, color: Colors) {
        match *self {
            Some(ref mut boxed_node) => {
                boxed_node.color = color;
            },
            None => {},
        }
    }

    fn change_red(&mut self) {
        self.change_color(Colors::RED);
    }

    fn change_black(&mut self) {
        self.change_color(Colors::BLACK);
    }

    fn rotate_left(&mut self) {
        if let Some(mut boxed_node) = self.take() {
            let mut x = boxed_node.right.take().unwrap();
            boxed_node.right = x.left.take();

            mem::swap(&mut x.color, &mut boxed_node.color);
            boxed_node.color = Colors::RED;
            x.n = boxed_node.n;
            boxed_node.n = boxed_node.left.size() + boxed_node.right.size() + 1;

            x.left = Some(boxed_node);
            *self = Some(x);
        }
    }

    fn rotate_right(&mut self) {
        if let Some(mut boxed_node) = self.take() {
            let mut x = boxed_node.left.take().unwrap();
            boxed_node.left = x.right.take();

            mem::swap(&mut x.color, &mut boxed_node.color);
            boxed_node.color = Colors::RED;
            x.n = boxed_node.n;
            boxed_node.n = 1 + boxed_node.left.size() + boxed_node.right.size();

            x.right = Some(boxed_node);
            *self = Some(x);
        }
    }

    fn flip_colors(&mut self) {
        self.change_red();
        self.as_mut().unwrap().left.change_black();
        self.as_mut().unwrap().right.change_black();
    }

    fn put(&mut self, key: K, val: V) {
        match *self {
            Some(ref mut boxed_node) if key == boxed_node.key => {
                boxed_node.val = val;
            },
            Some(ref mut boxed_node) => {
                if key < boxed_node.key {
                    boxed_node.left.put(key, val);
                }
                else {
                    boxed_node.right.put(key, val);
                }
            },
            None => {
                *self = Link::new(key, val);
            }
        }

        if ! self.left().is_red() && self.right().is_red() {
            self.rotate_left();
        }

        if self.left().is_red() && self.left().left().is_red() {
            self.rotate_right();
        }

        if self.left().is_red() && self.right().is_red() {
            self.flip_colors();
        }

        self.update_n();
    }

    fn flip_colors_inverse(&mut self) {
        self.change_black();
        self.as_mut().unwrap().left.change_red();
        self.as_mut().unwrap().right.change_red();
    }

    fn balance(&mut self) {
        if self.right().is_red() {
            self.rotate_left();
        }

        if ! self.left().is_red() && self.right().is_red() {
            self.rotate_left();
        }

        if self.left().is_red() && self.left().left().is_red() {
            self.rotate_right();
        }

        if self.left().is_red() && self.right().is_red() {
            self.flip_colors();
        }

        self.update_n();
    }

    fn move_red_left(&mut self) {
        self.flip_colors_inverse();

        if self.right().left().is_red() {
            self.right_mut().rotate_right();
            self.rotate_left();
        }
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
        self.balance();
    }

    fn move_red_right(&mut self) {
        self.flip_colors_inverse();

        if ! self.left().left().is_red() {
            self.rotate_right();
        }
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
        self.balance();
    }

    fn compare(&self, key: &K) -> Option<Ordering> {
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

    fn delete(&mut self, key: K) {
        match self.compare(&key) {
            Some(Ordering::Less) => {
                if ! self.left().is_red() && ! self.left().left().is_red() {
                    self.move_red_left();
                }

                self.left_mut().delete(key);
            },
            Some(_) => {
                if self.left().is_red() {
                    self.rotate_right();
                }

                if let Some(Ordering::Equal) = self.compare(&key) {
                    if self.right().is_none() {
                        *self = None;
                        return
                    }
                }

                if ! self.right().is_red() && ! self.right().left().is_red() {
                    self.move_red_right();
                }

                if let Some(Ordering::Equal) = self.compare(&key) {
                    if let Some(mut boxed_node) = self.take() {
                        {
                            let node = &mut *boxed_node;
                            let next = node.right.min_mut();

                            mem::swap(&mut node.key, &mut next.as_mut().unwrap().key);
                            mem::swap(&mut node.val, &mut next.as_mut().unwrap().val);
                            mem::swap(&mut node.n, &mut next.as_mut().unwrap().n);
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

        self.balance();
    }
}


#[test]
fn test() {
    let mut tree_node = Link::new("A", 1);
    assert_eq!(tree_node.is_red(), true);
}