pub fn merge_sort<T: Copy, F: Fn(&T, &T) -> bool>(seq: &mut [T], compare: &F) {
    let n = seq.len();
    let m = n / 2;

    if n <= 1 {
        return
    }

    merge_sort(&mut seq[..m], compare);
    merge_sort(&mut seq[m..], compare);

    let mut y = seq.to_vec();
    merge(&seq[..m], &seq[m..], &mut y, compare);
    seq.copy_from_slice(&y);
}


fn merge<T: Copy, F: Fn(&T, &T) -> bool>(x1: &[T], x2: &[T], y: &mut [T], f: &F) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < x1.len() && j < x2.len() {
        if f(&x1[i], &x2[j]) {
            y[k] = x1[i];
            i += 1;
        }
        else {
            y[k] = x2[j];
            j += 1;
        }
        k += 1;
    }

    if i < x1.len() {
        y[k..].copy_from_slice(&x1[i..]);
    }

    if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }
}


#[test]
fn test() {
    // 从大到小排序数字
    let mut seq = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    merge_sort(&mut seq, &|x, y| x > y);
    assert_eq!(seq, [782, 99, 83, 65, 4, 2, 2, 1, 0, -31]);

    // 按字母顺序排序字符串
    let mut seq = ["beach", "hotel", "airplane", "car", "house", "art"];
    merge_sort(&mut seq, &|x, y| x < y);
    assert_eq!(seq, ["airplane", "art", "beach", "car", "hotel", "house"]);

    // 按长度排序字符串
    merge_sort(&mut seq, &|x, y| x.len() < y.len());
    assert_eq!(seq, ["car", "art", "house", "hotel", "beach", "airplane"]);
}