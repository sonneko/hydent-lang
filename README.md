# Hydent Programming Language


**Hydent** is a modern, statically-typed, JIT-compiled language designed for building **safe, high-performance, and maintainable** applications. It combines a familiar and intuitive syntax inspired by TypeScript with the robust design philosophy gained from Rust.

## Core Philosophy

Hydent is built upon four fundamental principles:

1.  **Pragmatic Safety**: Eliminate entire classes of runtime errors at compile-time. Hydent enforces Null Safety through `Option<T>` and provides a clear dual-model error handling system: `Result<T, E>` for recoverable errors and compiler-trackable `try fn` for unrecoverable panics.

2.  **Expressive & Familiar Syntax**: Lower the barrier to entry with a clean syntax that feels comfortable for developers coming from languages like TypeScript, Kotlin, and Swift. The focus is on readability and reducing boilerplate code.

3.  **Integrated Code Quality**: Language features should guide developers toward writing better code. Hydent has a first-class, compiler-aware documentation system using **directives** (`#summary`, `#panics`), which enables semantic linting and ensures API contracts are documented and checked.

4.  **Performance by Design**: By compiling to LLVM IR and using a Just-In-Time (JIT) compiler, Hydent achieves high execution performance. This makes it suitable for demanding applications without sacrificing the productivity of a high-level language.

## Feature Overview

* **Type System**: Strong, static, and nominal typing.
* **Null Safety**: The `Option<T>` type is used to eradicate null pointer exceptions.
* **Immutable by Default**: Mutability is made explicit with the `mut` keyword.
* **Modern Error Handling**: A clear distinction between recoverable errors (`Result<T, E>` and the `?` operator) and unrecoverable, bug-induced errors (`try fn` and `panic`).
* **Expression-Based**: Most control flow constructs like `if` and `match` are expressions that return values.
* **Object-Oriented**: A consistent object model based on `class` and `protocol` for defining interfaces.
* **Generics**: Write flexible and reusable code with type-parameterized functions and classes.
* **Compiler-Checked Documentation**: Use `#directives` to describe API contracts, which the compiler verifies, preventing documentation from becoming stale.

## Hello, Hydent!

A short code sample showcasing several features of Hydent.

```zf
import { Result, Ok, Err } from "std/result";
import { Option, Some, None } from "std/option";

/// Represents a potential validation error about a user.
class UserError {
    message: String;
}

class User {
    name: String;

    /// Retrieves the username. This function is marked with `try` to indicate
    /// a check for potential programmer errors that might panic.
    #summary "Retrieves the user's name."
    #panics "Panics if the internal name field is unexpectedly empty, indicating a bug."
    try fn get_name(self): String {
        if self.name.is_empty() {
            // If the constructor was used correctly, this should never happen.
            panic("Invariant violation: User object's name is empty!");
        }
        return self.name;
    }

    #summary "Creates a user with a validated name."
    #params name: "The user's name. Must not be empty."
    #returns "A Result containing either a new User or a UserError."
    #side_effects "Prints to the console during validation."
    pub fn new(name: String): Result<Self, UserError> {
        if name.is_empty() {
            return Err(UserError { message: "Name cannot be empty." });
        }
        std::io::println("User successfully validated!");
        Ok(Self { name })
    }
}

fn main() {
    match User::new("Alice".to_string()) {
        Ok(user) => {
            let name = try user.get_name();
            std::io::println("Created user: " + name);
        },
        Err(error) => {
            std::io::println("Failed to create user: " + error.message);
        },
    }
}
````

## Error Handling: In Detail

Hydent takes errors very seriously and provides two distinct pathways.

### 1\. Recoverable Errors (`Result<T, E>`)

For operations that are expected to fail under normal circumstances (e.g., network requests, file I/O, input validation), the function should return a `Result<T, E>`. The `?` operator provides a clean way to propagate these errors.

```zf
fn process_data(): Result<Int, DataError> {
    let raw_data = fetch_data_source1()?; // Propagates Err(DataError)
    let processed_data = transform_data(raw_data)?;
    Ok(processed_data.value)
}
```

### 2\. Unrecoverable Errors (`try fn` & `panic`)

For programming errors that should **never** occur if the program is correct (e.g., broken invariants, out-of-bounds access on a trusted index), Hydent uses `panic`.

A function that can `panic` **must** be marked with the `try` keyword. Any call to a potentially `panic`-ing function marked with `try` must use `try` at the call site to acknowledge the safety risk. This makes the possibility of a `panic` traceable through the entire call stack at compile time.

```zf
// This function assumes the index is always valid. A bug could break this.
try fn get_first_element(list: List<Int>): Int {
    if list.is_empty() {
        panic("Cannot get first element of an empty list!");
    }
    return list[0];
}

// The caller must also acknowledge the risk by being a `try fn`.
try fn main() {
    let my_list = List::new();
    // This will panic, and the compiler forced us to acknowledge that possibility.
    let element = try get_first_element(my_list);
}
```

## Status

**Proof-of-Concept Stage**.

## Contributing

Contributions of all kinds are welcome\! Please see [DEVELOPERS.md](DEVELOPERS.md) and [CONTRIBUTES.md](CONTRIBUTES.md).


---
