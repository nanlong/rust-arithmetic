extern crate arithmetic;

use arithmetic::rbt::tree::RBT;

fn main() {
    let mut rbt = RBT::<&str, isize>::new();

    rbt.put("S", 1);
    rbt.put("E", 2);
    rbt.put("X", 3);
    rbt.put("A", 4);
    rbt.put("R", 5);
    rbt.put("C", 6);
    rbt.put("H", 7);
    rbt.put("M", 8);

//    println!("{:#?}", rbt);

//    rbt.delete_min();
//    println!("{:#?}", rbt);

//    rbt.delete_max();
//    println!("{:#?}", rbt);

    rbt.delete("S");
    println!("{:#?}", rbt);
}