# Notes

- This repo is a **Rust project**
- It contains two **(Cargo) packages**: 
    1. `my-cargo-project`: which contains one binary crate `main.rs` and one library crate `lib.rs`
    2. `another-cargo-project`

## A Cargo project

- `cargo new --bin my-cargo-project`
- `cd my-cargo-project`
- `cargo run`
- `cargo test`

To define both a library and an executable, use the following convention:

- `src/lib.rs` is the main library file
- `src/main.rs` is the main executable file
- To use the lib in your main program, you need to import `extern crate my_cargo_project` and `use my_cargo_project::my_mod`