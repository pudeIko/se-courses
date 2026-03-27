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
- Like in Python, `if` can be used in a let statement to assign values based on conditions.

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

## 13. Iterators and Closures

- Iterators are lazy, meaning they do not produce values until they are consumed by a method like `collect`, `sum`, or `for_each`. This allows for efficient chaining of iterator methods without creating intermediate collections.
- Closures are anonymous functions that can capture their environment. They are defined using the `||` syntax and can be used in places where a function pointer is expected, such as with iterator methods. They can capture variables from their environment by reference, mutable reference, or by value, depending on how they are used.

## 14. More about Cargo and Crates.io

- workspaces allow for managing multiple related packages (crates) in a single project. They share the same Cargo.lock and output directory, making it easier to manage dependencies and build artifacts across multiple crates.
- `cargo doc --open` generates documentation for the project and opens it in a web browser, allowing for easy access to API documentation.

## 15. Smart Pointers

- Smart pointers are data structures that not only act like a pointer but also have additional metadata and capabilities. They are used to manage memory and resources in Rust, providing features like automatic cleanup and reference counting.
- `Box<T>` is a smart pointer that allocates data on the heap and provides ownership of that data. It is used when you want to store data on the heap or when you need to have a type that has a known size at compile time but contains data of an unknown size (like a recursive data structure).
- `Rc<T>` is a reference-counted smart pointer that allows multiple ownership of the same data. It keeps track of the number of references to the data and deallocates the data when the reference count drops to zero. It is used in single-threaded scenarios where multiple parts of the code need to share ownership of the same data.
- `RefCell<T>` is a smart pointer that provides interior mutability, allowing you to mutate data even when there are immutable references to it. It uses Rust's borrowing rules at runtime, meaning that it will panic if you try to violate the borrowing rules (like having multiple mutable references or a mutable reference while immutable references exist). It is often used in combination with `Rc<T>` to allow for shared ownership and mutation of data in single-threaded scenarios.

## 16. Fearless Concurrency

- `thread::spawn` is used to create a new thread of execution in Rust. It takes a closure as an argument, which contains the code that will be executed in the new thread. The closure can capture variables from its environment, allowing for communication between threads.
- The `move` keyword is used in closures to indicate that the closure should take ownership of the variables it captures from its environment. This is necessary when spawning a new thread, as the closure will be executed in a different thread and needs to own the data it uses to avoid issues with borrowing and lifetimes.
- Message passing is a concurrency model where threads communicate by sending messages to each other, rather than sharing memory. In Rust, this can be achieved using channels, which provide a way for threads to send and receive messages safely without the need for locks.
- `Mutex<T>` is a synchronization primitive that provides mutual exclusion, allowing only one thread to access the data it protects at a time. It is used to protect shared data from concurrent access, ensuring that only one thread can access the data at a time to prevent data races.
- `Arc<T>` is an atomic reference-counted smart pointer that allows for shared ownership of data across multiple threads. It is similar to `Rc<T>`, but it is thread-safe and can be used in multi-threaded scenarios. It keeps track of the number of references to the data and deallocates the data when the reference count drops to zero, just like `Rc<T>`, but it uses atomic operations to ensure thread safety.

## 17. Asynchronous Programming

- `async` functions are functions that can be paused and resumed, allowing for asynchronous programming in Rust. They return a `Future`, which is a value that represents a computation that may not have completed yet.
- `Future` is a trait that represents a value that may not have completed yet. It has a method called `poll` that checks if the future has completed and returns either `Ready` with the result or `Pending` if it is still in progress.
- `async` functions are executed lazily, meaning that they do not start running until they are awaited or polled. This allows for efficient use of resources, as the function will only run when it is needed.
- To execute an `async` function, you need to use an executor, such as the `tokio` crate, which provides an event loop for running asynchronous tasks.
- `await` is a keyword that can be used inside an `async` function to pause the execution of the function until the future it is awaiting has completed. It allows for writing asynchronous code in a more synchronous style, making it easier to read and write. It is necessary to use `await` to get the result of a future, as simply calling an `async` function will return a future that has not yet been executed, which results in a compile-time error if you try to use it without awaiting. Awaiting is also necessary to give back control to the executor, allowing other tasks to run while waiting for the future to complete, and therefore should be thought through carefully to avoid unnecessary blocking of the executor.
- Stream trait is like an asynchronous version of an iterator, allowing for processing a sequence of values asynchronously. It has a method called `poll_next` that checks if the next value is available and returns either `Ready` with the value or `Pending` if it is still in progress. It is useful for handling situations where you need to process a stream of data that may not be available all at once, such as reading from a network socket or processing user input.
- Having a collection of futures that you want to execute concurrently can be achieved using the `join!` macro (when you know the number of futures at compile time) or `join_all` function from the `futures` crate (when you have a dynamic number of futures). These allow you to run multiple futures concurrently and wait for all of them to complete before proceeding. However, it is important that the futures have no self references, as this can lead to issues with borrowing and lifetimes when trying to run them concurrently. Pinning is a concept in Rust that allows you to prevent a value from being moved in memory, which is necessary for certain types of futures that have self-references. The `Pin` type is used to wrap a future and ensure that it cannot be moved, allowing it to safely reference itself without violating Rust's borrowing rules. `Unpin` is a marker trait that indicates that a type can be safely moved, meaning it does not have self-references and does not require pinning. Types that implement `Unpin` can be used with the `join!` macro or `join_all` function without needing to worry about pinning, while types that do not implement `Unpin` will require pinning to be used safely in concurrent execution.

## 18. Object Oriented Programming Features

- trait objects allow for dynamic dispatch, enabling polymorphism in Rust. They are created by using a trait as a type, such as `Box<dyn Trait>`, which allows for storing different types that implement the same trait in a single collection or variable. This is useful for situations where you want to work with different types that share common behavior defined by the trait, without needing to know the specific types at compile time. However, using trait objects comes with some overhead due to dynamic dispatch, so it is important to consider whether they are the right choice for your use case.

## 19. Patterns and Matching

- Patterns are a powerful feature in Rust that allow for matching and destructuring complex data structures. They can be used in `match` statements, `if let` expressions, and `while let` loops to handle different cases based on the structure of the data. Patterns can include literals, variables, wildcards, and more complex structures like tuples and structs. They provide a concise and expressive way to handle different cases in your code, making it easier to read and maintain.

## 20. Advanced Features

- Unsafe Rust allows for writing code that can bypass some of Rust's safety guarantees, such as dereferencing raw pointers, calling unsafe functions, and accessing mutable static variables. It is used when you need to perform operations that are not possible in safe Rust, such as interfacing with low-level system APIs or optimizing performance-critical code. However, it is important to use unsafe code with caution, as it can lead to undefined behavior if not used correctly. It is recommended to minimize the amount of unsafe code in your project and to thoroughly test any unsafe code you write to ensure it is correct and does not introduce security vulnerabilities.
- Most common use cases for unsafe code include:
  - Interfacing with C code or other foreign function interfaces (FFI).
  - Implementing low-level data structures or algorithms that require direct memory manipulation.
  - Optimizing performance-critical code where the overhead of safe abstractions is unacceptable.
- When implementing a struct with multiple traits, and those traits have methods with the same name, you can disambiguate which method to call by using fully qualified syntax, such as `<Struct as Trait>::method()`. This allows you to specify which trait's method you want to call, avoiding ambiguity and ensuring that the correct method is executed.
- `NewType` pattern is a design pattern in Rust where you create a new type that wraps an existing type, allowing you to add additional functionality or enforce certain invariants while still leveraging the underlying type's behavior. This is often done by defining a tuple struct with a single field that holds the wrapped type. The new type can then implement traits and methods to provide additional functionality or enforce specific rules, while still allowing access to the underlying type's methods and properties when needed. This pattern is useful for creating more expressive and type-safe code without sacrificing the benefits of the underlying type.
- Synonyms and type aliases in Rust allow you to create new names for existing types, making your code more readable and easier to understand. Type aliases are defined using the `type` keyword and can be used to give a more descriptive name to a type, especially when dealing with complex types or when you want to abstract away implementation details.
- Never Type (`!`) is a special type in Rust that represents a value that never exists. It is used to indicate that a function will never return, such as when it panics or enters an infinite loop. The never type can be useful for functions that are intended to diverge, as it allows you to specify that they will not return a value, and can help with type inference in certain situations.
- Function pointers in Rust are a way to refer to functions as values, allowing you to pass them around and call them dynamically.
- Macros in Rust are a powerful feature that allows for metaprogramming, enabling you to write code that generates other code. They are defined using the `macro_rules!` macro and can be used to create custom syntax, reduce boilerplate, and implement domain-specific languages (DSLs). Most common macros include:
  - `println!`: a macro for printing to the console.
  - `vec!`: a macro for creating a vector with a specified set of elements.
  - `format!`: a macro for creating formatted strings.
  - `assert!`, `assert_eq!`, and `assert_ne!`: macros for writing assertions in tests.
