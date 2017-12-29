use std::rc::Rc;
use super::edge::Edge;
use super::edge_weighted_graph::EdgeWeightedGraph;

// 最小生成树的API
pub trait MST {
    fn new(ewg: &EdgeWeightedGraph);
    fn edges(&self) -> Vec<&Rc<Edge>>;
    fn weight(&self) -> f32;
}