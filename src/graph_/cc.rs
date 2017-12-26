use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use super::graph::Graph;

// 连通分量
pub struct CC<T: Copy + Hash + Eq> {
    marked: HashMap<T, bool>,
    id: HashMap<T, usize>,
    pub count: usize,
}

impl<T: Copy + Hash + Eq> CC<T> {
    pub fn new(g: &Graph<T>) -> Self {
        let mut cc = CC {
            marked: HashMap::new(),
            id: HashMap::new(),
            count: 0,
        };

        for v in g.vertices() {
            if let None = cc.marked.get(v) {
                cc.dfs(g, *v);
                cc.count += 1;
            }
        }

        cc
    }

    pub fn dfs(&mut self, g: &Graph<T>, v: T) {
        self.marked.insert(v, true);
        self.id.insert(v, self.count);

        if let Some(ref edges) = g.adj(v) {
            for w in edges.iter() {
                if let None = self.marked.get(&w) {
                    self.dfs(g, *w);
                }
            }
        }
    }

    pub fn connected(&self, v: T, w: T) -> bool {
        if let (Some(v_marked), Some(w_marked)) = (self.id.get(&v), self.id.get(&w)) {
            v_marked == w_marked
        }
        else {
            false
        }
    }
}


#[test]
fn test() {
    let tiny_g = [
        (0, 5), (4, 3), (0, 1), (9, 12), (6, 4), (5, 4), (0, 2),
        (11, 12), (9, 10), (0, 6), (7, 8), (9, 11), (5, 3),
    ];

    let mut g = Graph::new();

    for &(v, w) in tiny_g.iter() {
        g.add_edge(v, w);
    }

    let cc = CC::new(&g);
    assert_eq!(cc.count, 3);
}