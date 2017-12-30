use super::graph::Graph;

// 二分图检测
pub struct TwoColor {
    marked: Vec<bool>,
    color: Vec<bool>,
    is_two_colorable: bool,
}

impl TwoColor {
    pub fn new(g: &Graph) -> Self {
        let mut this = TwoColor {
            marked: Vec::with_capacity(g.v()),
            color: Vec::with_capacity(g.v()),
            is_two_colorable: true,
        };

        for _ in 0..g.v() {
            this.marked.push(false);
            this.color.push(false);
        }

        for s in 0..g.v() {
            if ! this.marked[s] {
                this.dfs(g, s);
            }
        }

        this
    }

    fn dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;

        for w in g.adj(v) {
            if ! self.marked[*w] {
                self.color[*w] = ! self.color[v];
                self.dfs(g, *w);
            }
            else if self.color[*w] == self.color[v] {
                self.is_two_colorable = false;
            }
        }
    }

    pub fn is_two_colorable(&self) -> bool {
        self.is_two_colorable
    }
}


#[test]
fn test() {
    // 生成一个图
    let tiny_g = [
        (0, 5), (4, 3), (0, 1), (9, 12), (6, 4), (5, 4), (0, 2),
        (11, 12), (9, 10), (0, 6), (7, 8), (9, 11), (5, 3),
    ];

    let mut g = Graph::with_capacity(13);

    for &(v, w) in tiny_g.iter() {
        g.add_edge(v, w);
    }

    let two_color = TwoColor::new(&g);
    assert!(! two_color.is_two_colorable());

    // 生成一个二分图
    let two_g = [
        (0, 2), (0, 3),
        (1, 2), (1, 3),
    ];

    let mut g = Graph::with_capacity(13);

    for &(v, w) in two_g.iter() {
        g.add_edge(v, w);
    }

    let two_color = TwoColor::new(&g);
    assert!(two_color.is_two_colorable());
}
