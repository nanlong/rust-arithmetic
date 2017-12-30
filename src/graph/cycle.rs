use super::graph::Graph;

// 环检测
pub struct Cycle {
    marked: Vec<bool>,
    has_cycle: bool,
}

impl Cycle {
    pub fn new(g: &Graph) -> Self {
        let mut this = Cycle {
            marked: Vec::with_capacity(g.v()),
            has_cycle: false,
        };

        for _ in 0..g.v() {
            this.marked.push(false);
        }

        for s in 0..g.v() {
            if ! this.marked[s] {
                this.dfs(g, s, s);
            }
        }

        this
    }

    fn dfs(&mut self, g: &Graph, v: usize, u: usize) {
        self.marked[v] = true;

        for w in g.adj(v) {
            if ! self.marked[*w] {
                self.dfs(g, *w, v);
            }
            else if *w == u {
                self.has_cycle = true;
            }
        }
    }

    pub fn has_cycle(&self) -> bool {
        self.has_cycle
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

    let cycle = Cycle::new(&g);
    assert!(cycle.has_cycle());
}