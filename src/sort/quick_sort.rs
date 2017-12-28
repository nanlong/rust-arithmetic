pub fn quick_sort<T, F: Fn(&T, &T) -> bool>(seq: &mut [T], compare: &F) {
    let len = seq.len();

    if len > 1 {
        let pivot_index = partition(seq, compare);
        quick_sort(&mut seq[..pivot_index], compare);
        quick_sort(&mut seq[pivot_index + 1..], compare);
    }
}

fn partition<T, F: Fn(&T, &T)  -> bool>(seq: &mut [T], compare: &F) -> usize {
    let len = seq.len();
    let mut store_index = 0;

    for i in 0..len - 1 {
        if compare(&seq[i], &seq[len - 1]) {
            seq.swap(i, store_index);
            store_index += 1;
        }
    }

    seq.swap(store_index, len - 1);
    store_index
}

#[test]
fn test() {
    // 从大到小排序数字
    let mut seq = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    quick_sort(&mut seq, &|x, y| x > y);
    assert_eq!(seq, [782, 99, 83, 65, 4, 2, 2, 1, 0, -31]);

    // 按字母顺序排序字符串
    let mut seq = ["beach", "hotel", "airplane", "car", "house", "art"];
    quick_sort(&mut seq, &|x, y| x < y);
    assert_eq!(seq, ["airplane", "art", "beach", "car", "hotel", "house"]);

    // 按长度排序字符串
    quick_sort(&mut seq, &|x, y| x.len() < y.len());
    assert_eq!(seq, ["car", "art", "house", "beach", "hotel", "airplane"]);
}