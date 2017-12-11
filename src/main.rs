extern crate arithmetic;

use arithmetic::rbt::tree::RBT;

fn main() {
    let mut rbt = RBT::<&str, isize>::new();

    rbt.put("C", 3);
    rbt.put("B", 2);
    rbt.put("A", 1);
    rbt.put("D", 4);

    println!("{:#?}", rbt);

//    node.rotate_right();
//    println!("{:#?}", node);
}