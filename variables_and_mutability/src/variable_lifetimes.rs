fn get_ref() -> &i32 {
    let num = 42;
    &num // ❌ Will this work?
    // &num tries to return a reference to num
    // but since num was declared in get_ref, it gets dropped at the 
    // end of the function
    // this produces a "borrowed value does not live long enough"
    // error, particularly a dangling reference error
    // num is a local variable inside get_ref, and it goes
    // out of scope when get_ref finishes executing
    // to fix this, we can return the value of num itself rather than
    // a reference to num, but i suspect this will be an issue if
    // we cant return references to dynamically allocated arrays
}

fn main() {
    let reference = get_ref();
    println!("{}", reference);
}
