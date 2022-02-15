use std::mem::{self, ManuallyDrop};
use std::ptr;

// test code here so just put the 'Vec' as the parameter
// TODO: modify replacement need
fn quick_sort<T>(vec: &mut [T], lo: usize, hi: usize) 
	where T: PartialOrd,
{
	if hi == lo {
		return;
	}
	let (sp_idx0, sp_idx1) = partition(vec, lo, hi);
	if sp_idx0 != lo {
		quick_sort(vec, lo, sp_idx0 - 1);
	}
	if sp_idx1 != hi {
		quick_sort(vec, sp_idx1 + 1, hi);
	}
}

fn partition<T>(vec: &mut [T], lo: usize, hi: usize) -> (usize, usize)
	where T: PartialOrd,
{
	let mut lt = lo;
	let mut gt = hi;
	let mut i = lo + 1;
	let v = ManuallyDrop::new(unsafe {ptr::read(&vec[lo])});
	while i <= gt {
		if vec[i] < *v {
			swap(vec, i, lt);
			i += 1;
			lt += 1;
		}else if vec[i] > *v {
			swap(vec, i, gt);
			gt -= 1;
		}else {
			i += 1;
		}
	}
	(lt, gt)
}

fn swap<T>(vec: &mut [T], idx1: usize, idx2: usize)
	where T: PartialOrd,
{
	if idx1 == idx2 {
		return;
	}
	let mut tmp = unsafe { ptr::read(&mut vec[idx1]) };
	mem::swap(&mut tmp, &mut vec[idx2]);
    unsafe {
        ptr::write(&mut vec[idx1] as *mut _, tmp);
    }
/*
	if idx1 == idx2 {
		return;
	}
	let tmp = &vec[idx1];
	vec[idx1] = vec[idx2];
	vec[idx2] = *tmp;
*/
}

fn main() {
	let mut vec = vec![10, 4, 20, 11, 5, 6, 22, 50, 43, 25, 33, 13, 26, 27, 43, 47, 48];
	let len = vec.len();
	quick_sort(&mut vec[..], 0, len - 1);
	println!("{:?}", vec);
	println!("hello night!");
}
