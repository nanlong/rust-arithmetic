// 并查集
pub struct UnionFind {
    id: Vec<usize>,
    sz: Vec<usize>,
    count: usize,
}

impl UnionFind {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut this = UnionFind {
            id: Vec::with_capacity(capacity),
            sz: Vec::with_capacity(capacity),
            count: capacity,
        };

        for i in 0..capacity {
            this.id.push(i);
            this.sz.push(1);
        }

        this
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn find(&self, mut p: usize) -> usize {
        while p != self.id[p] {
            p = self.id[p];
        }

        p
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let i = self.find(p);
        let j = self.find(q);

        if i == j {
            return
        }

        if self.sz[i] < self.sz[j] {
            self.id[i] = j;
            self.sz[j] += self.sz[i];
        }
        else {
            self.id[j] = i;
            self.sz[i] += self.sz[j];
        }

        self.count -= 1;

    }
}

#[test]
fn test() {
    let tiny_uf = [
        (4, 3), (3, 8), (6, 5), (9, 4), (2, 1),
        (8, 9), (5, 0), (7, 2), (6, 1), (1, 0),
        (6, 7),
    ];

    let mut uf = UnionFind::with_capacity(10);

    for &(p, q) in tiny_uf.iter() {
        uf.union(p, q);
    }

    assert_eq!(uf.count(), 2);
    assert!(uf.connected(4, 8));
}