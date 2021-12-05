use std::ptr::{self, NonNull};
use std::mem::{self, MaybeUninit};
use std::alloc::{self, Layout};

const B: usize = 8;
pub const CAPACITY_KEY: usize = 2 * B - 1;
pub const MIN_KEY_CNT: usize = B - 1;

struct FullError;

struct BPlusNode<K, V>
    where K: PartialOrd,
{
    keys: [MaybeUninit<K>; 2 * B],
    childs: [MaybeUninit<Option<NonNull<BPlusInterOrLeafNode<K, V>>>>; 2 * B],
    len: usize,
}

impl<K: PartialOrd, V> BPlusNode<K, V> {
    fn init(this: *mut Self) {
        unsafe {
            ptr::addr_of_mut!((*this).len).write(0);
        }
    }

    fn new_ptr() -> *mut Self {
        let bpn_size = mem::size_of::<BPlusNode<K, V>>();
        let bpn_align = mem::align_of::<BPlusNode<K, V>>();
        let mut bpn_layout = unsafe{Layout::from_size_align_unchecked(bpn_size, bpn_align)};
        bpn_layout = bpn_layout.pad_to_align();
        let ret = unsafe{alloc::alloc(bpn_layout) as *mut Self};
        BPlusNode::init(ret);
        ret
    }

    fn new_nnptr() -> NonNull<BPlusNode<K, V>> {
        unsafe{NonNull::new_unchecked(BPlusNode::new_ptr())}
    }
}

struct BPlusLeafNode<K, V>
    where K: PartialOrd,
{
    keys: [MaybeUninit<K>; 2 * B],
    values: [MaybeUninit<V>; 2 * B],
    next: Option<NonNull<BPlusLeafNode<K, V>>>,
    prev: Option<NonNull<BPlusLeafNode<K, V>>>,
    len: usize,
}

impl<K: PartialOrd, V> BPlusLeafNode<K, V> {
    fn init(this: *mut Self) {
        unsafe{
            ptr::addr_of_mut!((*this).len).write(0);
            ptr::addr_of_mut!((*this).next).write(None);
            ptr::addr_of_mut!((*this).prev).write(None);
        }
    }

    fn new_ptr() -> *mut Self {
        let bpln_size = mem::size_of::<BPlusNode<K, V>>();
        let bpln_align = mem::align_of::<BPlusNode<K, V>>();
        let mut bpln_layout = unsafe{Layout::from_size_align_unchecked(bpln_size, bpln_align)};
        bpln_layout = bpln_layout.pad_to_align();
        let ret = unsafe{alloc::alloc(bpln_layout) as *mut Self};
        BPlusLeafNode::init(ret);
        ret
    }

    fn new_nnptr() -> NonNull<BPlusLeafNode<K, V>> {
        unsafe{NonNull::new_unchecked(BPlusLeafNode::new_ptr())}
    }

    fn insert(&mut self, key: K, value: V) -> Result<(), FullError> {
        if self.len == 2 * B {
            return Err(FullError{});
        }else {
            unsafe{
                (ptr::addr_of_mut!((*self).keys) as *mut K).add(self.len).write(key);
                (ptr::addr_of_mut!((*self).values) as *mut V).add(self.len).write(value);
            }
            self.len += 1;
            Ok(())
        }
    }
}

enum BPlusInterOrLeafNode<K: PartialOrd, V> {
    Internal(BPlusNode<K, V>),
    Leaf(BPlusLeafNode<K, V>),
}

struct BPlusTree<K, V>
    where K: PartialOrd,
{
    root: Option<NonNull<BPlusInterOrLeafNode<K, V>>>,
}

impl<K: PartialOrd, V> BPlusTree<K, V> {
    pub fn new() -> Self {
        BPlusTree{root: None}
    }

    fn _insert(mut node_nnptr: NonNull<BPlusInterOrLeafNode<K, V>>, key: K, value: V) -> NonNull<BPlusInterOrLeafNode<K, V>> {
        // TODO
        node_nnptr
    }

    pub fn insert(&mut self, key: K, value: V) {
        match self.root {
            None => {
                let node_nnptr = BPlusLeafNode::new_nnptr();
                nn_to_mut(node_nnptr).insert(key, value);
            },
            Some(mut node_nnptr) => {
                BPlusTree::_insert(node_nnptr, key, value);
            },
        }
    }
}

/// just a warp to let us dont write too many unsafe in the code
fn nn_to_ref<'a, T>(nn_ptr: NonNull<T>) -> &'a T {
    unsafe{nn_ptr.as_ref()}
}

fn nn_to_mut<'a, T>(mut nn_ptr: NonNull<T>) -> &'a mut T {
    unsafe{nn_ptr.as_mut()}
}
/// end

fn main() {

}