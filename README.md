# RustTutorial

## Resources

The rust book where the content of the youtube tutorial comes from: [Rust Book](https://doc.rust-lang.org/book/)

The youtube video used: [Rust Tutorial](https://www.youtube.com/watch?v=T_KrYLW4jw8&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ)

## Introduction

Rust is strongly typed i.e once a variable is created with a specific type you **cannot change the type of the variable at a later time**. The compiler is able to **guess the type of a variable** when it is defined or one can expliciltly state its type.

Rust is a **compiled language** meaning you have to first compile it to an executable then run the executable. The beauty of this is that the executable can be run by *any machine even if it doesn't have Rust installed*. (Maybe this is how Rust is used in IoT things)

Rust uses rustup for **version management**.

To view a local version of the rust documentation in your browser you can run the command

```bash
rustup doc
```

## Functions

The **main** function denotes the entry point for a rust file i.e it is the function that is executed when the rust program is executed.

The way to denote that something is a function is to use the **fn** keyword. This is then followed by the **name of the function**, **arguements** in parentheses and curly braces for the function body. An example of a function is:

```rust
fn main() {
    println!("Hello World");
}
```

As shown above functions are executed by typing the name of the function, specifying the arguements in parentheses then placing a semicolon.

## Operators

1. **!** operator is used to indicate that the function being called is infact a macro and not a defined function
1. **::** path seperator operator is used to allow one to access the method of an object or to specify the module of a crate

## Compiling

To compile the written rust code run the command:

```bash
rustc [name_of_rust_file].rs
```

## Creating Rust packages / applications

Every project has a **main.rs** file that acts as the entry point to the project. The main.rs file also has a **main** function.

When you want to create a rust application that could have multiple files and many dependencies we use the **cargo** command line tool to create for us a Rust project.

Cargo is useful for managing our packages, building our applications, running them.

To create a project run the following command:

```bash
cargo new [project_name]
```

This creates a new directory called [project_name] that will hold the Rust code that belong to your project.

If you started a project that doesn’t use Cargo you can convert it to a project that does use Cargo. **Move the project code into the src directory** and **create an appropriate Cargo.toml** file.

When using cargo to manage our projects we **NO LONGER BUILD OUR FILES USING THE RUSTC COMMAND**. We instead use the *cargo build* command to build the files in the src folder which will then create executables that will be stored in the *target* folder.

To run the built project we run the file that has the **same name as our project** and is found in the **target/debug** folder.

**cargo run** builds and executes the project for us i.e it combines the previos 2 steps

**cargo check** is used to check whether your program is able to compile

## Variables

To define a variable we use the **let** keyword. For example:

```rust
let name = "Roman";
let x: u32 = 4;
```

By default variables defined in Rust are **immutable**. To define a variable as mutable in Rust we add the **mut** keyword to the definition. For example:

```rust
let mut x:u32 = 4;
x = 5; // This will work
```

To define constants in Rust we use the **const** keyword followed by the **name of the variable**, the **type** of the variable and the **value** of the variable. For example:

```rust
const MINUTES_IN_HOUR: u32 = 60;
```

## Data Types

Some of the primitive data types include:

1. **Signed integers** e.g **i32**, **i16** etc.
1. **Unsigned integers** e.g **u32**, **u16** etc.
1. **Floating point** numbers e.g **f32**, **f64**.
1. **Booleans** which are either **true** or **false**.
1. **Characters** (char) which are written using single quotes e.g 'a'.
1. **Tuples** which are written using (). Tuples can store values of many different types e.g *let tup = ('a', 1, true)*. To *index values in the tuple we use a .* e.g tup.0 is the first element.
1. **Arrays** which are written using []. The elements of an array are all of the same type. To *index arrays we use brackets e.g arr[0]* will take the first element in the arr array. Arrays in rust are not growable. By default arrays are not mutable unless they are defined to be i.e *let mut arr = [1, 2, 3]*. 
