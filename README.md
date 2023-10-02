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

## Organizing Code Into Packages, Crates and Modules

### Creating Modules

Rust offers the module system which aims to assist programmers in organizing how they organize projects that could have multiple files and multiple modules.

Rust compiles code in chunks called crates. Crates could be a single file that has code. There are 2 kinds of crates:

1. **Binary Crates** are crates that can be compiled into an executable since they contain a main function.
1. **Library Crates** are not intended to be executed when compiled but provide functionality that can be used in other projects. They do not contain a main function.

A package is a collection of crates that have a Cargo.toml file to describe how to build the package. Packages can have **at most one library crate**.

When creating a package Cargo assumes that the **src/main.rs file is the crate root** of the binary crate with the same name as the package. At the same time if the package contains a **src/lib.rs file Cargo assumes that the package contains a library crate** with the same name as the package and that the file is its root. A package can have **multiple binary crates by placing them in the src/bin** directory.

To use modules in Rust we first declare them in the crate root e.g src/main.rs. We use the **mod** keyword to declare a module. Declared modules are private to its parent by default so to make it public we define it as public using the **pub mod** keywords. We then define what we want to be accessible as public in the module using the **pub** keyword.

If I would want to create a garden module I would add the declaration for it in the crate root which could be either src/main.rs or src/lib.rs. The definition could look like this **mod garden**. Rust would then look for where the code defining the module is. It first looks for a **src/gardern.rs** file, if that is not there it looks for a **src/garden/mod.rs** file.

The defined module can also define its own submodule. If I wanted to define a vegetable submodule for the garden module I would. Move the garden module declaration to the **src/garden/mod.rs** location (*this is optional but in my opinion makes things abit more orderly*). I then create the declaration for the vegetable submodule i.e **mod vegetable**. Rust would then look for the file defining the module at **src/garden/vegetable.rs**. If that is not there it looks for the file at **src/garden/vegetable/mod.rs**.

Once the module is attached to the crate you can reference it either with the path. For example if we wanted to access an aspurugus in the vegetables submodule we would do **crate::garden::vegetable::aspurugus**. To avoid having to write the long path all the time we can use the use keyword e.g **use crate::garden::vegetable::aspurugus**. We would then be able to type aspurugus when we want to access it. The code to do this looks like this:

The src/main.rs file:

```rust
use crate::garden::vegetable::Aspurugus;

pub mod garden;

fn main() {
    let plant = Aspurugus {};
}
```

The src/garden/mod.rs file:

```rust
pub mod vegatable;
```

The src/garden/vegetable.rs file:

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

### Referencing Code In Modules

If we want to be able to use functions or classes defined in modules we would need a way to tell Rust where the code is i.e we need to define a **path to the function**. There are 2 kinds of paths:

1. **Absolute** paths where the path starts from the root crate.
1. **Relative** paths where the path starts from an identifier in the file such as **self**, **super** e.t.c

Let's take this case as an example

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

This brings an error since the hosting module is private. To be able to access the hosting module we make it public by adding the **pub** keyword to it's definition. An error is still brought since the add_to_waitlist method is still private. To fix this we add the **pub** keyword again. In Rust we need to make the module and its contents public if we want to be able to use them.

Modules that are siblings are able to access each other even if they are not defined as public.

## Ownership

This is how Rust ensures memory safety without having a garbage collector i.e avoiding memory leaks.

The stack is a part of the memory that the program has access to during run time that stores elements in a **LIFO** fashion. The items stored on the stack **must be of a known and fixed size**. Items with unkown size are **stored on the heap**.

Storing data on the heap is known as **allocating data**. When you want to allocate some data you tell Rust how much data you want then it will look for a space big enough to fit the data on the heap. Once the space is found it returns a **pointer** to the space which you will use when you want to access the data. **Allocating data on the heap is slower than pushing data on the stack**. *Because pointers are of a fixed size they can be stored on a stack*.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.

These are the ownership rules:

1. Each value in Rust has an owner.
1. There can only be one owner at a time.
1. When the owner is out of scopped the value is dropped.

There's a difference between a **string literal** and a **String**. String is used in cases where we do not know how big the string will be at run time e.g when receiving input from user. You can create a **String from a string** by doing **String::from(literal)**. string literals are **immutable** while String is **mutable**.

How Rust deals with the memory allocated to String or other similar types is that once the variable goes out of scope **the memory allocated to it is deallocated**.

```rust
let s1 = String::from("hello");
let s2 = s1;
```

When we do this no new string is allocated in the heap, what actually happens is that the **String pointer stored in the stack is copied from s1 to s2** making them all point to the same string in the heap.

Also when we do this **Rust ignores s1 making it unusable**. It does this so as to avoid a case where s1 and s2 both try to unallocate their memory that would cause an issue. **s1 was moved to s2**.

To create a deep copy of the string we use the **clone** method i.e:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

In this way s1 can still be used and s1 and s2 have pointers that point to different areas in the heap.

When passing values to functions the same logic applies where some types such as String are moved while others such as i32 are copied. Here's an example:

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

When values go out of scope one of 2 things happen; they are either **Copy** or **Drop**. Values that are **Copy** can still be used after they have gone out of scoped because a copy can be created while values that are **Drop** bring a compile time error if used afterwards.

When a function returns it gives ownership of whatever it's returned to the variable storing the returned value.

But returning a value all the time just to return the ownership can become tedious. To counter this Rust allows us to pass something like a string as an arguement but the function will not take ownership of the passed value. This feature is called **references**

### References

To pass an arguement to a function without passing the ownership of the arguement to the function we use **references** that are indicated with the **&** character. An example is:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

When the function completes executing the passed reference still goes out of scope but because the **reference does not own the data it is pointing to the pointed data is not dropped**. This is called **borrowing**.

In the current implementation we cannot change the string passed because by default **references in Rust are immutable**. To be able to change the data in the passed reference we define the reference parameter as mutable and make sure that the string the reference comes from is also mutable. For example:

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Mutable references have one big restriction: **if you have a mutable reference to a value, you can have no other references to that value, even if they are immutable.**.This is done to prevent the chance of have race conditions.

Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.

### Slices

Rust has slices that allow you to get a smaller portion of an array or String. 