use std::cmp::Ordering;

// 加权图中的边
pub struct Edge {
    v: usize,
    w: usize,
    weight: f32,
}

impl Edge {
    pub fn new(v: usize, w: usize, weight: f32) -> Self {
        Edge {v, w, weight}
    }

    // 权重
    pub fn weight(&self) -> f32 {
        self.weight
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

    // 边的对比
    pub fn compare_to(&self, edge: &Edge) -> Ordering {
        if self.weight() < edge.weight() {
            Ordering::Less
        }
        else if self.weight() > edge.weight() {
            Ordering::Greater
        }
        else {
            Ordering::Equal
        }
    }
}


#[test]
fn test() {
    let edge = Edge::new(0, 5, 0.8);
    assert_eq!(edge.weight(), 0.8);
    assert_eq!(edge.either(), 0);
    assert_eq!(edge.other(0), Some(5));
    assert_eq!(edge.other(1), None);

    let edge2 = Edge::new(1, 2, 1.1);
    assert_eq!(edge.compare_to(&edge2), Ordering::Less);
    assert_eq!(edge2.compare_to(&edge), Ordering::Greater);
}