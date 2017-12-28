pub fn heap_sort<T, F: Fn(&T, &T) -> bool>(seq: &mut [T], compare: &F) {
    let len = seq.len();

    if len > 1 {
        for start in (0..len / 2).rev() {
            sift_down(seq, start, len - 1, compare);
        }

        for finish in (1..len).rev() {
            seq.swap(0, finish);
            sift_down(seq, 0, finish - 1, compare);
        }
    }
}

fn sift_down<T, F: Fn(&T, &T) -> bool>(seq: &mut [T], root: usize, finish: usize, compare: &F) {
    let mut start = root;

    while start < finish {
        let mut child = start * 2 + 1;

        if child > finish {
            break
        }

        if child + 1 <= finish && compare(&seq[child], &seq[child + 1]) {
            child += 1;
        }

        if compare(&seq[start], &seq[child]) {
            seq.swap(start, child);
            start = child;
        }
        else {
            break
        }
    }
}


#[test]
fn test() {
    // 从大到小排序数字
    let mut seq = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    heap_sort(&mut seq, &|x, y| x > y);
    assert_eq!(seq, [782, 99, 83, 65, 4, 2, 2, 1, 0, -31]);

    // 按字母顺序排序字符串
    let mut seq = ["beach", "hotel", "airplane", "car", "house", "art"];
    heap_sort(&mut seq, &|x, y| x < y);
    assert_eq!(seq, ["airplane", "art", "beach", "car", "hotel", "house"]);

    // 按长度排序字符串
    heap_sort(&mut seq, &|x, y| x.len() < y.len());
    assert_eq!(seq, ["art", "car", "beach", "hotel", "house", "airplane"]);
}