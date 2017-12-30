use std::rc::Rc;
use std::collections::BinaryHeap;
use super::edge::Edge;
use super::edge_weighted_graph::EdgeWeightedGraph;

// 最小生成树 Prim 算法（延迟版本）
pub struct LazyPrimMST {
    marked: Vec<bool>,          // 最小生成树的顶点
    mst: Vec<Rc<Edge>>,         // 最小生成树的边
    pq: BinaryHeap<Rc<Edge>>,   // 横切边（包括失效的边）这个优先队列由小到大排序 原因看 Edge 的代码实现
}

impl LazyPrimMST {
    pub fn new(g: &EdgeWeightedGraph) -> Self {
        let mut this = LazyPrimMST {
            marked: Vec::with_capacity(g.v()),
            mst: Vec::with_capacity(g.v() - 1),
            pq: BinaryHeap::new(),
        };

        for _ in 0..g.v() {
            this.marked.push(false);
        }

        this.visit(g, 0);

        while ! this.pq.is_empty() {
            // 从 pq 中得到权重最小的边
            let edge = this.pq.pop().unwrap();

            // 跳过失效的边
            let v = edge.either();
            let w = edge.other(v).unwrap();

            if this.marked[v] && this.marked[w] {
                continue
            }

            // 将边添加到树中
            this.mst.push(edge.clone());

            // 将顶点（ v 或者 w ）添加到树中
            if ! this.marked[v] {
                this.visit(g, v);
            }

            if ! this.marked[w] {
                this.visit(g, w);
            }
        }

        this
    }

    pub fn visit(&mut self, g: &EdgeWeightedGraph, v: usize) {
        self.marked[v] = true;

        for edge in g.adj(v) {
            let w = edge.other(v).unwrap();

            if ! self.marked[w] {
                self.pq.push(edge.clone());
            }
        }
    }

    // 最小生成树的边
    pub fn edges(&self) -> Vec<Rc<Edge>> {
        self.mst.to_vec()
    }

    // 最小生成树的权重
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

    let mst = LazyPrimMST::new(&g);

    // 最小边包含
    //    0-7 0.16
    //    1-7 0.19
    //    0-2 0.26
    //    2-3 0.17
    //    5-7 0.28
    //    4-5 0.35
    //    6-2 0.40
    assert_eq!(mst.edges().len(), g.v() - 1);
    assert_eq!(mst.weight(), 1.8100001);
}