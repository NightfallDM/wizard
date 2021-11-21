use std::fmt::Display;
use std::ptr::NonNull;

#[derive(Debug)]
struct Xxx<T: Display> {
    val: T,
}

impl<T: Display> Xxx<T> {
    pub fn new(val: T) ->Self {
        Xxx {val: val}
    }
    
    pub fn test_lt<'a>(&'a self, contain: &mut Vec<&'a Xxx<T>>) {
        contain.push(self);
    }
    
    pub fn test_mut(&mut self) {
        
    }
    pub fn test_lt_nnptr(&self, contain: &mut Vec<NonNull<Xxx<T>>>) {
        contain.push(unsafe{
            NonNull::new_unchecked(&self as *const _ as *mut _)
        });
    }
}

fn test_da(refdouble_xxx: &i32) {
    println!("{}", refdouble_xxx);
}

fn main() {
    let mut vec1 = Vec::with_capacity(10);
    println!("vec1 = {:?}", vec1);
    let mut x1 = Xxx::new(100);
    // x1.test_lt(&mut vec1);
    x1.test_lt_nnptr(&mut vec1);
    x1.test_mut();
    println!("vec1 = {:?}", vec1);
    println!("Hello, world!");
    
    let xxx = 100;
    let xxx_ref = &xxx;
    let xxx_double_ref = &xxx_ref;
    test_da(xxx_double_ref);
}