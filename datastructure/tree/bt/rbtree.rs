use std::ptr::NonNull;
use std::fmt::Display;

#[derive(Copy, Clone)]
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
        RbNode {key, value, color, node_count: 0, left: None, right: None}
    }

    fn new_box(key: K, value: V, color: Color) -> Box<Self> {
        Box::new(RbNode::new(key, value, color))
    }

    fn new_box_leak<'a>(key: K, value: V, color: Color) -> &'a mut Self {
        Box::leak(RbNode::new_box(key, value, color))
    }

    fn nn_to_ref<'a>(node_nnptr: NonNull<Self>) -> &'a Self {
        unsafe{node_nnptr.as_ref()}
    }

    fn nn_to_mut<'a>(mut node_nnptr: NonNull<Self>) -> &'a mut Self {
        unsafe{node_nnptr.as_mut()}
    }
}

struct RbTree<K, V>
    where K: PartialOrd,
          V: Display,
{
    root: Option<NonNull<RbNode<K, V>>>,
}

impl<K: PartialOrd, V: Display> RbTree<K, V> {
    pub fn new() -> Self {
        RbTree{root: None}
    }

    // we do not check the child here, so it's the caller responsibility to call this in the correct way
    fn _totate_left(mut node_nnptr: NonNull<RbNode<K ,V>>) -> NonNull<RbNode<K, V>> {
        let ret_node = RbNode::nn_to_ref(node_nnptr).right.unwrap();
        RbNode::nn_to_mut(ret_node).color = RbNode::nn_to_ref(node_nnptr).color;
        RbNode::nn_to_mut(node_nnptr).right = RbNode::nn_to_ref(ret_node).left;
        RbNode::nn_to_mut(node_nnptr).color = Color::Red;
        RbNode::nn_to_mut(ret_node).left = Some(node_nnptr);
        ret_node
    }

    pub fn totate_left(&mut self) {
        match self.root {
            None => {},
            Some(node_nnptr) => {self.root = Some(RbTree::_totate_left(node_nnptr));},
        }
    }

    // do not check
    fn _totate_right(mut node_nnptr: NonNull<RbNode<K ,V>>) -> NonNull<RbNode<K ,V>> {
        let ret_node = RbNode::nn_to_ref(node_nnptr).left.unwrap();
        RbNode::nn_to_mut(ret_node).color = RbNode::nn_to_ref(node_nnptr).color;
        RbNode::nn_to_mut(node_nnptr).left = RbNode::nn_to_ref(ret_node).right;
        RbNode::nn_to_mut(node_nnptr).color = Color::Red;
        ret_node
    }

    pub fn totate_right(&mut self) {
        match self.root {
            None => {},
            Some(node_nnptr) => {
                self.root = Some(RbTree::_totate_right(node_nnptr));
            },
        }
    }

    // do not check
    fn _flip_color(node_nnptr: NonNull<RbNode<K, V>>) {
        let curr_node = RbNode::nn_to_mut(node_nnptr);
        curr_node.color = Color::Red;
        RbNode::nn_to_mut(curr_node.left.unwrap()).color = Color::Black;
        RbNode::nn_to_mut(curr_node.right.unwrap()).color = Color::Black;
    }
}

fn main() {

}