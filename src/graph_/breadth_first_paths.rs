use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use std::collections::VecDeque;
use super::graph::Graph;

// 广度优先路径
pub struct BreadthFirstPaths<T: Copy + Hash + Eq> {
    marked: HashMap<T, bool>,
    edge_to: HashMap<T, T>,
    count: usize,
    s: T,
}

impl<T: Copy + Hash + Eq> BreadthFirstPaths<T> {
    pub fn new(g: &Graph<T>, s: T) -> Self {
        let mut bfp = BreadthFirstPaths {
            marked: HashMap::new(),
            edge_to: HashMap::new(),
            count: 0,
            s,
        };

        bfp.bfs(g, s);
        bfp
    }

    fn bfs(&mut self, g: &Graph<T>, v: T) {
        let mut queue = VecDeque::new();
        queue.push_back(v);

        while ! queue.is_empty() {
            let v = queue.pop_front().unwrap();

            if let Some(ref edges) = g.adj(v) {
                for w in edges.iter() {
                    if let None = self.marked.get(w) {
                        self.marked.insert(*w, true);
                        self.edge_to.insert(*w, v);
                        self.count += 1;
                        queue.push_back(*w);
                    }
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

    let bfp = BreadthFirstPaths::new(&g, 0);
    assert_eq!(bfp.has_path_to(4), true);
    assert_eq!(bfp.path_to(4), [0, 2, 4]);
    assert_eq!(bfp.has_path_to(5), true);
    assert_eq!(bfp.path_to(5), [0, 5]);
}