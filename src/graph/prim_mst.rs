use std::f32;
use std::rc::Rc;
use super::edge::Edge;
use super::edge_weighted_graph::EdgeWeightedGraph;
use super::super::queue::index_min_pq::IndexMinPQ;

// 最小生成树 Prim 算法（即时版本）
pub struct PrimMST {
    edge_to: Vec<Option<Rc<Edge>>>,
    dist_to: Vec<f32>,
    marked: Vec<bool>,
    pq: IndexMinPQ<f32>,
}

impl PrimMST {
    pub fn new(g: &EdgeWeightedGraph) -> Self {
        let mut this = PrimMST {
            edge_to: Vec::with_capacity(g.v()),
            dist_to: Vec::with_capacity(g.v()),
            marked: Vec::with_capacity(g.v()),
            pq: IndexMinPQ::with_capacity(g.v()),
        };

        for _ in 0..g.v() {
            this.edge_to.push(None);
            this.dist_to.push(f32::INFINITY);
            this.marked.push(false);
        }

        this.dist_to[0] = 0.0;
        this.pq.push(0, 0.0);

        while ! this.pq.is_empty() {
            let v = this.pq.pop();
            this.visit(g, v);
        }

        this
    }

    pub fn visit(&mut self, g: &EdgeWeightedGraph, v: usize) {
        self.marked[v] = true;

        for e in g.adj(v) {
            let w = e.other(v).unwrap();

            if self.marked[w] {
                continue
            }

            if e.weight() < self.dist_to[w] {
                self.edge_to[w] = Some(e.clone());
                self.dist_to[w] = e.weight();

                if self.pq.contains(w) {
                    self.pq.change(w, e.weight());
                }
                else {
                    self.pq.push(w, e.weight());
                }
            }
        }
    }

    pub fn edges(&self) -> Vec<Rc<Edge>> {
        let mut edges = Vec::new();

        for e in &self.edge_to {
            if let &Some(ref e) = e {
                edges.push(e.clone());
            }
        }

        edges
    }

    pub fn weight(&self) -> f32 {
        let mut weight = 0.0;

        for edge in self.edges() {
            weight += edge.weight();
        }

        weight
    }
}

#[test]
fn test() {
    let tiny_ewg = [
        (4, 5, 0.35), (4, 7, 0.37), (5, 7, 0.28), (0, 7, 0.16),
        (1, 5, 0.32), (0, 4, 0.38), (2, 3, 0.17), (1, 7, 0.19),
        (0, 2, 0.26), (1, 2, 0.36), (1, 3, 0.39), (2, 7, 0.34),
        (6, 2, 0.40), (3, 6, 0.52), (6, 0, 0.58), (6, 4, 0.93),
    ];

    let mut g = EdgeWeightedGraph::new(8);

    for &(v, w, weight) in tiny_ewg.iter() {
        g.add_edge(Edge::new(v, w, weight));
    }

    let mst = PrimMST::new(&g);

    //    1-7 0.19
    //    0-2 0.26
    //    2-3 0.17
    //    4-5 0.35
    //    5-7 0.28
    //    6-2 0.4
    //    0-7 0.16
    assert_eq!(mst.edges().len(), g.v() - 1);
    assert_eq!(mst.weight(), 1.81);
}