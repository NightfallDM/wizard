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
    fn new(val: T, key: K) -> Self {
        BstNode {value: val, key: key, left: None, right: None}
    }

    fn new_box(val: T, key: K) -> Box<Self> {
        Box::new(BstNode::new(val, key))
    }

    fn new_box_leak<'a>(val: T, key: K) -> &'a mut Self {
        Box::leak(Self::new_box(val, key))
    }

    fn nn_to_node_ref<'a>(node_nnptr: NonNull<BstNode<T, K>>) -> &'a Self {
        unsafe{node_nnptr.as_ref()}
    }

    fn nn_to_node_mut<'a>(mut node_nnptr: NonNull<Self>) -> &'a mut Self {
        unsafe{node_nnptr.as_mut()}
    }

    pub fn get_val(&self) -> &T {
        &self.value
    }

    pub fn get_key(&self) -> &K {
        &self.key
    }

    pub fn traverse_with_vec(&self, contain: &mut Vec<NonNull<BstNode<T, K>>>) {
        if let Some(nnptr) = self.left {
            unsafe {
                nnptr.as_ref().traverse_with_vec(contain);
            }
        }
        println!("key = {}, value = {}", self.key, self.value);
        contain.push(unsafe{
            NonNull::new_unchecked(self as *const _ as *mut _)
        });
        if let Some(nnptr) = self.right {
            unsafe {
                nnptr.as_ref().traverse_with_vec(contain);
            }
        }
    }

    pub fn traverse(&self) {
        if let Some(left_nnptr) = self.left {
            BstNode::nn_to_node_ref(left_nnptr).traverse();
        }
        println!("key = {}, value = {}", self.key, self.value);
        if let Some(right_nnptr) = self.right {
            BstNode::nn_to_node_ref(right_nnptr).traverse();
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

    pub fn insert(&mut self, val: T, key: K) {
        if key < self.key {
            match self.left {
                None => self.left = Some(BstNode::new_box_leak(val, key).into()),
                Some(mut next_nnptr) => BstNode::nn_to_node_mut(next_nnptr).insert(val, key),
            }
        }else {
            match self.right {
                None => self.right = Some(BstNode::new_box_leak(val, key).into()),
                Some(mut next_nnptr) => BstNode::nn_to_node_mut(next_nnptr).insert(val, key),
            }
        }
    }

    pub fn min(&self) -> NonNull<Self> {
        if let Some(next_ndnnptr) = self.left {
            BstNode::nn_to_node_ref(next_ndnnptr).min()
        }else {
            unsafe{NonNull::new_unchecked(self as *const _ as *mut _)}
        }
    }

    pub fn max(&self) -> NonNull<Self> {
        if let Some(next_ndnnptr) = self.right {
            BstNode::nn_to_node_ref(next_ndnnptr).max()
        }else {
            unsafe{NonNull::new_unchecked(self as *const _ as *mut _)}
        }
    }

    pub fn search(&self, key: &K) -> Option<NonNull<Self>> {
        if *key == self.key {
            return Some(unsafe{NonNull::new_unchecked(self as *const _ as *mut _)});
        }

        if *key < self.key {
            match self.left {
                None => return None,
                Some(next_nnptr) => BstNode::nn_to_node_ref(next_nnptr).search(key),
            }
        }else {
            match self.right {
                None => None,
                Some(next_nnptr) => BstNode::nn_to_node_ref(next_nnptr).search(key),
            }
        }
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

    pub fn traverse_with_vec(&self, contain: &mut Vec<NonNull<BstNode<T, K>>>) {
        match self.root {
            None => return,
            Some(nnptr) => {BstNode::nn_to_node_ref(nnptr).traverse_with_vec(contain);},
        }
    }

    pub fn traverse(&self) {
        match self.root {
            None => {},
            Some(node_nnptr) => {
                BstNode::nn_to_node_ref(node_nnptr).traverse();
            },
        }
    }

    pub fn insert(&mut self, val: T, key: K) {
        match self.root {
            None => self.root = Some(BstNode::new_box_leak(val, key).into()),
            Some(mut root_node_nnptr) => {
                BstNode::nn_to_node_mut(root_node_nnptr).insert(val, key);
            },
        }
    }

    pub fn search_with_vec(&self, key: &K, contain: &mut Vec<NonNull<BstNode<T, K>>>) -> Option<NonNull<BstNode<T, K>>> {
        match self.root {
            None => return None,
            Some(node_nnptr) => {
                contain.push(node_nnptr);
                if BstNode::nn_to_node_ref(node_nnptr).key == *key {
                    return Some(node_nnptr);
                }else {
                    if BstNode::nn_to_node_ref(node_nnptr).key > *key {
                        match BstNode::nn_to_node_ref(node_nnptr).get_left() {
                            None => return None,
                            Some(next_nnptr) => {
                                Bst {root: Some(next_nnptr)}.search_with_vec(key, contain)
                            },
                        }
                    }else {
                        match BstNode::nn_to_node_ref(node_nnptr).get_right() {
                            None => return None,
                            Some(next_nnptr) => {
                                Bst {root: Some(next_nnptr)}.search_with_vec(key, contain)
                            },
                        }
                    }
                }
            }
        }
    }

    pub fn search(&self, key: &K) -> Option<NonNull<BstNode<T, K>>> {
        match self.root {
            None => None,
            Some(node_nnptr) => {
                BstNode::nn_to_node_ref(node_nnptr).search(key)
            }
        }
    }


    fn _delete_min(node_nnptr: &mut NonNull<BstNode<T, K>>) -> Option<NonNull<BstNode<T, K>>> {
        // match unsafe{node_nnptr.as_ref()}.left {
        //     None => {
        //         let ret_node = unsafe{node_nnptr.as_ref()}.right;
        //         {
        //             unsafe{Box::from_raw(node_nnptr.as_ptr());}
        //         }
        //         ret_node
        //     },
        //     Some(mut next_node_nnptr) => {
        //         Bst::_delete_min(&mut next_node_nnptr)
        //     },
        // }
        if let None = BstNode::nn_to_node_ref(*node_nnptr).left {
            let ret_node = BstNode::nn_to_node_ref(*node_nnptr).right;
            {
                // free this node
                unsafe{Box::from_raw(node_nnptr.as_ptr());}
            }
            return ret_node;
        }

        BstNode::nn_to_node_mut(*node_nnptr).left = Bst::_delete_min(&mut BstNode::nn_to_node_mut(*node_nnptr).left.unwrap());
        Some(*node_nnptr)

    }

    pub fn delete_min(&mut self) {
        if let Some(mut node_nnptr) = self.root {
            self.root = Bst::_delete_min(&mut node_nnptr);
        }
    }

    fn _min(node_nnptr: NonNull<BstNode<T, K>>) -> NonNull<BstNode<T, K>> {
        BstNode::nn_to_node_ref(node_nnptr).min()
    }

    pub fn min(&self) -> Option<NonNull<BstNode<T, K>>> {
        match self.root {
            None => None,
            Some(node_nnptr) => {
                Some(Bst::_min(node_nnptr))
            },
        }
    }

    fn _delete(mut node_nnptr_opt: Option<NonNull<BstNode<T, K>>>, key: &K) -> Option<NonNull<BstNode<T, K>>> {
        if let None = node_nnptr_opt {
            return None;
        }

        if BstNode::nn_to_node_ref(node_nnptr_opt.unwrap()).key > *key {
            Bst::_delete(BstNode::nn_to_node_ref(node_nnptr_opt.unwrap()).left, key)
        }else if BstNode::nn_to_node_ref(node_nnptr_opt.unwrap()).key < *key {
            Bst::_delete(BstNode::nn_to_node_ref(node_nnptr_opt.unwrap()).right, key)
        }else {
            if let None = BstNode::nn_to_node_ref(node_nnptr_opt.unwrap()).left {
                return BstNode::nn_to_node_ref(node_nnptr_opt.unwrap()).right;
            }

            if let None = BstNode::nn_to_node_ref(node_nnptr_opt.unwrap()).right {
                return BstNode::nn_to_node_ref(node_nnptr_opt.unwrap()).left;
            }
            
            let del_after_min_nnptr = Bst::_min(BstNode::nn_to_node_ref(node_nnptr_opt.unwrap()).right.unwrap());
            BstNode::nn_to_node_mut(node_nnptr_opt.unwrap()).key = BstNode::nn_to_node_ref(del_after_min_nnptr).key;
            BstNode::nn_to_node_mut(node_nnptr_opt.unwrap()).value = BstNode::nn_to_node_ref(del_after_min_nnptr).value;
            BstNode::nn_to_node_mut(node_nnptr_opt.unwrap()).right = Bst::_delete_min(&mut BstNode::nn_to_node_mut(node_nnptr_opt.unwrap()).right.unwrap());
            node_nnptr_opt
        }
    }

    pub fn delete(&mut self, key: &K) {
        self.root = Bst::_delete(self.root, key);
    }

}

fn main() {
    // let mut node1 = BstNode::new(6, 6);
    // let mut bs = Bst::new_with_node(&mut node1);
    // let mut node2 = BstNode::new(12, 12);
    // bs.insert(&mut node2);
    // let mut node3 = BstNode::new(3, 3);
    // bs.insert(&mut node3);
    // let mut node4 = BstNode::new(1, 1);
    // let mut node5 = BstNode::new(5, 5);
    // let mut node6 = BstNode::new(10, 10);
    // let mut node7 = BstNode::new(14, 14);
    // bs.insert(&mut node4);
    // bs.insert(&mut node5);
    // bs.insert(&mut node6);
    // bs.insert(&mut node7);
    // let mut contain = Vec::with_capacity(16);
    // bs.traverse(&mut contain);
    // println!("{:?}", contain);
    // bs.delete(&6);
    // bs.traverse(&mut contain);
    // println!("{:?}", contain);

    let mut bs = Bst::new();
    bs.insert(6, 6);
    bs.insert(12, 12);
    bs.insert(3, 3);
    bs.insert(1, 1);
    bs.insert(5, 5);
    bs.insert(10, 10);
    bs.insert(14, 14);
    bs.delete_min();
    bs.delete_min();
    bs.traverse();
    println!("first traverse");
    bs.delete(&6);
    bs.traverse();
}