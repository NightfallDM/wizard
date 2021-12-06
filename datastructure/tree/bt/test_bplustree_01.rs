use std::ptr::NonNull;

fn nn_to_ref<'a, T>(nn_ptr: NonNull<T>) -> &'a T {
    unsafe {nn_ptr.as_ref()}
}

fn nn_to_mut<'a, T>(mut nn_ptr: NonNull<T>) -> &'a mut T {
    unsafe {nn_ptr.as_mut()}
}

struct Xxx {
    value: i32,
    x_str: String,
}

struct Yyy {
    value: i32,
}

enum Zzz {
    X(Xxx),
    Y(Yyy),
}

fn main() {
    let x1 = Xxx {value: 100, x_str: "xxx".to_string()};
    let y1 = Yyy {value: 200};
    let mut z1 = Zzz::X(x1);
    let z1_nnptr = unsafe {NonNull::new_unchecked(&mut z1)};
    
    match nn_to_mut(z1_nnptr) {
    // match z1 {
        Zzz::X(ref mut x) => {
            x.x_str.push_str("xxx");
            x.value += 200;
            println!("{}", x.value);
        },
        Zzz::Y(y) => println!("{}", y.value),
    }
}