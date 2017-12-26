fn main() {
//    let adj: [Vec<isize>; 13] = [Vec::new(); 13];

    let n = 13;
    let mut adj : Vec<Vec<isize>> = Vec::with_capacity(n);

    for _ in 0..n {
        adj.push(Vec::new());
    }

    println!("{:?}", adj);
}