extern crate arithmetic;

use arithmetic::rbt::tree::RBT;

fn main() {
    let mut rbt = RBT::<&str, isize>::new();

    rbt.put("C", 3);
    rbt.put("B", 2);
    rbt.put("A", 1);
    rbt.put("D", 4);
    rbt.put("E", 5);
    rbt.put("F", 6);

    println!("{:#?}", rbt);
}