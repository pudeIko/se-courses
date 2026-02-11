# Rust Programming

## Rust Fundamentals

### Setting up

- Rust - A systems programming language the course focuses on learning.
- Cargo - Rust's built-in package manager mentioned for managing dependencies.
- crates.io - A repository of Rust packages that can be added as dependencies.
- Compiler - Translates Rust code and provides errors during compilation.
- Rust Analyzer - A Visual Studio Code extension for Rust development.
- Visual Studio Code - The recommended text editor for the course.
- Settings Sync - Syncs VS Code settings between devices. Useful for codespaces.

---

- GitHub Copilot - An AI assistant extension for Visual Studio Code that
provides code suggestions and completions.
- Code suggestions - GitHub Copilot provides relevant code suggestions as you
type based on context.
- Completion - Pressing tab inserts Copilot's suggested code completions.
- Prompt-based programming - Writing comments to prompt Copilot to generate
code blocks.
- Copilot Explain - A Copilot feature that explains highlighted code.

---

- Codespaces - Provides a cloud IDE through Visual Studio Code. Offers a
normalized environment.
- Dev containers - Configuration files that define the Codespaces environment.
- Devcontainer.json - The main dev container config file that sets up tools,
extensions, etc.
- Dockerfile - Optional file to customize the OS and install system packages.
- Rebuild container - Rebuilds the Codespaces environment with your new config
files.
- Quotas - Usage limits for Codespaces, including compute time and storage.

### Basics

- **Binary application/package**: Executables generated from Rust source files,
typically containing a main function.
- **Library**: A collection of Rust modules providing functionality meant to be
shared among multiple projects.
- **Cargo.toml**: A configuration file read by Cargo, listing metadata (e.g., name,
version) and dependencies required by the package.
- **Shadowing**: Reassigning a variable to a new value while preserving its
original binding, enabling changes to its type or scope.
- **Control Flow**: Conditional execution paths based on evaluation of logical
expressions, including if, else, and else if clauses.
- **Scope**: A region within source code where names (e.g., functions, variables)
are accessible; determined by enclosing braces ({}) or indentation levels.
- Semicolons: Terminators denoting statement boundaries, required in most cases
except inside blocks, expressions, and macros.

---

- **loop**: A keyword used for an infinite loop, which can be exited using a break
statement.
- **while**: A conditional loop that continues as long as its condition is true.
- **for**: A loop that iterates through elements of a collection or range.
- **break**: A control flow keyword to exit the current innermost loop early.
- **mutability**: The ability for a variable to have its value changed during
runtime by marking it as mutable with the mut keyword
- **option**: A Rust enum type that can be either Some(value) or None, used to
represent optional values.
- **continue**: A control flow keyword to skip an iteration and move on to the next
one in the same loop.
- **if let**: A pattern matching construct that allows you to conditionally bind
variables based on their match against a given value or pattern.
- **sum**: An enum type wrapper around Option which can be either Some(value) or
None.
- **range**: Represents a sequence of numbers, often used in loops for iteration
purposes.
- **shadowing**: A variable redeclaration with the same name but different value
and/or scope within the same context.

---

- **Function**: A block of code designed to carry out a specified task. In Rust, it's a crucial part of the language as Rust is almost like a functional programming language.
- **Unit Function**: A function that doesn't return any value but does some work when called.
- **Return Value**: The result given by a function upon completion. It can be explicitly defined or implicitly returned as unit type in case of unit functions.
- **Borrowing**: Concept in Rust which ensures efficient memory usage by safely lending values to other parts of the code without taking ownership away from their original scopes.
- **Panic**: A special call syntax in Rust used to stop all execution in a program; it's not commonly used in production code but can be helpful during development or for certain error handling scenarios.
- **Control Flow**: The order in which code is executed based on conditions and loops. In Rust, control flow includes if, else, match, and looping constructs like for or while.
- **Enumerator (Enum)**: A data type representing a set of values where each value represents a distinct case. An example in the transcript is the Option<T> enum with cases Some(T) and None.
- **Move**: In Rust, move occurs when ownership of a variable is transferred from one scope to another without any borrowing mechanism being used. This results in the original variable becoming invalid.
- **Copy**: A special trait in Rust which allows values of certain types (e.g., integers and booleans) to be copied instead of moved or borrowed when assigned or passed as arguments.
- **Vector**: A dynamic array data structure provided by Rust's standard library, used for storing a variable number of elements efficiently. It can grow or shrink in size during runtime.
- **Shadowing**: In Rust, the ability to redefine a variable with a new value within the same scope hiding the original definition but not invalidating it. This is useful when changing types locally without affecting other parts of the code.

### Struct, Types, and Enums

- **Struct**: A keyword used to organize similar data in a structure. It is like an object in JavaScript or Python dictionary where you are organizing data in a structured way.
- **Field**: The values of the struct, such as first name and last name string for person struct.
- **Debug**: An attribute that allows printing the whole struct instead of specific fields.
- **Type**: The kind of value each field can hold, such as string or unsigned integer 8 bits in size (u8).
- **Instance**: A created struct with data in its fields, like Fredo equals person with first name Sanchez and age H25.
- **Option**: Represents the absence of a value or a specific type that could be (for example) an unsigned integer for eight bits in size (u8) or none.
- **Implementation**: A keyword used to extend struct by adding functions and associated code.
- **Associated Function**: A function that doesn't require self, allowing easy creation of a user instance with new constructor.
- **Constructor**: Automates tedious repetitive tasks when creating instances, like setting the active field to true in user struct.
- **Immutable**: Cannot be changed after initialization, such as new user being immutable by default.

--

- **String** : A sequence of characters, typically used to represent text. In Rust, there are two primary types of strings: string slices (&str) and strings (String).
- **String slice** : A reference to a sequence of characters in memory. It is immutable and has a fixed size. Represented as &str.
- **String (this is not repeated!)** - A growable, owned sequence of characters. It is mutable and its size can change during runtime. Represented as String.
- **Vector** : A collection of items that can be of any type. It is similar to arrays or lists in other languages. In Rust, vectors are represented as Vec<T>, where T is the type of elements contained within the vector.
- **Immutable** : A value that cannot be changed after it has been created. Strings slices and vectors can be immutable.
- **Mutable** - A value that can be changed after it has been created. Only strings and mutable vectors can be mutable.
- **Borrowing** : The process of temporarily accessing a resource without taking ownership. In Rust, borrowing is used to allow multiple references to the same data without violating memory safety rules.
- **Ownership** : The concept that a value can only have one owner at a time in Rust. When a value is transferred or dropped, its previous owner loses access to it.
- **Slice** : A view into a contiguous sequence of elements, such as an array, string, or vector. It has a fixed size and does not own the data it points to. Represented as [T].
- **Borrowing and Lifetimes** : Borrowing is a mechanism in Rust that allows multiple references to the same data without violating memory safety rules. Lifetimes ensure that borrowed references remain valid for as long as they are needed, preventing dangling pointers or use-after-free errors.
- **Mutable References** : A mutable reference is a reference to a value that can be changed during its lifetime. In Rust, only one mutable reference can exist at any given time for a particular piece of data, ensuring memory safety and preventing race conditions.

---

- **Enum**: An enum, short for enumerator, is a data type in Rust that allows you to define a set of possible values. It's used to create custom types that represent distinct variants or cases. Enums are powerful because they enable you to encapsulate related values and provide exhaustive matching capabilities through the match keyword.
- **Variant**: A variant is a specific case within an enum. Each enum can have multiple variants, which may optionally contain associated data.
- **Match keyword**: The match keyword is a powerful control flow construct that enables exhaustive pattern matching against enums or other values. It checks each case to find the first match, making it an excellent way to handle different enum variants in Rust.
- **Option**: The Option type is a built-in enum provided by Rust that represents either the presence or absence of a value, denoted as Some(value) or None. It's often used to handle nullable values in other languages.
- **Sum type**: A sum type is an enumeration that can hold a variant with associated data of some type D, allowing you to create instances containing different types or combinations of data. In Rust, enums are examples of sum types because they let you define multiple variants, each potentially carrying distinct information.
- **Associated functions/methods**: In Rust, it's possible to extend enums with implementation blocks that contain associated functions or methods, just like structs. This enables you to add functionality specific to the enum type directly within the enum definition.
- **Vectors**: A vector is a growable array-like data structure in Rust that can hold multiple values of the same type. In the context of enums, vectors are useful when you want to store and manipulate collections of related enum instances or variants.
- **Exhaustive matches**: Exhaustive matching is the practice of handling all possible cases when working with enums, ensuring that every variant has been accounted for in your code. Rust's type system helps enforce exhaustiveness by requiring you to handle each enum variant or use a catch-all pattern (_) if needed.
- **Non-exhaustive patterns**: When working with enums, non-exhaustive patterns refer to cases where not all possible enum variants are explicitly handled in a match statement or conditional expression. Rust's compiler will warn you about missing arms (unhandled variants) unless you use the catch-all pattern _ or explicitly mark your enum as non-exhaustive using the ... syntax.

### Applying Rust

- **CLI tool**: Command Line Interface tool is a type of software that can be used at the command line of an operating system.
- **Makefile**: A makefile contains targets to compile your code into executable binaries or libraries. It also includes rules for managing dependencies between files and creating distribution packages.
- **debugger**: Debugging tools help developers identify issues in their programs by stepping through the code, line by line, while monitoring variable values and other aspects of program execution.
- **library**: A collection of reusable functions, structures, macros, traits, and types. Libraries are distributed as binary crates or source code packages on crates.io.
- **Cargo.toml**: The configuration file for Cargo that contains metadata about your project such as its name, version, authors, dependencies, and links to documentation.
- **buffered reader**: A buffered reader is a type of input stream in Rust which reads data from a source (like stdin) into an internal buffer before providing it to the application for processing. This can improve performance by reducing the number of system calls required for reading small chunks of data.
- **std::io::Stdin**: The standard input handle, allowing you to read from user input or pipes in Rust programs.

---

- **Modules**: In Rust, modules provide a way of organizing code within a crate by grouping related functions, types, and constants together. They help with code reuse, separation of concerns, and improving the readability of large projects.
- **Public/Private visibility**: Public items are accessible from outside their defining module, while private ones can only be accessed from within that module. This allows controlling access to parts of your library or application's internals.
- **pub keyword**: The pub keyword is used in Rust to make an item public and visible outside its defining module. It can be applied before functions, structs, enums, modules, and other items.
- **Doc tests**: Doc tests are a feature of the Rust documentation system that allows you to include executable examples within your comments. These examples will automatically run when building or testing your project, ensuring they remain up-to-date with any changes in the codebase.

---

- **Testing**: The process of evaluating a program or system by running it under controlled conditions to ensure correctness, robustness, and reliability.
- **Strongly Typed Language**: A programming language that utilizes static typing, enforcing strict data types during compilation rather than runtime.
- **Test Attribute**: In Rust, a special attribute (#[test]) used to denote functions as tests in test modules or files.
- **Integration Tests**: Tests written for external interfaces and behavior of software components working together; often placed in the tests/ directory in Rust projects.
- **Unit Tests**: Tests focusing on individual units, methods, or functions within a module or library file.
- **Public vs Private Code**: In Rust, code visibility can be controlled using the pub keyword for public access and without it (private) to restrict usage.
- **Test Coverage**: The degree of testing performed in a project, often measured as a percentage of lines of code tested or branches covered.
- **Assertions**: Statements used within tests to verify expected outcomes by comparing actual results with desired values.
- **Buff Read**: A Rust trait implementing read operations on buffered data streams; useful for reading from files and standard input/output.

### Data Engineering with Rust

### Rust Data Structures: Collections

- **Rayon**: A Rust library providing simple parallelism for performance. Uses
thread pool model for concurrent code easily.

---

- **Vector** - Growable array, similar to Python list
- **VecDeque** - Double-ended queue with fast appends/pops on both ends
- **Linked List** - List with efficient inserts/removals in middle but slower indexing
- **HashMap** - Key-value store, similar to Python dict
- **Sequence** - Immutable ordered collection indexed by position, similar to Python tuple

---

- **Graph**: A structure to model connections between data. Used for social networks, recommendations, etc
- **Centrality**: A graph metric that indicates the importance of a node based on its connections. Useful for finding key players. Higher scores mean more connections.
- **HashSet**: A Rust collection that only stores unique elements.
- **Shortest Path Algorithm**: Algorithms like Dijkstra that efficiently find the shortest distance between two nodes in a graph. Useful for GPS navigation.
- **Community Detection**: Algorithms that identify closely connected groups like friend circles. Useful for social network analysis.
- **BTreeSet**: A ordered, unique Rust collection.
- **Binary Heap**: A priority Rust queue useful for ordering elements.
- **Node**: A record in the graph representing an entity like a person or event.
- **Relationship**: A connection between nodes like friends or co-workers. Has a direction and type.
- **Label**: Categories used to group similar nodes like "Person". Help query specific nodes faster.

### Safety, Security, and Concurrency with Rust

- **Ownership** - Resource management concept where variables have a single owner. When owner goes out of scope, resource is dropped.
- **Borrowing** - Using a reference to a resource owned by another variable instead of taking ownership.
- **Lifetimes** - Annotate references with how long they live. Ensures validity.
- **Threads** - Concurrent lightweight tasks.
- **Mutex** - Mutual exclusion mechanism to coordinate thread safety.

---

- **Threads** - Lightweight units of execution that allow concurrent execution in a program. Can be spawned with the thread::spawn function.
- **Channels** - Allow message passing between threads. Created with mpsc::channel and allow sending values with send and receiving with recv.
- **Mutex** - Enforces exclusive access to data across threads. Created with Mutex::new and data accessed through the lock method.
- **Arc** - Safely shares ownership of data across threads. Wraps types and allows them to be immutably shared between threads.
- **Rayon** - Parallelization library that launches threads to speed up operations like maps, filters, reductions etc.

---

- **DataFrame**: A tabular data structure containing rows and columns, similar to a spreadsheet or SQL table. Useful for data analysis and manipulation.
- **EDA (Exploratory Data Analysis)**: The process of exploring and summarizing datasets to understand their characteristics, find patterns, and validate assumptions prior to applying predictive models.
- **Notebook**: An interactive coding environment that allows writing and executing code, visualizations, and text. Useful for EDA and sharing work.
- **MLFlow**: An open source platform for managing the machine learning lifecycle, including experimentation, reproducibility, and deployment.
- **Polars**: A Rust DataFrame library optimized for speed and productivity. Useful for data manipulation at scale. Available in Python and Rust.
- **Evcxr_jupyter**: A Rust kernel for Jupyter notebooks that allows running Rust code interactively within a notebook environment.
