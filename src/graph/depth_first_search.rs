use super::graph::Graph;

// 深度优先搜索
pub struct DepthFirstSearch {
    marked: Vec<bool>,
    count: usize,
}

impl DepthFirstSearch {
    pub fn new(g: &Graph, s: usize) -> Self {
        let mut this = DepthFirstSearch {
            marked: Vec::with_capacity(g.v()),
            count: 0,
        };

        for _ in 0..g.v() {
            this.marked.push(false);
        }

        this.dfs(g, s);

        this
    }

    fn dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;
        self.count += 1;

        for w in g.adj(v) {
            if ! self.marked[*w] {
                self.dfs(g, *w);
            }
        }
    }

    pub fn marked(&self, v: usize) -> bool {
        self.marked[v]
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

#[test]
fn test() {
    let tiny_g = [
        (0, 5), (4, 3), (0, 1), (9, 12), (6, 4), (5, 4), (0, 2),
        (11, 12), (9, 10), (0, 6), (7, 8), (9, 11), (5, 3),
    ];

    let mut g = Graph::new(13);

    for &(v, w) in tiny_g.iter() {
        g.add_edge(v, w);
    }

    let dfs = DepthFirstSearch::new(&g, 0);

    assert!(dfs.marked(4));
    assert!(! dfs.marked(12));
    assert_eq!(dfs.count(), 7);
}