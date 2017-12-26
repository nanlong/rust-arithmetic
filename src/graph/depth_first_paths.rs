use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use super::graph::Graph;

// 深度优先路径
pub struct DepthFirstPaths<T: Copy + Hash + Eq> {
    marked: HashMap<T, bool>,
    edge_to: HashMap<T, T>,
    count: usize,
    s: T,
}

impl<T: Copy + Hash + Eq> DepthFirstPaths<T> {
    pub fn new(g: &Graph<T>, s: T) -> Self {
        let mut dfp = DepthFirstPaths {
            marked: HashMap::new(),
            edge_to: HashMap::new(),
            count: 0,
            s,
        };

        dfp.dfs(g, s);
        dfp
    }

    fn dfs(&mut self, g: &Graph<T>, v: T) {
        self.marked.entry(v).or_insert(true);
        self.count += 1;

        if let Some(ref edges) = g.adj(v) {
            for w in edges.iter() {
                if let None = self.marked.get(w) {
                    self.edge_to.insert(*w, v);
                    self.dfs(g, *w);
                }
            }
        }
    }

    pub fn has_path_to(&self, v: T) -> bool {
        match self.marked.get(&v) {
            Some(&true) => true,
            Some(&false) | None => false,
        }
    }

    pub fn path_to(&self, v: T) -> Vec<T> {
        let mut res = Vec::new();
        let mut path = Vec::new();
        let mut x = v;

        while x != self.s {
            path.push(x);

            if let Some(ref w) = self.edge_to.get(&x) {
                x = **w;
            }
        }

        path.push(self.s);

        for v in path.iter().rev() {
            res.push(*v);
        }

        res
    }
}
