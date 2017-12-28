use std::rc::Rc;
use super::edge::Edge;


// 加权无向图
pub struct EdgeWeightedGraph {
    v: usize,
    e: usize,
    adj: Vec<Vec<Rc<Edge>>>,
}

impl EdgeWeightedGraph {
    pub fn new(v: usize) -> Self {
        let mut this = EdgeWeightedGraph {
            v,
            e: 0,
            adj: Vec::with_capacity(v),
        };

        for _ in 0..v {
            this.adj.push(Vec::new());
        }

        this
    }

    pub fn v(&self) -> usize {
        self.v
    }

    pub fn e(&self) -> usize {
        self.e
    }

    pub fn add_edge(&mut self, edge: Edge) {
        let edge = Rc::new(edge);
        let v = edge.either();

        if let Some(w) = edge.other(v) {
            self.adj[v].push(edge.clone());
            self.adj[w].push(edge.clone());
            self.e += 1;
        }
    }

    pub fn adj(&self, v: usize) -> &Vec<Rc<Edge>> {
        &self.adj[v]
    }

    pub fn edges(&self) -> Vec<&Rc<Edge>> {
        let mut edges = Vec::with_capacity(self.e());

        for v in 0..self.v() {
            for edge in self.adj(v) {
                if let Some(w) = edge.other(v) {
                    if w > v {
                        edges.push(edge);
                    }
                }
            }
        }

        edges
    }
}


#[test]
fn test() {
    let tiny_ewg = [
        (4, 5, 0.35),
        (4, 7, 0.37),
        (5, 7, 0.28),
        (0, 7, 0.16),
        (1, 5, 0.32),
        (0, 4, 0.38),
        (2, 3, 0.17),
        (1, 7, 0.19),
        (0, 2, 0.29),
        (1, 2, 0.36),
        (1, 3, 0.39),
        (2, 7, 0.34),
        (6, 2, 0.40),
        (3, 6, 0.52),
        (6, 0, 0.58),
        (6, 4, 0.93),
    ];

    let mut ewg = EdgeWeightedGraph::new(8);

    for &(v, w, weight) in tiny_ewg.iter() {
        ewg.add_edge(Edge::new(v, w, weight));
    }

    assert_eq!(ewg.v(), 8);
    assert_eq!(ewg.e(), 16);
    assert_eq!(ewg.adj(0).len(), 4);
    assert_eq!(ewg.edges().len(), 16);
}