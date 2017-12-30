use super::graph::Graph;

// 深度优先搜索路径
pub struct DepthFirstPaths {
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    s: usize,
}

impl DepthFirstPaths {
    pub fn new(g: &Graph, s: usize) -> Self {
        let mut this = DepthFirstPaths {
            marked: Vec::with_capacity(g.v()),
            edge_to: Vec::with_capacity(g.v()),
            s,
        };

        for _ in 0..g.v() {
            this.marked.push(false);
            this.edge_to.push(None);
        }

        this.dfs(g, s);

        this
    }

    fn dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;

        for w in g.adj(v) {
            if ! self.marked[*w] {
                self.edge_to[*w] = Some(v);
                self.dfs(g, *w);
            }
        }
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked[v]
    }

    pub fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        if ! self.has_path_to(v) {
            return None
        }

        let mut res = Vec::new();
        let mut path = Vec::new();
        let mut x = v;

        while x != self.s {
            path.push(x);

            x = match self.edge_to[x] {
                Some(v) => v,
                None => self.s,
            }
        }

        path.push(self.s);

        for v in path.iter().rev() {
            res.push(*v);
        }

        Some(res)
    }
}

#[test]
fn test() {
    let tiny_g = [
        (0, 5), (4, 3), (0, 1), (9, 12), (6, 4), (5, 4), (0, 2),
        (11, 12), (9, 10), (0, 6), (7, 8), (9, 11), (5, 3),
    ];

    let mut g = Graph::with_capacity(13);

    for &(v, w) in tiny_g.iter() {
        g.add_edge(v, w);
    }

    let dfp = DepthFirstPaths::new(&g, 0);

    assert!(dfp.has_path_to(6));
    assert!(! dfp.has_path_to(9));
    assert_eq!(dfp.path_to(6), Some(vec![0, 5, 4, 6]));
    assert_eq!(dfp.path_to(9), None);
}