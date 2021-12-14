use slab::Slab;

fn main() {
    let mut xxx = Slab::with_capacity(12);
    let x1 = xxx.insert(1);
    println!("x1 = {}", x1);
    let x2 = xxx.insert(2);
    let x3 = xxx.insert(3);
    let x4 = xxx.insert(4);
    let val_4 = xxx.remove(x4);
    assert_eq!(4, val_4);
    let x5 = xxx.insert(5);
    println!("x5 = {}", x5);
    //println!("x1 = {}", x1);
    println!("Hello, world!");
}
