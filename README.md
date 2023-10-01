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

To specify the parameters for a function you write the name of the parameter and indicate the type. For example:

```rust
fn add_2_numbers(x: i32, y: i32) -> i32{
    println!("The sum of {} and {} is {}", x, y, x + y);
}
```

The difference between a statement and an expression is that an expression evaluates something and can return a result while a statement does not evaluate anything i.e it defines something. For example **let x = 20** is a statement because it does not return anything while **x + 20** is an expression because it returns something.

To return a value from a function use the **return** keyword. When creating a function that returns a value you have to specify the type of the returned value.

```rust
fn add_2_numbers(x: i32, y: i32) {
    return x + y;
    // can also return by just writing the expression without the return keyword
    // x + y
}
```

## Operators

1. **!** operator is used to indicate that the function being called is infact a macro and not a defined function
1. **::** path seperator operator is used to allow one to access the method of an object or to specify the module of a crate
1. **+** addition operator.
1. **-** subtraction operator.
1. **/** division operator.
1. __*__ multiplication operator
1. **%** modulo operator.
1. **<** less than
1. **>** greater than
1. **<=** less than or equal
1. **>=** greater than or equal
1. **!=** not equal to
1. **==** equal to
1. **!** not operator
1. **&&** and operator
1. **||** or operator
1. **->** return operator that is used to indicate the return type of a function

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

## Importing

Libraries in rust are known as **crates** and the functionalities that can be imported from the crate are known as **modules**.

An example of a crate is **std** which has modules such as **io** that can be used for input and output. An example of how to import the io module is:

```rust
use std::io;
```

## Console Input

We use the **io::read_line()** method to receive input from the terminal.

The method returns accepts a **mutable reference** of the variable you want to store the read value in. The need of a mutable reference is that by **default functions take the copy of an arguement as a value**, in this case we **want to change the actual variable** so we need to **pass a reference**. By **default references are immutable** so we need to **define the reference as mutable**.

The method returns a **Result** which is similar to a future. It is possible that the Result will be an error so we need to handle it. This is done using the **expect()** function.

An example of how to get input is:

```rust
let mut input = String::new();

io::stdin().read_line(&mut input).expect("failed to read");
```

## Type Casting

One way to type cast is to write the type immediately after a literal. For example to cast 255.0 to a f32 we would do: **255.0f32** or seperate the type with an underscore e.g **255_i32**

The other method is to use the **as** keyword e.g **255 as i32**

## Conditions and Control Flow

You can create condition expressions using the condition operators or by calling a function that returns a boolean i.e any **expression that results in a boolen is a condition**.

The **if**, **else if** and **else** operators are used for branching and control flow. An example:

```rust
if condition_1 {
    // Do something
} else if condtion_2 {
    // Do something else
} else {
    // Do something
}
```

## Looping

One way to loop is by using the **loop** keyword. This tells Rust to loop the code in the loop body **forever until it is told to stop**. We tell the loop to stop or *break out of the loop* using the **break** keyword. We can also skip the current iteration and go to the next one using the **continue** keyword. For example:

```rust
let mut i = 1;

// Looping with the loop keyword
loop {
    if i < age {
        println!("Are you {}", i);
        i = i + 1; // Remember to increment control variable
    } else if i == age { // break condition
        println!("Are you {}", i);
        println!("Hooray!");
        break
    } else {
        break
    }
}
```

If the loop was calculating a value that needs to be returned you can place the value to be returned after the break statement. For example:

```rust
let mut counter = 0;

let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
}
```

loop is useful when you want to repeat an action until a certain thing happens but you do not know how many iterations till the desired event or condition occurs.

If you want to loop while a condition is true you could use the loop keyword but **while** has been built to achieve this goal. An example of a while loop:

```rust
while remaining_income > 500.0 {
    remaining_income -= 500.0;
    println!("kshs 500 has been removed for tax, balance is {}", remaining_income);
}
```

If you want to loop through a collection of items you could use the while loop but the **for** loop was built for this purpose. For example:

```rust
for grade in grades {
    summed_grades += grade;
}
```