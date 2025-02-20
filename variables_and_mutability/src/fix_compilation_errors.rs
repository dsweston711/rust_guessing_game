fn main() {
    let mut x = 5; // let x = 5;
    println!("x before: {}", x);

    x = 10; // this can be fixed using either mut or let (shadowing)
    // i chose mut

    println!("x after: {}", x);
}
