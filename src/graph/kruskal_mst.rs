use std::rc::Rc;
use std::collections::BinaryHeap;
use super::union_find::UnionFind;
use super::edge::Edge;
use super::edge_weighted_graph::EdgeWeightedGraph;


pub struct KruskalMST {
    mst: Vec<Rc<Edge>>,
    pq: BinaryHeap<Rc<Edge>>,
    un: UnionFind,
}

impl KruskalMST {
    pub fn new(g: &EdgeWeightedGraph) -> Self {
        let mut this = KruskalMST {
            mst: Vec::new(),
            pq: BinaryHeap::new(),
            un: UnionFind::with_capacity(g.v()),
        };

        for edge in g.edges() {
            this.pq.push(edge.clone());
        }

        while ! this.pq.is_empty() && this.mst.len() < g.v() - 1 {
            let edge = this.pq.pop().unwrap();
            let v = edge.either();
            let w = edge.other(v).unwrap();

            if this.un.connected(v, w) {
                continue
            }

            this.un.union(v, w);
            this.mst.push(edge);
        }

        this
    }

    pub fn edges(&self) -> Vec<Rc<Edge>> {
        let mut edges = Vec::new();

        for edge in &self.mst {
            edges.push(edge.clone());
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

    let mut g = EdgeWeightedGraph::with_capacity(8);

    for &(v, w, weight) in tiny_ewg.iter() {
        g.add_edge(Edge::new(v, w, weight));
    }

    let mst = KruskalMST::new(&g);

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