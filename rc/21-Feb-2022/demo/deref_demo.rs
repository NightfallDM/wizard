// This is the Deref trait demo, but i used to write this code just for 
// research the 'iter' method for array, though the std lib 'array' told
// array coerec to 'slice', and  there have 'iter' method in 'slice' scope.

use std::borrow::Borrow;
use std::ops::Deref;
pub struct Xxx {
    pub val: i32,
}

impl Xxx {
    pub fn pr(&self) {
        println!("val = {}", self.val);
    }
}

impl From<&Yyy> for Xxx {
    fn from(y: &Yyy) -> Self {
        Xxx {val: y.val}
    }
}

struct Yyy {
    val: i32,
}

impl AsRef<Xxx> for Yyy {
    fn as_ref(&self) -> &Xxx {
        //&self.into()
        //self as *const Yyy as *const Xxx as &Xxx
        unsafe {&*(self as *const Yyy as *const Xxx)}
    }
}

impl Borrow<Xxx> for Yyy {
    fn borrow(&self) -> &Xxx {
        unsafe {&*(self as *const Yyy as *const Xxx)}
    }
}

impl Deref for Yyy {
    type Target = Xxx;
    fn deref(&self) -> &Xxx {
        unsafe {&*(self as *const Yyy as *const Xxx)}
    }
}

pub fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    for x in vec.iter() {
        println!("{}", x);
    }
    let x = Xxx {val: 10};
    x.pr();
    let y = Yyy {val: 20};
    y.pr();
}

