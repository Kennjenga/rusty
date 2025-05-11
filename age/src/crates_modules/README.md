# Packages, Crates, and Modules in Rust

This document provides an overview of how to organize and structure your Rust code using packages, crates, and modules.

## Key Concepts

### 1. **Packages**

- A package is a collection of one or more crates.
- It contains a `Cargo.toml` file that describes how to build the crates.
- A package can contain:
  - **0 or 1 library crate**.
  - **Multiple binary crates**.

### 2. **Crates**

- A crate is a compilation unit in Rust.
- Crates can be:
  - **Binary Crates**: Produce an executable.
  - **Library Crates**: Provide functionality to be used by other crates.

### 3. **Modules**

- Modules help organize code and control privacy.
- You can split code into multiple files and directories.
- Use `mod.rs` to define a module in a directory.

### 4. **Paths**

- Paths are used to name items such as structs, functions, or modules.
- Paths can be:
  - **Absolute**: Starts from the crate root.
  - **Relative**: Starts from the current module.

## Example: Organizing Code

### Step 1: Create a Module

1. Create a directory named `restaurant` inside the `src` directory.
2. Add a file named `mod.rs` inside the `restaurant` directory.

### Step 2: Add Binary Crates

1. Create a `bin` directory inside the `src` directory.
2. Add files like `more_stuff.rs` to define additional binary crates.

## Summary

- **Packages**: Build, test, and share crates.
- **Crates**: Define libraries or executables.
- **Modules**: Organize code and manage privacy.
- **Paths**: Reference items within your code.

By following these principles, you can keep your Rust projects well-organized and maintainable.
