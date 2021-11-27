use std::ptr::NonNull;
use std::fmt::Display;

enum Color {
    Red,
    Black,
}

struct RbNode<K, V> 
    where K: PartialOrd,
          V: Display,
{
    key: K,
    value: V,
    color: Color,
    node_count: usize,
    left: Option<NonNull<Self>>,
    right: Option<NonNull<Self>>,
}

impl<K: PartialOrd, V: Display> RbNode<K, V> {
    fn new(key: K, value: V, color: Color) -> Self {
        RbNode {key, vaule, color, node_count: 0, left: None, right: None}
    }
}

fn main() {

}