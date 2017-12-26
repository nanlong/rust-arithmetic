use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use super::graph::Graph;

// 检测图是否有环
pub struct Cycle<T: Copy + Hash + Eq> {
    marked: HashMap<T, bool>,
    pub has_cycle: bool,
}

impl<T: Copy + Hash + Eq> Cycle<T> {
    pub fn new(g: &Graph<T>) -> Self {
        let mut cycle = Cycle {
            marked: HashMap::new(),
            has_cycle: false,
        };

        for v in g.vertices() {
            if let None = cycle.marked.get(v) {
                cycle.dfs(g, *v, *v);
            }
        }

        cycle
    }

    fn dfs(&mut self, g: &Graph<T>, v: T, u: T) {
        self.marked.insert(v, true);

        if let Some(edges) = g.adj(v) {
            for w in edges {
                if let None = self.marked.get(w) {
                    self.dfs(g, *w, v);
                }
                else if *w != u {
                    self.has_cycle = true;
                }
            }
        }
    }
}

#[test]
fn test() {
    let tiny_g = [
        (0, 5), (4, 3), (0, 1), (9, 12), (6, 4), (5, 4), (0, 2),
        (11, 12), (9, 10), (0, 6), (7, 8), (9, 11), (5, 3),
    ];

    let mut g = Graph::<i32>::new();

    for &(v, w) in tiny_g.iter() {
        g.add_edge(v, w);
    }

    let cycle = Cycle::new(&g);
    assert!(cycle.has_cycle);
}