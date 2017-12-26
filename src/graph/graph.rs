pub struct Graph {
    v: usize,
    e: usize,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(v: usize) -> Self {
        let mut graph = Graph {
            v,
            e: 0,
            adj: Vec::with_capacity(v),
        };

        for _ in 0..v {
            graph.adj.push(Vec::new());
        }

        graph
    }

    pub fn v(&self) -> usize {
        self.v
    }

    pub fn e(&self) -> usize {
        self.e
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.adj[v].push(w);
        self.adj[w].push(v);
        self.e += 1;
    }

    pub fn adj(&self, v: usize) -> &Vec<usize> {
        &self.adj[v]
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
