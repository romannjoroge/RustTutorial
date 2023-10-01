fn main() {
    // Variables are defined using the let keyword
    let x:u32 = 4; // Immutable variable

    let mut y:u32 = 5;

    // To place values in a format string we place curly braces in the position where we want the variable
    println!("x is: {}", x);
    println!("y is {}", y);

    y = 6;
    println!("y is {}", y)
}
