use std::ptr::NonNull;
use std::fmt::Display;

enum Color {
    Red,
    Black,
}

struct RbNode<T, K> 
    where T: Display,
          K: PartialOrd,
{
    value: T,
    key: K,
    color: Color,
    node_count: usize,
    left: NonNull<Self>,
    right: NonNull<Self>,
}

fn main() {

}