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

## Primitive data types

### Declaring variables

Rust naming [conventions](http://doc.rust-lang.org/1.0.0/style/style/naming/README.html)

```rust
fn main() {
    // unmutable variable by default
    let x = 10;
    println!("The value of x is: {}", x);

    // nutable variable
    let mut y = 20;
    println!("The value of y is: {}", y);
    y = 30;
    println!("The value of y is now: {}", y);
}
```

### Integers

```rust
fn main() {
    let mut x: u8 = 255;
    x = x + 1;
    println!("x is {}", x);
}
```

### Floats

```rust
fn main() {
    let x: f32 = 10.123456789123456789;
    println!("x is {}", x);
}
```

### Operations

```rust
fn main() {
    let a = 10;
    let b = 3.0;
    let c = a as f64 / (b + 1.0);
    println!("c is {}", c);
}
```

### Formatting

```rust
fn main() {
    let a = 10.0;
    let b = 3.0;
    let c = a / b;
    print!("c is {0:08.3}\na is {1}\nonce again, c is {0}", c, a);
}
```

### Bitwise

```rust
fn main() {
    let mut value = 0b1111_0101u8;
    println!("value is {value}");
    println!("value is {value:08b}");

    value = !value; // NOT
    println!("value is {value:08b}");

    value = value & 0b1111_0111; // clear bit with AND
    println!("value is {value:08b}");
    println!("bit 6 is {}", value & 0b0100_0000); // check bit with AND

    value = value | 0b0100_0000; // set bit with OR
    println!("value is {value:08b}");

    value = value ^ 0b0101_0101; // XOR
    println!("value is {value:08b}");

    value = value << 4; // shift left by 4
    println!("value is {value:08b}");

    value = value >> 2; // shift left by 2
    println!("value is {value:08b}");
}
```

### Boolean

```rust
fn main() {
    let a = true;
    let b = false;
    println!("a is {a} and b is {b}");
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b {}", a ^ b);

    let c = (a ^ b) && panic!();
    println!("c is {}", c);
}
```

### Comparison

```rust

```