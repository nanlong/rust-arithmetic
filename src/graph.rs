use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use std::fmt::Debug;
use std::collections::VecDeque;


pub struct Graph<T: Copy> {
    v: usize,
    e: usize,
    _adj: HashMap<T, Vec<T>>,
}

impl<T: Copy + Hash + Eq> Graph<T> {
    pub fn new() -> Self {
        Graph {v: 0, e: 0, _adj: HashMap::new()}
    }

    pub fn add_edge(&mut self, v: T, w: T) {
        {
            let x_edge = self._adj.entry(v).or_insert(vec![]);
            x_edge.push(w);
        }

        {
            let y_edge = self._adj.entry(w).or_insert(vec![]);
            y_edge.push(v);
        }

        self.v = self._adj.keys().len();
        self.e += 1;
    }

    pub fn adj(&self, v: T) -> Option<&Vec<T>> {
        self._adj.get(&v)
    }
}


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
    let mut g = Graph::new();

    g.add_edge(0 ,5);
    assert_eq!(g.adj(0), Some(&vec![5]));

    g.add_edge(2, 4);
    g.add_edge(2, 3);
    g.add_edge(1, 2);
    g.add_edge(0, 1);
    g.add_edge(3, 4);
    g.add_edge(3, 5);
    g.add_edge(0, 2);

    assert_eq!(g.v, 6);
    assert_eq!(g.e, 8);

    let dfs = DepthFirstSearch::new(&g, 0);
    assert_eq!(dfs.is_marked(5), true);

    let dfp = DepthFirstPaths::new(&g, 0);
    assert_eq!(dfp.has_path_to(4), true);
    assert_eq!(dfp.path_to(4), [0, 5, 3, 2, 4]);

    let bfp = BreadthFirstPaths::new(&g, 0);
    assert_eq!(bfp.has_path_to(4), true);
    assert_eq!(bfp.path_to(4), [0, 2, 4]);
    assert_eq!(bfp.has_path_to(5), true);
    assert_eq!(bfp.path_to(5), [0, 5]);
}