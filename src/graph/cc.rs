use super::graph::Graph;

// 连通分量
pub struct CC {
    marked: Vec<bool>,
    id: Vec<usize>,
    count: usize,
}

impl CC {
    pub fn new(g: &Graph) -> Self {
        let mut this = CC {
            marked: Vec::with_capacity(g.v()),
            id: Vec::with_capacity(g.v()),
            count: 0,
        };

        // 初始化值
        for _ in 0..g.v() {
            this.marked.push(false);
            this.id.push(0);
        }

        // 通过深度优先搜索计算连通分量
        for v in 0..g.v() {
            if ! this.marked[v] {
                this.dfs(g, v);
                this.count += 1;
            }
        }

        this
    }

    fn dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;
        self.id[v] = self.count;

        for w in g.adj(v) {
            if ! self.marked[*w] {
                self.dfs(g, *w);
            }
        }
    }

    // 给定两个顶点，判断是否为同一连通分量
    pub fn connected(&self, v: usize, w: usize) -> bool {
        self.id[v] == self.id[w]
    }

    pub fn id(&self) -> &Vec<usize> {
        &self.id
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

    let mut g = Graph::with_capacity(13);

    for &(v, w) in tiny_g.iter() {
        g.add_edge(v, w);
    }

    let cc = CC::new(&g);

    assert_eq!(cc.count(), 3);
    assert!(cc.connected(2, 6));
    assert!(! cc.connected(6, 7));

    let mut components = Vec::new();

    for _ in 0..cc.count() {
        components.push(Vec::new());
    }

    for v in 0..g.v() {
        components[cc.id()[v]].push(v);
    }

    assert_eq!(components[0], [0, 1, 2, 3, 4, 5, 6]);
    assert_eq!(components[1], [7, 8]);
    assert_eq!(components[2], [9, 10, 11, 12]);
}