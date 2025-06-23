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
fn main() {
    let a = 1;
    let b = 2;
    println!("a is {a} and b is {b}");
    println!("a EQUAL TO b is {}", a == b);
    println!("a NOT EQUAL TO b is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THAN OR EQUAL TO b is {}", a >= b);
    println!("a LESS THAN b is {}", a < b);
    println!("a LESS THAN OR EQUAL TO b is {}", a <= b);
}
```

### Char

```rust
fn main() {
    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}';
    println!("{letter}\n{number}\n{finger}");
}
```

## Compound types

### Arrays

- Collection of elements of a type
- Stored in order
- Stored in contiguous memory positions
- Fixed length

```rust
fn main() {
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);

    let numbers: [i32; 5];
    numbers = [0; 5];
    let index: usize = numbers.len();
    println!("last number is {}", numbers[index - 1]);
}
```

### Multidimensional arrays

```rust
fn main() {
    let parking_lot = [[1, 2, 3], [4, 5, 6]];

    let number = parking_lot[1][2];
    println!("number is {}", number);

    let garage = [[[0; 10]; 10]; 100];
}
```

### Tuples

- Groups multiple items of mixed data types.
- Elements are ordered
- Stored in coniguous memory
- Types must be known at compile time

```rust
fn main() {
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first_item is {}", first_item);

    let (a, b, c) = stuff;
    println!("b is {}", b);
}
```

## Functions

### Parameters

```rust
fn main() {
    say_hello();
    say_hello();
    let x = 1;
    let y = 2;
    say_the_sum(x, y);
    say_a_number(x as i32);
}

fn say_hello() {
    println!("Hello!");
    say_a_number(13);
}

fn say_a_number(number: i32) {
    println!("number is {number}");
}

fn say_the_sum(a: u8, b: u8) {
    let sum = a + b;
    println!("sum is {sum}");
}
```

### Statements vs expressions

#### Statements

- Perform an action without returning a value
- Ends with a semicolon

#### Expressions

- Evaluates to a resulting value
- Does not end with semicolon
- 

### Function return values

```rust
fn main() {
    let result = square(13);
    println!("result is {:?}", result);
}

fn square(x: i32) -> (i32, i32) {
    println!("squaring {}", x);
    return (x, x * x);
    // println!("End of function"); // This line will never be executed because of the return statement above.
}
```

### CHallenge

```rust
fn main() {
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}

/* YOUR CODE GOES HERE */
fn celsius_to_fahrenheit(temp: f64) -> f64 {
    (temp * 9.0 / 5.0) + 32.0
}
```

## Program control flow

### Conditional execution

```rust
fn main() {
    let x = 4;

    if x + 1 != 3 {
        println!("x + 1 is NOT 3!");
    }
}
```

### Multiple conditions

```rust
fn main() {
    let x = 3;
    let y = 5;

    if x > y {
        println!("x is greater than y");
    } else {
        if x < y {
            println!("x is less than y");
        } else {
            println!("x is equal to y");
        }
    }

    if x > y {
        println!("x is greater than y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x is equal to y");
    }
}
```

### Conditional assignement

```rust
fn main() {
    let make_x_odd = true;
    let x = if make_x_odd {1} else {2.0};
    
    /* if make_x_odd {
        x = 1;
    } else {
        // x = 2;
    } */
    
    println!("x is {}", x);    
}
```

### 