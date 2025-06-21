# Rust essential training

Rust [book](https://doc.rust-lang.org/book/)

## First program

### Install

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
rustup update
rustup doc
```

### Anatomy

```rust
fn main() {
    println!("Hello, world!");
}
```

```sh
rustc main.rs
./main
```

### Comments

```rust
// main function is the entry point of a Rust program.
fn main() {
    /*
    This is a comment in Rust.
    Rust supports both single-line and multi-line comments.
    Single-line comments start with `//`.
    */

    println!("Hello, world!");
}
```

### Cargo

```sh
cargo --version
cargo new my_project
cargo run
cargo build --release
```

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2024"

[dependencies]
```

Cargo [book](https://doc.rust-lang.org/cargo/)