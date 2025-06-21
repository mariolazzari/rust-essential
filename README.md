# Rust essential training

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