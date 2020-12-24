fn main() {
    println!("Hello, world!");
    // arrays
    const SIZE: usize = 4;
    let _arr: [i32; SIZE] = [1, 2, 3, 4];
    // immutability
    let mut _x = 2;
    let _y = _x;
    _x = 3;
    // _y = 3; ERROR

    let _x = "hello";
    let _y = _x;
    // _x = " world"; ERROR
    // _y = 3; ERROR

    let _s = String::from("hello ");
    // _s.push_str("world"); ERROR
    let mut _s = String::from("hello ");
    _s.push_str("world");

    let _x = 2;
    let mut _y = _x;
    _y = 3;
    // _x = 3; ERROR

    // immutable + mutable references
    let _x = 2;
    let _y = &_x;
    // _y = &2; ERROR
    // _y = 2; ERROR even if mut
    // let _y = &mut _x; ERROR: _x not declared as mutable
    let mut _y = &_x;
    _y = &3;
    // _y = 3; ERROR, not compatible types
    let mut _x = String::from("hello");
    let _y = &_x;
    // _y = &mut _x; ERROR: can't access assign twice to immutable {_y}
    let _y = &mut _x;
    _y.push_str(" world");
    let mut _y = &mut _x;
    _y = &mut String::from("blah"); // works because mut

    let _s = String::from("hello");
    // _s.push_str(" world"); ERROR, not mutable

    let _arr = [1, 2, 3];
    let _s = &_arr[0..2];

    let _x: (&str, i32, i32) = ("hello", 2, 3);
    _x.1;
}

fn _takes_ownership_and_modifies_imm(_x: u32) {
    // x = 3; ERROR
}

fn _takes_ownership_and_modifies_mut(mut _x: u32) {
    _x = 2;
}

fn _borrows_and_modifies_mut(mut _s: &String) {
    // _s.push_str("foo"); ERROR
}

fn _borrows_and_modifies_mutref(_s: &mut String) {
    _s.push_str("foo");
}
