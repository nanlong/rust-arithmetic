use std::fmt;

// 图
pub struct Graph {
    v: usize,
    e: usize,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // 给定顶点数量，初始化图
    pub fn new(v: usize) -> Self {
        let mut this = Graph {
            v,
            e: 0,
            adj: Vec::with_capacity(v),
        };

        for _ in 0..v {
            this.adj.push(Vec::new());
        }

        this
    }

    // 顶点数量
    pub fn v(&self) -> usize {
        self.v
    }

    // 边数量
    pub fn e(&self) -> usize {
        self.e
    }

    // 增加边
    pub fn add_edge(&mut self, v: usize, w: usize) {
        // 避免平行边和自环
        if (! self.adj[v].contains(&w) && ! self.adj[w].contains(&v)) || (v != w) {
            self.adj[v].push(w);
            self.adj[w].push(v);
            self.e += 1;
        }
    }

    // 顶点指向的顶点
    pub fn adj(&self, v: usize) -> &Vec<usize> {
        &self.adj[v]
    }

    // 顶点度数
    pub fn degree(&self, v: usize) -> usize {
        self.adj(v).len()
    }

    // 最大顶点度数
    pub fn max_degree(&self) -> usize {
        let mut max = 0;

        for i in 0..self.v() {
            if self.degree(i) > max {
                max = self.degree(i)
            }
        }

        max
    }

    // 平均顶点度数
    pub fn avg_degree(&self) -> usize {
        2 * self.e() / self.v()
    }

    // 自环个数
    pub fn number_of_self_loops(&self) -> usize {
        let mut count = 0;

        for v in 0..self.v() {
            for w in self.adj(v) {
                if w == &v {
                    count += 1;
                }
            }
        }

        count / 2
    }
}


impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();

        string.push_str(fmt::format(format_args!("{} vertices, {} edges \n", self.v(), self.e())).as_str());

        for v in 0..self.v() {
            string.push_str(fmt::format(format_args!("{}: {:?} \n", v, self.adj(v))).as_str());
        }

        write!(f, "{}", string)
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

    assert_eq!(g.v(), 13);
    assert_eq!(g.e(), 13);
    assert_eq!(g.adj(0), &[5, 1, 2, 6])
}
