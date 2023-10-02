use std::io;

pub fn get_input() -> String {
    // // Print greater than symbol and space
    // print!("> ");

    // Get input
    let mut input = String::new();
    io::stdin().
    read_line(&mut input).
    expect("failed to read");

    // Return input
    return input;
}