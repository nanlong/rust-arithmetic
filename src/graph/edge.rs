use std::cmp::Ordering;
use std::f32;

// 加权图中的边
#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
pub struct Edge {
    v: usize,
    w: usize,
    weight: u32,
}


impl Edge {
    pub fn new(v: usize, w: usize, weight: f32) -> Self {
        Edge {v, w, weight: weight.to_bits()}
    }

    // 权重
    pub fn weight(&self) -> f32 {
        f32::from_bits(self.weight)
    }

    // 其中的1个顶点
    pub fn either(&self) -> usize {
        self.v
    }

    // 给定一个顶点，返回另一个
    pub fn other(&self, v: usize) -> Option<usize> {
        if v == self.v {
            Some(self.w)
        }
        else if v == self.w {
            Some(self.v)
        }
        else {
            None
        }
    }
}


#[test]
fn test() {
    let ref mut edge = Edge::new(0, 5, 0.8);
    assert_eq!(edge.weight(), 0.8);
    assert_eq!(edge.either(), 0);
    assert_eq!(edge.other(0), Some(5));
    assert_eq!(edge.other(1), None);

    let ref mut edge2 = Edge::new(1, 2, 1.1);
    assert_eq!(edge.cmp(&edge2), Ordering::Less);
    assert_eq!(edge2.cmp(&edge), Ordering::Greater);
}