use std::ptr::NonNull;
use std::fmt::Display;

#[derive(Copy, Clone, PartialEq)]
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

    fn is_red(node_nnptr_opt: Option<NonNull<Self>>) -> bool {
        match node_nnptr_opt {
            None => false,
            Some(node_nnptr) => {
                RbNode::nn_to_ref(node_nnptr).color == Color::Red
            },
        }
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

    fn _put(node_nnptr_opt: Option<NonNull<RbNode<K, V>>>, key: K, value: V) -> NonNull<RbNode<K, V>> {
        if let None = node_nnptr_opt {
            return unsafe{NonNull::new_unchecked(RbNode::new_box_leak(key, value, Color::Red) as *mut _)};
        }

        let mut curr = RbNode::nn_to_mut(node_nnptr_opt.unwrap());
        if curr.key > key {
            curr.left = Some(RbTree::_put(curr.left, key, value));
        }else if curr.key < key {
            curr.right = Some(RbTree::_put(curr.right, key, value));
        }else {
            curr.key = key;
            curr.value = value;
        }
        
        if RbNode::is_red(curr.right) && !RbNode::is_red(curr.left){
            curr = RbNode::nn_to_mut(RbTree::_totate_left(unsafe{NonNull::new_unchecked(curr as *mut _)}));
        }

        if RbNode::is_red(curr.left) && RbNode::is_red(RbNode::nn_to_ref(curr.left.unwrap()).left) {
            curr = RbNode::nn_to_mut(RbTree::_totate_right(curr.left.unwrap()));
        }

        if RbNode::is_red(curr.right) && RbNode::is_red(curr.left) {
            RbTree::_flip_color(unsafe{NonNull::new_unchecked(curr as *mut _)});
        }

        unsafe{NonNull::new_unchecked(curr as *mut _)}
    }

    pub fn put(&mut self, key: K, value: V) {
        self.root = Some(Self::_put(self.root, key, value));
        RbNode::nn_to_mut(self.root.unwrap()).color = Color::Red;
    }

    // use in delete
    fn flip_color_delete(mut node_nnptr: NonNull<RbNode<K, V>>) {
        let curr = RbNode::nn_to_mut(node_nnptr);
        curr.color = Color::Black;
        RbNode::nn_to_mut(curr.left.unwrap()).color = Color::Red;
        RbNode::nn_to_mut(curr.right.unwrap()).color = Color::Red;
    }

    // input Node and make node.left not be the 2 node.
    fn move_red_left(mut node_nnptr: NonNull<RbNode<K, V>>) -> NonNull<RbNode<K, V>> {
        RbTree::flip_color_delete(node_nnptr);
        if RbNode::is_red(RbNode::nn_to_ref(RbNode::nn_to_ref(node_nnptr).right.unwrap()).left) {
            RbNode::nn_to_mut(node_nnptr).right = Some(RbTree::_totate_right(RbNode::nn_to_mut(node_nnptr).right.unwrap()));
            node_nnptr = RbTree::_totate_left(node_nnptr);
        }
        node_nnptr
    }

    fn blance(mut node_nnptr: NonNull<RbNode<K, V>>) -> NonNull<RbNode<K, V>>{
        let mut curr = RbNode::nn_to_mut(node_nnptr);

        if RbNode::is_red(curr.right) {
            node_nnptr = RbTree::_totate_left(node_nnptr);
        }

        if RbNode::is_red(curr.right) && !RbNode::is_red(curr.left){
            curr = RbNode::nn_to_mut(RbTree::_totate_left(unsafe{NonNull::new_unchecked(curr as *mut _)}));
        }

        if RbNode::is_red(curr.left) && RbNode::is_red(RbNode::nn_to_ref(curr.left.unwrap()).left) {
            curr = RbNode::nn_to_mut(RbTree::_totate_right(curr.left.unwrap()));
        }

        if RbNode::is_red(curr.right) && RbNode::is_red(curr.left) {
            RbTree::_flip_color(unsafe{NonNull::new_unchecked(curr as *mut _)});
        }

        unsafe{NonNull::new_unchecked(curr as *mut _)}

    }

    fn _delete_min(mut node_nnptr: NonNull<RbNode<K, V>>) -> Option<NonNull<RbNode<K, V>>> {
        if let None = RbNode::nn_to_ref(node_nnptr).left {
            return None;
        }

        if !RbNode::is_red(RbNode::nn_to_ref(node_nnptr).left) && !RbNode::is_red(RbNode::nn_to_ref(RbNode::nn_to_ref(node_nnptr).left.unwrap()).left) {
            node_nnptr = RbTree::move_red_left(node_nnptr);
        }

        RbNode::nn_to_mut(node_nnptr).left = RbTree::_delete_min(RbNode::nn_to_mut(node_nnptr).left.unwrap());
        Some(RbTree::blance(node_nnptr))
    }

    pub fn delete_min(&mut self) {
        match self.root {
            None => {},
            Some(mut node_nnptr) => {
                self.root = RbTree::_delete_min(node_nnptr);
            }
        }
    }

    fn _delete(){}
}

fn main() {

}