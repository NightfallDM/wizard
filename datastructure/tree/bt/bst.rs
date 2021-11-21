use std::ptr::NonNull;
use std::fmt::Display;

#[derive(Debug)]
struct BstNode<T: Display + Copy, K: PartialOrd + Display + Copy> {
    value: T,
    key: K,
    left: Option<NonNull<BstNode<T, K>>>,
    right: Option<NonNull<BstNode<T, K>>>,
}

impl<T: Display + Copy, K: PartialOrd + Display + Copy> BstNode<T, K> {
    pub fn new(val: T, key: K) -> Self {
        BstNode {value: val, key: key, left: None, right: None}
    }

    pub fn get_val(&self) -> &T {
        &self.value
    }

    pub fn get_key(&self) -> &K {
        &self.key
    }

    pub fn traverse(&self, contain: &mut Vec<NonNull<BstNode<T, K>>>) {
        if let Some(nnptr) = self.left {
            unsafe {
                nnptr.as_ref().traverse(contain);
            }
        }
        println!("key = {}, value = {}", self.key, self.value);
        contain.push(unsafe{
            NonNull::new_unchecked(&self as *const _ as *mut _)
        });
        if let Some(nnptr) = self.right {
            unsafe {
                nnptr.as_ref().traverse(contain);
            }
        }
    }

    pub fn set_left(&mut self, left_child: &BstNode<T, K>) {
        self.left = NonNull::new(left_child as *const _ as *mut _);
    }

    pub fn set_right(&mut self, right_child: &BstNode<T, K>) {
        self.right = NonNull::new(right_child as *const _ as *mut _);
    }

    pub fn get_left(&self) -> Option<NonNull<BstNode<T, K>>> {
        self.left
    }

    pub fn get_right(&self) -> Option<NonNull<BstNode<T, K>>> {
        self.right
    }

    pub fn get_next(&self, contain: &Vec<NonNull<BstNode<T, K>>>) -> NonNull<BstNode<T, K>> {
        let mut idx = 0;

        // if the node is last we won't call it
        // so do not need to check the situation
        for x in contain {
            idx += 1;
            if self.key == unsafe{x.as_ref()}.key {
                break;
            }
        }
        // NonNull impl "Copy" so we can just return the val in Vec
        contain[idx]
    }

    pub fn set_val_key(&mut self, val: &T, key: &K) {
        self.value = *val;
        self.key = *key;
    }

}


struct Bst<T: Display + Copy, K: PartialOrd + Display + Copy> {
    root: Option<NonNull<BstNode<T, K>>>,
}

impl<T: Display + Copy, K: PartialOrd + Display + Copy> Bst<T, K> {
    pub fn new() -> Self {
        Bst {root: None}
    }

    pub fn new_with_node(root_node: &mut BstNode<T, K>) -> Self {
        Bst {root: NonNull::new(root_node as *const _ as *mut _)}
    }

    pub fn traverse(&self, contain: &mut Vec<NonNull<BstNode<T, K>>>) {
        match self.root {
            None => return,
            Some(nnptr) => unsafe {nnptr.as_ref().traverse(contain);},
        }
    }

    pub fn insert(&mut self, ins_node: &mut BstNode<T, K>) {
        match self.root {
            None => self.root = Some(unsafe{NonNull::new_unchecked(ins_node as *mut _)}),

            Some(mut root_node_nnptr) => {
                unsafe{
                    if root_node_nnptr.as_ref().get_key() > ins_node.get_key() {
                        match root_node_nnptr.as_ref().get_left() {
                            None => root_node_nnptr.as_mut().set_left(ins_node),
                            Some(next_node) => {
                                Bst {root: Some(next_node)}.insert(ins_node);
                            },
                        }
                    }else {
                        match root_node_nnptr.as_ref().get_right() {
                            None => root_node_nnptr.as_mut().set_right(ins_node),
                            Some(next_node) => {
                                // next_node.as_mut().insert(ins_node);
                                Bst {root: Some(next_node)}.insert(ins_node);
                            },
                        }
                    }
                }
            }

        }
    }

    pub fn search(&self, key: &K, contain: &mut Vec<NonNull<BstNode<T, K>>>) -> Option<NonNull<BstNode<T, K>>> {
        match self.root {
            None => return None,
            Some(node_nnptr) => {
                contain.push(node_nnptr);
                if unsafe{node_nnptr.as_ref().get_key()} == key {
                    return Some(node_nnptr);
                }else {
                    if unsafe{node_nnptr.as_ref().get_key()} > key {
                        match unsafe{node_nnptr.as_ref().get_left()} {
                            None => return None,
                            Some(next_nnptr) => {
                                Bst {root: Some(next_nnptr)}.search(key, contain)
                            },
                        }
                    }else {
                        match unsafe{node_nnptr.as_ref().get_right()} {
                            None => return None,
                            Some(next_nnptr) => {
                                Bst {root: Some(next_nnptr)}.search(key, contain)
                            },
                        }
                    }
                }
            }
        }
    }

    // ret the nnptr point to the real BstNode
    pub fn delete(&mut self, key: &K) -> Option<NonNull<BstNode<T, K>>> {
        let mut contain = Vec::with_capacity(16);
        if let Some(del_nnptr) = self.search(key, &mut contain) {
            let mut idx = 0;
            let length = contain.len();
            let contain_clone = contain.clone();
            for elem in contain_clone.iter().rev() {
                if unsafe{elem.as_ref()}.get_right() == None && unsafe{elem.as_ref()}.get_left() == None {
                    unsafe{contain[length - 1].as_mut()}.set_val_key(unsafe{elem.as_ref()}.get_val(), unsafe{elem.as_ref()}.get_key());
                    break;
                    // return Some(*elem);
                }
                idx += 1;
            }
            println!("{}", idx);
            Some(contain[contain.len() - idx - 1])
            // Some(contain[contain.len() - 2])
        }else {
            None
        }
    }

}

fn main() {
    let mut node1 = BstNode::new(6, 6);
    let mut bs = Bst::new_with_node(&mut node1);
    let mut node2 = BstNode::new(12, 12);
    bs.insert(&mut node2);
    let mut node3 = BstNode::new(3, 3);
    bs.insert(&mut node3);
    let mut node4 = BstNode::new(1, 1);
    let mut node5 = BstNode::new(5, 5);
    let mut node6 = BstNode::new(10, 10);
    let mut node7 = BstNode::new(14, 14);
    bs.insert(&mut node4);
    bs.insert(&mut node5);
    bs.insert(&mut node6);
    bs.insert(&mut node7);
    let mut contain = Vec::with_capacity(16);
    bs.traverse(&mut contain);
    println!("{:?}", contain);
    bs.delete(&6);
    bs.traverse(&mut contain);
    println!("{:?}", contain);
}