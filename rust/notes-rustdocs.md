# Notes from "The Rust Programming Language"

## 1. Getting Started

- cargo build, cargo run, and cargo check are essential commands for building and running Rust projects, allowing for:
  - `cargo build`: compiles the project.
  - `cargo run`: compiles and runs the project.
  - `cargo check`: checks the code for errors without producing an executable.

## 3. Common Programming Concepts

- Integer overflow in Rust in debug mode causes a panic, while in release mode it wraps around using two's complement wrapping (starting from 0 again).
- Statements and expressions: Statements perform actions and do not return values, while expressions evaluate to a value.
- Loop's labels allow breaking out of nested loops by specifying which loop to break from.
- Like in Python, if can be used in a let statement to assign values based on conditions.

## 4. Understanding Ownership

- Ownership rules:
  1. Each value in Rust has a variable that’s called its owner.
  2. There can only be one owner at a time.
  3. When the owner goes out of scope, the value will be dropped.
- Stack vs. Heap:
  - The stack stores data with a known, fixed size at compile time, and is faster to access.
  - The heap is used for data that can change in size or is too large to fit on the stack, and requires more overhead for allocation and deallocation.
- The `String` type is stored on the heap, allowing for dynamic size and mutability
- The `&str` type is a string slice that is stored on the stack and is immutable.
- Move vs. Clone:
  - Move transfers ownership of data from one variable to another, invalidating the original variable.
  - Clone creates a deep copy of the data, allowing both variables to own their own copies.
- Clone vs copy:
  - Types that implement the `Copy` trait (like integers and booleans) are copied on assignment, meaning both variables can be used independently.
  - Types that do not implement the `Copy` trait (like `String` and vectors) are moved on assignment.
- References and Borrowing:
  - References allow borrowing data without taking ownership, using `&` for immutable references and `&mut` for mutable references.
- Referencing and Dereferencing:
  - The `&` operator is used to create a reference (pointer) to a value, allowing borrowing without taking ownership.
  - The `*` operator is used to dereference a reference, allowing access to the value it points to.
- Rules of references:
  1. At any given time, you can have either one mutable reference or any number of immutable references.
  2. References must always be valid.
- Slices, pros and cons:
  - Pros: allow for safe access to a portion of a collection without needing to copy data.
  - Cons: can lead to dangling references if not managed properly, and require careful handling of lifetimes.

## 5. Using Structs to Structure Related Data

- *Struct Update Syntax*: allows to create a new instance of a struct using values from another instance, but takes ownership of data types that do not implement the `Copy` trait.
- `dbg!` Macro: a macro that prints the value of an expression along with the source code location, useful for debugging.
- With private fields and public getter methods one can implement read-only access to struct fields from outside the module.
- *Associated functions* are good for constructors, especially when they return an instance of the struct.

## 6. Enums and Pattern Matching

- `Option<T>` is an enum that represents either a value of type `T` (`Some(T)`) or no value (`None`). The advantage is that it forces handling of the absence of a value, preventing null pointer exceptions.

## 8. Common Collections

- Using enums in vectors allows storing different types of data in a single collection by defining an enum with variants for each type. Works similarly to lists in Python that can hold mixed types and still have ability to expand.

## 9. Error Handling

- `panic!` macro is used to cause a program to terminate when it encounters an unrecoverable error.
- `Result<T, E>` is an enum that represents either a success (`Ok(T)`) or an error (`Err(E)`), allowing for graceful error handling and propagation.
- The `?` operator is a shorthand for propagating errors in functions that return `Result`. It returns the value inside an `Ok` variant or exits the function early with the `Err` variant if an error occurs. It also works with `Option`, returning `Some` or exiting with `None`, but one cannot mix `Result` and `Option` with `?`.

## 10. Generics, Traits, and Lifetimes

- Generics allow for writing code that can operate on different types while still maintaining type safety. They are defined using angle brackets (`<>`) and can be used with structs, enums, functions, and methods.
- Traits are a way to define shared behavior across different types. They are similar to interfaces in other languages and can be implemented for any type. They are defined using the `trait` keyword and can include method signatures and default implementations.
- Traits can have default method implementations, allowing types that implement the trait to use the default behavior without needing to provide their own implementation or override it if they need custom behavior.
- Lifetimes are a way to specify how long references are valid in Rust. They do not affect the actual lifetime of the data, only that the references are valid for a certain scope (the one of shortest reference).
- Elision rules help the compiler infer lifetimes in certain situations, reducing the need for explicit lifetime annotations. The three rules are:
  1. Each parameter that is a reference gets its own lifetime parameter.
  2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
  3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters.

## 11. Testing

- assert_eq! and assert_ne! can be implemented for any type that implements the `PartialEq` trait, allowing for custom equality checks in tests. They also need the `Debug` trait to print the values when the assertion fails.
- Tests in rust run in parallel by default, but you can use `--test-threads=1` to run them sequentially if needed.
- Unit tests should be placed in a `tests` module within the same file as the code being tested (after #[cfg(test)] annotation), while integration tests should be placed in a separate `tests` directory at the root of the project. Unit tests have access to private items, while integration tests only have access to public items.
- `cargo test` compiles only files in tests directory, not in any subdirectories. Therefore, using convention `./tests/eg_setup/mod.rs`, is a nice way to create a module for shared test setup code that can be used across multiple test files in the tests directory, but not treated as an integration test itself.
- Binary crates (things in `src/main.rs`) cannot be tested with integration tests. The heart and soul of the API must be exposed with a library crate, which integration tests can import with `use`. The idea is that `main.rs` should be thin and only contain code related to running the application, and therefore does not need to be tested directly (lol).
