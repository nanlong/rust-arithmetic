pub fn select_sort<T, F: Fn(&T, &T) -> bool>(seq: &mut [T], compare: &F) {
    let len = seq.len();
    let mut i = 0;

    while i < len {
        let mut min = i;
        let mut j = i + 1;

        while j < len {
            if compare(&seq[j], &seq[min]) {
                min = j;
            }
            j += 1;
        }
        
        seq.swap(i, min);
        i += 1;
    }
}


#[test]
fn test() {
    // 从大到小排序数字
    let mut seq = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    select_sort(&mut seq, &|x, y| x > y);
    assert_eq!(seq, [782, 99, 83, 65, 4, 2, 2, 1, 0, -31]);

    // 按字母顺序排序字符串
    let mut seq = ["beach", "hotel", "airplane", "car", "house", "art"];
    select_sort(&mut seq, &|x, y| x < y);
    assert_eq!(seq, ["airplane", "art", "beach", "car", "hotel", "house"]);

    // 按长度排序字符串
    select_sort(&mut seq, &|x, y| x.len() < y.len());
    assert_eq!(seq, ["art", "car", "beach", "hotel", "house", "airplane"]);
}