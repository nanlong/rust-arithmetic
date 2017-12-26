use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use std::fmt::Debug;
use super::graph::Graph;

// 深度优先搜索
pub struct DepthFirstSearch<T: Copy + Hash + Eq> {
    marked: HashMap<T, bool>,
    count: usize,
}

impl<T: Copy + Hash + Eq + Debug> DepthFirstSearch<T> {
    pub fn new(g: &Graph<T>, s: T) -> Self {
        let mut dfs = DepthFirstSearch {
            marked: HashMap::new(),
            count: 0,
        };
        dfs.dfs(g, s);
        dfs
    }

    fn dfs(&mut self, g: &Graph<T>, s: T) {
        self.marked.entry(s).or_insert(true);
        self.count += 1;

        if let Some(ref edges) = g.adj(s) {
            for w in edges.iter() {
                if let None = self.marked.get(w) {
                    self.dfs(g, *w);
                }
            }
        }
    }

    pub fn is_marked(&self, v: T) -> bool {
        match self.marked.get(&v) {
            Some(&true) => true,
            Some(&false) | None => false,
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

    let dfs = DepthFirstSearch::new(&g, 0);
    assert_eq!(dfs.is_marked(5), true);
}