// Accepts input from user and converts to int

use std::io;

fn main() {
    // Get user input
    let mut input = String::new();

    io::stdin().
    read_line(&mut input).
    expect("failed to get user input");

    // Convert to int
    let converted_int: i64 = input.trim().parse::<i64>().unwrap();

    println!("{} + 2 = {}", converted_int, converted_int + 2);
}
