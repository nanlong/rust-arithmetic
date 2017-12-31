// 索引优先队列
pub struct IndexBinaryHeap<T> {
    n: usize,                   // 元素数量
    pq: Vec<Option<usize>>,     // pq[n], 第 n 名是什么数字
    qp: Vec<Option<usize>>,     // qp[n], 数字 n 是什么排名
    keys: Vec<Option<T>>,       // keys[n], 数字 n 关联的对象
}

impl<T: PartialOrd> IndexBinaryHeap<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut this = IndexBinaryHeap {
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

    pub fn put(&mut self, i: usize, key: T) {
        if ! self.contains(i) {
            // 添加元素, 放到最后，然后上浮
            let k = self.n + 1;

            self.n = k;
            self.pq[k] = Some(i);
            self.qp[i] = Some(k);
            self.keys[i] = Some(key);
            self.swim(k);
        }
        else {
            // 更新元素，先对当前位置上浮，再对当前位置下沉
            let k = self.qp[i].unwrap();

            self.keys[i] = Some(key);
            self.swim(k);
            self.sink(k);
        }
    }

    pub fn pop(&mut self) -> usize {
        // 堆顶
        let v = self.pq[1].unwrap();

        // 与堆尾交换
        let k = self.size();
        self.swap(1, k);

        // 调整堆，调整到 k - 1 的位置
        self.n -= 1;
        self.sink(1);

        // 删掉值
        self.pq[k] = None;
        self.qp[v] = None;
        self.keys[v] = None;

        v
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn contains(&self, i: usize) -> bool {
        self.qp[i].is_some()
    }

    pub fn size(&self) -> usize {
        self.n
    }

    fn compare(&self, i: usize, j: usize) -> bool {
        if let (Some(n), Some(m)) = (self.pq[i], self.pq[j]) {
            match (&self.keys[n], &self.keys[m]) {
                (&Some(ref n_key), &Some(ref m_key)) => n_key < m_key,
                (&None, &Some(_)) => true,
                (&Some(_), &None) => false,
                (&None, &None) => false,
            }
        }
        else {
            false
        }
    }

    // 交换元素
    fn swap(&mut self, i: usize, j: usize) {
        if let (Some(n), Some(m)) = (self.pq[i], self.pq[j]) {
            self.qp.swap(n, m);
            self.pq.swap(i, j);
        }
    }

    // 上浮元素
    fn swim(&mut self, mut k: usize) {
        // k 不能为根节点, k 的父节点为 k / 2
        while k > 1 && self.compare(k / 2, k) {
            self.swap(k / 2, k);
            k = k / 2;
        }
    }

    // 下沉元素
    fn sink(&mut self, mut k: usize) {
        while k * 2 <= self.size() {
            // k 的左元素
            let mut j = k * 2;

            // k 的右元素
            if j + 1 <= self.size() && self.compare(j, j + 1) {
                j += 1;
            }

            if ! self.compare(k, j) {
                break;
            }

            // 交换根节点和子元素
            self.swap(k, j);

            // 对子元素继续处理
            k = j;
        }
    }
}

#[test]
fn test() {
    // 最大索引队列
    let mut pq = IndexBinaryHeap::with_capacity(10);

    pq.put(0, 0.33);
    pq.put(1, 0.0002);
    pq.put(5, 0.001);
    pq.put(8, 0.01);
    pq.put(1, 0.8);

    assert_eq!(pq.pop(), 1);
    assert_eq!(pq.pop(), 0);
    assert_eq!(pq.pop(), 8);
    assert_eq!(pq.pop(), 5);


    // 最小索引队列
    use std::cmp::Ordering;
    use std::f32;

    #[derive(Eq, PartialEq)]
    struct Weight {
        value: u32,
    };

    impl Weight {
        pub fn new(n: f32) -> Self {
            Weight { value: n.to_bits() }
        }

//        pub fn value(&self) -> f32 {
//            f32::from_bits(self.value)
//        }
    }

    impl Ord for Weight {
        fn cmp(&self, other: &Weight) -> Ordering {
            other.value.cmp(&self.value)
        }
    }

    impl PartialOrd for Weight {
        fn partial_cmp(&self, other: &Weight) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }


    let mut pq = IndexBinaryHeap::with_capacity(10);

    pq.put(0, Weight::new(0.33));
    pq.put(1, Weight::new(0.0002));
    pq.put(5, Weight::new(0.001));
    pq.put(8, Weight::new(0.01));
    pq.put(1, Weight::new(0.8));

    assert_eq!(pq.pop(), 5);
    assert_eq!(pq.pop(), 8);
    assert_eq!(pq.pop(), 0);
    assert_eq!(pq.pop(), 1);
}