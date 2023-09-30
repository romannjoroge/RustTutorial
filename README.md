# RustTutorial

## Resources

The rust book where the content of the youtube tutorial comes from: [Rust Book](https://doc.rust-lang.org/book/)

The youtube video used: [Rust Tutorial](https://www.youtube.com/watch?v=T_KrYLW4jw8&list=PLzMcBGfZo4-nyLTlSRBvo0zjSnCnqjHYQ)

## Introduction

Rust is a language where you have to **specify the types of variables** or objects used in programs i.e the language can not guess for you the type of an entity.

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

**!** operator is used to indicate that the function being called is infact a macro and not a defined function


