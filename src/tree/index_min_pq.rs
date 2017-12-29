// 索引优先队列
// https://algs4.cs.princeton.edu/24pq/IndexMinPQ.java
#[derive(Debug)]
pub struct IndexMinPQ<T> {
    n: usize,
    pq: Vec<Option<usize>>,
    qp: Vec<Option<usize>>,
    keys: Vec<Option<T>>,
}

impl<T: Copy + PartialOrd> IndexMinPQ<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut this = IndexMinPQ {
            n: 0,
            pq: Vec::with_capacity(capacity + 1),
            qp: Vec::with_capacity(capacity + 1),
            keys: Vec::with_capacity(capacity + 1),
        };

        for _ in 0..capacity + 1 {
            this.pq.push(None);
            this.qp.push(None);
            this.keys.push(None);
        }

        this
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn contains(&self, i: usize) -> bool {
        self.qp[i].is_some()
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn push(&mut self, i: usize, key: T) {
        assert!(! self.contains(i), "index is already in the priority queue: push");
        self.n += 1;
        let n = self.size();
        self.qp[i] = Some(n);
        self.pq[n] = Some(i);
        self.keys[i] = Some(key);
        self.swim(n);
    }

    pub fn change_key(&mut self, i: usize, key: T) {
        assert!(self.contains(i), "index is not in the priority queue");
        self.keys[i] = Some(key);
        let k = self.qp[i].unwrap();
        self.swim(k);
        self.sink(k);
    }

    pub fn change(&mut self, i: usize, key: T) {
        self.change_key(i, key);
    }

    pub fn pop(&mut self) -> usize {
        let min = self.pq[1].unwrap();

        let n = self.size();
        self.exch(1, n);
        self.n -= 1;
        self.sink(1);

        self.qp[min] = None;
        self.keys[min] = None;
        self.pq[n + 1] = None;

        min
    }

    fn greater(&self, i: usize, j: usize) -> bool {
        let i = self.pq[i].unwrap();
        let j = self.pq[j].unwrap();

        match (self.keys[i], self.keys[j]) {
            (Some(i_key), Some(j_key)) => i_key > j_key,
            (None, Some(_)) => false,
            (Some(_), None) => true,
            (None, None) => false,
        }
    }

    fn exch(&mut self, i: usize, j: usize) {
        self.pq.swap(i, j);
        self.qp[self.pq[i].unwrap()] = Some(i);
        self.qp[self.pq[j].unwrap()] = Some(j);
    }

    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.greater(k / 2, k) {
            self.exch(k, k / 2);
            k = k / 2;
        }
    }

    fn sink(&mut self, mut k: usize) {
        while 2 * k <= self.size() {
            let mut j = 2 * k;

            if j < self.size() && self.greater(j, j + 1) {
                j += 1;
            }

            if ! self.greater(k, j) {
                break
            }

            self.exch(k, j);
            k = j;
        }
    }
}

#[test]
fn test() {
    let mut pq = IndexMinPQ::with_capacity(10);

    pq.push(0, 0.33);
    pq.push(1, 0.0002);
    pq.push(5, 0.001);
    pq.push(8, 0.01);
    pq.change(1, 0.8);

    assert_eq!(pq.pop(), 5);
    assert_eq!(pq.pop(), 8);
    assert_eq!(pq.pop(), 0);
    assert_eq!(pq.pop(), 1);
}