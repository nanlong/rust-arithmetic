use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use super::graph::Graph;


pub struct TwoColor<T: Copy + Hash + Eq> {
    marked: HashMap<T, bool>,
    color: HashMap<T, bool>,
    is_two_color: bool,
}

impl<T: Copy + Hash + Eq> TwoColor<T> {
    pub fn new(g: &Graph<T>) -> Self {
        let mut two_color = TwoColor {
            marked: HashMap::new(),
            color: HashMap::new(),
            is_two_color: true,
        };

        for s in g.vertices() {
            if let None = two_color.marked.get(s) {
                two_color.dfs(g, *s);
            }
        }

        two_color
    }

    fn dfs(&mut self, g: &Graph<T>, v: T) {
        self.marked.insert(v, true);
        self.color.entry(v).or_insert(false);

        if let Some(edges) = g.adj(v) {
            for w in edges.iter() {
                if let None = self.marked.get(w) {
                    let color = ! self.color(v);
                    self.color.insert(*w, color);
                    self.dfs(g, *w);
                }
                else if self.color(*w) == self.color(v) {
                    self.is_two_color = false;
                }
            }
        }
    }

    fn color(&self, v: T) -> bool {
        match self.color.get(&v) {
            Some(&false) | None => false,
            Some(&true) => true,
        }
    }
}

#[test]
fn test() {
    let tiny_cg = [
        (0, 5), (2, 4), (2, 3), (1, 2),
        (0, 1), (3, 4), (3, 5), (0, 2),
    ];

    let mut g = Graph::<i32>::new();

    for &(v, w) in tiny_cg.iter() {
        g.add_edge(v, w);
    }

    let two_color = TwoColor::new(&g);
    assert_eq!(two_color.is_two_color, false);
}