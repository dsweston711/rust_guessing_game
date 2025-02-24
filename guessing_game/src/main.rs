use rand::Rng;
// imports rand library into scope
use std::cmp::Ordering;
use std::io; // imports io library into scope

// one of the lines ever: cargo doc --open

fn main() {
    // main function
    println!("Guess the number!");

    // generate a random numnber, save it
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // new mutable, empty instance string
        let mut guess = String::new();

        // we could have used function call std::io::stdin
        // if we didnt import std::io
        io::stdin()
            // call the read_line method on standard input handle to get
            // input from the user
            .read_line(&mut guess)
            // & indicates that this argument is a reference, which gives
            // us a way to let multiple parts of your code access one piece of data
            // without needing to copy that data into memory
            // multiple times
            // references are immutable by default
            // note that this is still one line of code
            // expect method takes in Err value from read_line return value,
            // which is result
            // if you dont call expect, program will compile, but youll
            // get a warning
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            // note: secret_number is a string, so we have to convert it
            // see line 39
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
