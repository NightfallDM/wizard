use std::ptr::NonNull;
use std::mem::MaybeUninit;

const B: usize = 8;
pub const CAPACITY_KEY: usize = 2 * B - 1;
pub const MIN_KEY_CNT: usize = B - 1;

struct BPlusNode<K>
    where K: PartialOrd,
{
    keys: [MaybeUninit<K>; 2 * B],
    len: usize,
}

struct BPlusLeafNode<K, V>
    where K: PartialOrd,
{
    keys: [MaybeUninit<K>; 2 * B],
    values: [MaybeUninit<V>; 2 * B],
    len: usize,
}

enum BPlusInterOrLeafNode<K: PartialOrd, V> {
    internal(BPlusNode<K>),
    leaf(BPlusLeafNode<K, V>),
}

struct BPlusTree<K, V>
    where K: PartialOrd,
{
    root: Option<NonNull<BPlusInterOrLeafNode<K, V>>>,
}

fn main() {

}