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
      
    println!("x is {}", x);    
}
```

### Loops

```rust
fn main() {
    let mut count = 0;

    let result = loop {
        if count == 10 {
            break count * 10;
        }
        count += 1;
        println!("count is {}", count);
    };

    println!("After the loop!");
    println!("result is {}", result);
}
```

### While

```rust
fn main() {
    let mut count = 0;
    let letters = ['a', 'b', 'c'];

    while count < letters.len() {
        println!("letter is {}", letters[count]);
        count += 1;
    }
}
```

### For

#### Iterators

- Implement logic to iterate over each item of a collection
- *next()* returns next element of the sequence

```rust
fn main() {
    let message = ['h', 'e', 'l', 'l', 'o'];

    for (index, &item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
        if item == 'e' {
            break;
        }
    }

    for number in 0..5 {
        println!("number is {}", number);
    }
}
```

### Nested loops

```rust
fn main() {
    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t", num);
        }
        println!();
    }
}
```

### Challenge

```rust
fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = numbers[0];
    let mut min: i32 = numbers[0];
    let mean: f64;

    /* YOUR CODE GOES HERE */

    mean = loop {
        let mut sum = 0;
        for num in numbers {
            if num > max {
                max = num;
            }
            if num < min {
                min = num;
            }
            sum += num;
        }
        break sum as f64 / numbers.len() as f64;
    };

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}
```

## Ownership

### Scope

- Portion where a variable is valid
- Blocks of code are defined by {}

```rust
fn main() {
    let planet = "Earth";
    if true {
        println!("planet is {planet}");
    }
    println!("planet is {planet}");
}
```

### Shadowing

- Declare a new variable with the same name as an existing one
- New variable shadows the previous one
-  

```rust
fn main() {
    let planet = "Earth";
    {
        println!("planet is {planet}");
        let planet = 4;
        println!("planet is {planet}");
    }
    println!("planet is {planet}");
}
```

### Stack and heap values

#### Stack

- Values stored in sequential order
- LIFO
- Quick data access (push and pop)
- Small size
- Fixed size data

#### Heap

- Data store memory address
- Dara access is slower
- Dynamic data add and remove

### String

Strings [documentation](https://doc.rust-lang.org/std/string/struct.String.html)

#### Literal
- "..."
- Hard coded
- Immutable
- Must be known before compilation

#### Type

- Allocated on the heap
- Mutable
- Dynamically generated at runtime
  
```rust
fn main() {
    let mut message = String::from("Earth");
    println!("message is {message}");
    message.push_str(" is home.");
    println!("message is {message}");
}
```

### Ownership

- Head has lots of space
- Not infinite
- Cleanup unused blocks
- **Variables are responsible for freeing their own resources**
  - Value is owned by *one and only one* variable at the time
  - When a variable runs out of scoped, its value is dropped 

### Move, copy and clone data

- Copy
  - Stack only data types
  - Implicity

```rust
fn main() {
    let outer_planet: i32;
    {
        let mut inner_planet = 1;
        outer_planet = inner_planet;
        inner_planet += 1;
        println!("inner_planet is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);
}
```

### Transfering ownership

```rust
fn main() {
    let rocket_fuel = String::from("RP-1");
    let rocket_fuel = process_fuel(rocket_fuel);
    println!("rocket_fuel is {}", rocket_fuel);
}

fn process_fuel(propellant: String) -> String {
    println!("processing propellant {}...", propellant);
    let new_fuel = String::from("LNG");
    new_fuel
}
```

## Borrowing

### Borrowing references

- Access data without taking its ownership
- Create a reference using borrow operator *&*

```rust
fn main() {
    let rocket_fuel = String::from("RP-1");
    let length = process_fuel(&rocket_fuel);
    println!("rocket_fuel is {} and length is {}", rocket_fuel, length);
}

fn process_fuel(propellant: &String) -> usize {
    println!("processing propellant {}...", propellant);
    let length = propellant.len();
    length
}
```

### Mutable reference

- When you create a mutable reference, you cannot create other references
- Prevents race conditions

```rust
fn main() {
    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel(&mut rocket_fuel);
    println!("rocket_fuel is {} and length is {}", rocket_fuel, length);
}

fn process_fuel(propellant: &mut String) -> usize {
    println!("processing propellant {}...", propellant);
    propellant.push_str(" is highly flammable!");
    let length = propellant.len();
    length
}
```

### Dangling reference

```rust
fn main() {
    let rocket_fuel = produce_fuel();
    println!("rocket_fuel is {}", rocket_fuel);
}

fn produce_fuel() -> String {
    let new_fuel = String::from("RP-1");
    new_fuel
}
```

### Slice

- Reference to a contiguous section of a collection
- String slice: *&str*
- String literals are slices
  - Lenght is in byte not char
  - UTF-8 boundaries

```rust
fn main() {
    let message = String::from("Greetings from Earth!");
    println!("message is {message}");

    let last_word = &message[15..];
    println!("last_word is {last_word}");

    let planets = [1, 2, 3, 4, 5, 6, 7, 8]; // sorry, Pluto!
    let inner_planets: &[i32] = &planets[..4];
    println!("inner_planets are {inner_planets:?}");
}
```

### Slices as function parameters

```rust
fn main() {
    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message);
    println!("first_word is {}", first_word);
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space!
        }
    }

    &s // no spaces found; input is a single word
}
```