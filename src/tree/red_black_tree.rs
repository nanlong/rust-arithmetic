use std::cmp::Ordering;

type Link<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
enum Colors {
    RED,
    BLACK,
}

#[derive(Debug)]
pub struct Node<K, V> {
    key: K,
    val: V,
    n: usize,
    color: Colors,
    left: Link<K, V>,
    right: Link<K, V>,
}

trait ST<K, V> {
    fn new(key: K, val: V) -> Link<K, V>;
    fn put(&mut self, key: K, val: V);
    fn size(&self) -> usize;
    fn is_red(&self) -> bool;
    fn left(&self) -> &Link<K, V>;
    fn left_mut(&mut self) -> &mut Link<K, V>;
    fn right(&self) -> &Link<K, V>;
    fn right_mut(&mut self) -> &mut Link<K, V>;
    fn rotate_left(&mut self);
    fn rotate_right(&mut self);
    fn flip_colors(&mut self, is_delete: bool);
    fn balance(&mut self, is_delete: bool);
    fn move_red_left(&mut self);
    fn move_red_right(&mut self);
    fn compare_key(key: &K, link: &Link<K, V>) -> Option<Ordering>;
    fn delete(&mut self, key: K);
    fn delete_min(mut link: Link<K, V>) -> (Link<K, V>, Link<K, V>);
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

    fn put(&mut self, key: K, val: V) {
        match *self {
            Some(ref mut boxed_node) => {
                if key < boxed_node.key {
                    boxed_node.left.put(key, val);
                }
                    else if key > boxed_node.key {
                        boxed_node.right.put(key, val);
                    }
                        else {
                            boxed_node.val = val;
                        }
            },
            None => {
                *self = Self::new(key, val);
            }
        }

        self.balance(false);
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
                match &boxed_node.color {
                    &Colors::RED => true,
                    _ => false,
                }
            },
            _ => false,
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

    fn rotate_left(&mut self) {
        let mut h = self.take();
        let mut x = h.right_mut().take();

        x.as_mut().map(|boxed_node| {
            boxed_node.color = match &h.as_mut().unwrap().color {
                &Colors::RED => Colors::RED,
                &Colors::BLACK => Colors::BLACK,
            };
            boxed_node.n = h.size();
        });

        h.as_mut().map(|boxed_node| {
            boxed_node.right = x.left_mut().take();
            boxed_node.color = Colors::RED;
            boxed_node.n = boxed_node.left.size() + boxed_node.right.size() + 1;
        });

        x.as_mut().map(|boxed_node| boxed_node.left = h);

        *self = x;
    }

    fn rotate_right(&mut self) {
        let mut h = self.take();
        let mut x = h.left_mut().take();

        x.as_mut().map(|boxed_node| {
            boxed_node.color = match &h.as_mut().unwrap().color {
                &Colors::RED => Colors::RED,
                &Colors::BLACK => Colors::BLACK,
            };
            boxed_node.n = h.size();
        });

        h.as_mut().map(|boxed_node| {
            boxed_node.left = x.right_mut().take();
            boxed_node.color = Colors::RED;
            boxed_node.n = boxed_node.left.size() + boxed_node.right.size() + 1;
        });

        x.as_mut().map(|boxed_node| boxed_node.right = h);

        *self = x;
    }

    fn flip_colors(&mut self, is_delete: bool) {
        if is_delete {
            self.as_mut().map(|boxed_node| {
                boxed_node.color = Colors::BLACK;
                boxed_node.left.as_mut().map(|boxed_node| {
                    boxed_node.color = Colors::RED;
                });
                boxed_node.right.as_mut().map(|boxed_node| {
                    boxed_node.color = Colors::RED;
                });
            });
        }
            else {
                self.as_mut().map(|boxed_node| {
                    boxed_node.color = Colors::RED;
                    boxed_node.left.as_mut().map(|boxed_node| {
                        boxed_node.color = Colors::BLACK;
                    });
                    boxed_node.right.as_mut().map(|boxed_node| {
                        boxed_node.color = Colors::BLACK;
                    });
                });
            }
    }

    fn balance(&mut self, is_delete: bool) {
        if is_delete && self.right().is_red() {
            self.rotate_left();
        }

        if ! self.left().is_red() && self.right().is_red() {
            self.rotate_left();
        }

        if self.left().is_red() && self.left().left().is_red() {
            self.rotate_right();
        }

        if self.left().is_red() && self.right().is_red() {
            self.flip_colors(false);
        }

        self.as_mut().map(|boxed_node| {
            boxed_node.n = boxed_node.left.size() + boxed_node.right.size() + 1
        });
    }

    fn move_red_left(&mut self) {
        self.flip_colors(true);

        if self.right().left().is_red() {
            self.right_mut().rotate_left();
            self.rotate_left();
        }
    }

    fn move_red_right(&mut self) {
        self.flip_colors(true);

        if self.left().left().is_red() {
            self.rotate_right();
        }
    }

    fn compare_key(key: &K, link: &Link<K, V>) -> Option<Ordering> {
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

    fn delete(&mut self, key: K) {
        match Self::compare_key(&key, &self) {
            Some(Ordering::Less) => {
                if ! self.left().is_red() && self.left().left().is_red() {
                    self.move_red_left();
                }

                self.left_mut().delete(key);
            },
            Some(_) => {
                if self.left().is_red() {
                    self.rotate_right();
                }

                if let Some(Ordering::Equal) = Self::compare_key(&key, &self) {
                    if self.right().is_none() {
                        *self = None;
                        return
                    }
                }

                if ! self.right().is_red() && ! self.right().left().is_red() {
                    self.move_red_right();
                }

                if let Some(Ordering::Equal) = Self::compare_key(&key, &self) {
                    let mut node = self.take();
                    let (root, mut min) = Self::delete_min(node.right_mut().take());

                    min.as_mut().map(|boxed_node| {
                        boxed_node.right = root;
                        boxed_node.left = node.left_mut().take();
                        boxed_node.n = node.as_mut().unwrap().n;
                    });

                    *self = min;
                }
                    else {
                        self.right_mut().delete(key);
                    }
            },
            None => {},
        }

        self.balance(true);
    }

    fn delete_min(mut link: Link<K, V>) -> (Link<K, V>, Link<K, V>) {
        if link.is_none() {
            return (None, None)
        }

        match link.left_mut().take() {
            left @ Some(_) => {
                let (root, min) = Self::delete_min(left);
                link.as_mut().map(|boxed_node| boxed_node.left = root);
                (link, min)
            },
            None => (link.right_mut().take(), link),
        }
    }
}

#[derive(Debug)]
pub struct RedBlackTree<K, V> {
    root: Link<K, V>,
}

impl<K, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    pub fn put(&mut self, key: K, val: V) {
        self.root.put(key, val);
    }
}