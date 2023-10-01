use std::io; // Imports the io module

fn main() {
    // An example of how to accept input from terminal
    let mut input = String::new();

    // Get input from user
    io::stdin()
    .read_line(&mut input)
    .expect("failed to get input");

    // Print input
    println!("The user entered {}", input);
}
