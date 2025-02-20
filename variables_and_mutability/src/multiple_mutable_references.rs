fn main() {
    let value = 10; // let mut value = 10;

    let ref1 = &value; // let ref1 = &mut value;
    let ref2 = &value; // let ref2 = &mut value; // ❌ Will this work?
    // ref1 is a mutable refernce to value, meaning it exclusively owns the right to
    // modify it
    // ref2 tries to borrow value mutably while ref1 exists
    // Rust rejects this code for that reason (undefined behavior)
    // to fix this, we can use immutable references instead
    // multiple immutable borrows are allowed

    println!("ref1: {}, ref2: {}", ref1, ref2);
}
