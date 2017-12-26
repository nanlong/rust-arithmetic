use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use std::collections::hash_map::Keys;


pub struct Graph<T: Copy> {
    v: usize,
    e: usize,
    _adj: HashMap<T, Vec<T>>,
}

impl<T: Copy + Hash + Eq> Graph<T> {
    pub fn new() -> Self {
        Graph {v: 0, e: 0, _adj: HashMap::new()}
    }

    pub fn add_edge(&mut self, v: T, w: T) {
        {
            let x_edge = self._adj.entry(v).or_insert(vec![]);
            x_edge.push(w);
        }

        {
            let y_edge = self._adj.entry(w).or_insert(vec![]);
            y_edge.push(v);
        }

        self.v = self._adj.keys().len();
        self.e += 1;
    }

    pub fn adj(&self, v: T) -> Option<&Vec<T>> {
        self._adj.get(&v)
    }

    pub fn vertices(&self) -> Keys<T, Vec<T>> {
        self._adj.keys()
    }
}

#[test]
fn test() {
    let tiny_cg = [
        (0, 5), (2, 4), (2, 3), (1, 2),
        (0, 1), (3, 4), (3, 5), (0, 2),
    ];

    let mut cg = Graph::new();

    for &(v, w) in tiny_cg.iter() {
        cg.add_edge(v, w);
    }

    assert_eq!(cg.v, 6);
    assert_eq!(cg.e, 8);
}