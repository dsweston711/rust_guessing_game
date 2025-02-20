fn main() {
    let x = 3; // x is 3
    let x = x + 2; // x is 5 (?)
    {
        let x = x * 2; // x is 10 (5 from outer)
        // note that inner x is not used in the outer 
        println!("Inner x: {}", x);
    }
    println!("Outer x: {}", x);
}
