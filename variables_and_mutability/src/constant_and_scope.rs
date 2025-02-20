const LIMIT: u32 = 100;

fn main() {
    println!("Limit: {}", LIMIT);
    // LIMIT = 100; // ❌ Will this work?
    // no. constants are not mutable
    // to fix, either change the actual value of LIMIT
    // or change limit to a mut
    // i decided to get rid of LIMIT = 100 and change LIMIT
}
