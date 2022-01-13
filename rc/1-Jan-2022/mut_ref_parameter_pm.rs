// can not work, because &mut do not impl 'Copy' or 'Clone'
// fn use_mut_ref_g<T>(x: T) {}

// this can work
fn use_mut_ref(_x: &mut String){
    drop(x);
}

// also this example also can work
fn use_mut_ref_other<T>(_x: &mut T) {}

fn main() {
    let mut x = String::from("xxx");
    let x_mut_ref = &mut x;
    // use_mut_ref_g(x_mut_ref);
    use_mut_ref(x_mut_ref);
    use_mut_ref_other(x_mut_ref);
    println!("x = {}", x_mut_ref);
}

// So the important of whether above simple code can work depend on if there the compiler
// can dectect the fn arg if it is type '&' or '&mut' ?

// do it also told us that the compiler can't dectect the first example that the type of x is '&mut String'
// at compile time ? otherwise it can do the same thing just like the second and third example
// and can work, or mean there have a bug?

// and as this point, i know the reason why the second and third example can work, because we already
// mut borrow x, and also this variable do not impl 'Sync' or 'Send', so we just in a single thread env
// and the compiler already make sure we are the only one who mut borrow x. So it can let the fn which take
// '&mut String' as param get a mut borrow through 'x_mut_ref'(which also a mut borrow to x) right?
// At this point there do exist two mut borrow to x, but it won't operation on x at one time, So it is ok.

// By the way ,in '&mut T' case the T match the '&mut String' itself, So the match after is '&mut &mut String' 
