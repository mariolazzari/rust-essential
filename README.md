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

### Clhallenge: trim spaces

```rust
fn trim_spaces(s: &str) -> &str {
    // locate the first non-space character
    let mut start = 0;
    for (index, character) in s.chars().enumerate() {
        if character != ' ' {
            start = index;
            break;
        }
    }

    // search in reverse to locate the last non-space character
    let mut end = 0;
    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' {
            end = s.len() - index;
            break;
        }
    }

    &s[start..end]
}
```

## Modules

### Standard library

[Documentation](https://doc.rust-lang.org/std/)

### Standard input

```rust
use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Enter a message:");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {}", buffer);
}
```

### Parse strings

```rust
use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Enter a message:");
    io::stdin().read_line(&mut buffer);

    let number: i32 = buffer.trim().parse().unwrap();
    println!("number + 1 is {}", number + 1);
}
```

### Crates

[Crates site](https://crates.io/)

- Collection of Rust code
- Binary crates
- Libraries

```rust
use rand::prelude::*;

fn main() {
    let number = random::<f64>();
    println!("number is {}", number);

    let number = thread_rng().gen_range(1..11);
    println!("number is {}", number);
}
```

```toml
[package]
name = "crates"
version = "0.1.0"
authors = ["Barron Stone <omitted@email.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.0"
```

### Challenge

```rust
use rand::prelude::*;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input line.");
        let guess: u32 = guess.trim().parse().expect("Failed to parse the guess.");

        if guess > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou got it! The secret number was {}.", secret_number);
            break;
        }
    }
}
```

## Input / Output

### Command line args

- [Docs](https://doc.rust-lang.org/std/env/index.html)
- std:env
- Arg[0] is program name.

```rust
use std::env;

fn main() {
    if env::args().len() <= 2 {
        println!("Program requires as least 2 arguments.");
        return;
    }

    for (index, argument) in env::args().enumerate() {
        println!("argument {} is {}", index, argument);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);
}
```

### Reading files

- [fs](https://doc.rust-lang.org/std/fs/index.html)
- [path](https://doc.rust-lang.org/std/path/struct.Path.html)

```rust
use std::fs;

fn main() {
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("contents is {}", contents);

    for line in contents.lines() {
        println!("line is {}", line);
    }

    let contents = fs::read("planets.txt").unwrap();
    println!("contents is {:?}", contents);
}
```

### Write files

- [write](https://doc.rust-lang.org/std/io/trait.Write.html)

```rust
use std::fs;
use std::io::prelude::*;

fn main() {
    let mut speech = String::new();
    speech.push_str("We choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.");

    fs::write("speech.txt", speech);

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("planets.txt")
        .unwrap();
    file.write(b"\nPluto");
}
```

### Challenge

```rust
use std::env;
use std::fs;

fn main() {
    if env::args().len() < 2 {
        eprintln!("Program requires two arguments: <file path> <search name>");
        std::process::exit(1);
    }
    let file_path = env::args().nth(1).unwrap();
    let search_name = env::args().nth(2).unwrap();

    for line in fs::read_to_string(file_path).unwrap().lines() {
        if line == search_name {
            println!("{} did walk on the Moon!", search_name);
            return;
        }
    }

    println!("{} did NOT walk on the Moon... YET!", search_name);
}
```

## Structs

```rust
#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0,
    };
    println!("name is {}", vehicle.name);

    vehicle.name = String::from("Atlantis");
    println!("vehicle is {:?}", vehicle);
}
```

### Struct update

```rust
#[derive(Debug, Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0,
    };

    let vehicle2 = Shuttle { ..vehicle.clone() };
    vehicle.crew_size = 6;

    println!("vehicle is {:?}", vehicle);
    println!("vehicle2 is {:?}", vehicle2);
}
```

### Struct methods

- 1st parameter must be *&self*

```rust
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }
}

fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 0.0,
    };

    let vehicle_name = vehicle.get_name();
    println!("vehicle_name is {}", vehicle_name);

    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant is {}", vehicle.propellant);
}
```

### Associated functions

- No *&self* parameter

```rust
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0
        }
    }
}

fn main() {
    let mut vehicle = Shuttle::new("Endeavour");
    let mut vehicle2 = Shuttle::new("Discovery");

    let vehicle_name = vehicle.get_name();
    println!("vehicle_name is {}", vehicle_name);

    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant is {}", vehicle.propellant);
}
```

### Tuple structs

```rust
struct Color(u8, u8, u8); // RGB
struct Point(u8, u8, u8); // XYZ

fn get_y(p: Point) -> u8 {
    p.1
}

fn main() {
    let red = Color(255, 0, 0);
    println!("First value is {}", red.0);

    let coord = Point(4, 5, 6);
    let y = get_y(coord);
    println!("y is {}", y);
}
```

## Generic types

### Generic struct definition

```rust
#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

fn main() {
    let rect = Rectangle {
        width: 1u8,
        height: 3u16,
    };
    println!("rect is {:?}", rect);
}
```

### Generic method definitions

```rust
#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 1u8,
        height: 3u8,
    };
    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
    println!("perimeter is {}", rect.get_perimeter());
}
```

### Generic funciton definitions

```rust
fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("biggest is {}", get_biggest(1.2, 2.3));
}
```

### Box data type

```rust
use std::mem;

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0,
    };
    println!(
        "vehicle size on stack: {} bytes",
        mem::size_of_val(&vehicle)
    );

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    println!(
        "boxed_vehicle size on stack: {} bytes",
        mem::size_of_val(&boxed_vehicle)
    );
    println!(
        "boxed_vehicle size on heap: {} bytes",
        mem::size_of_val(&*boxed_vehicle)
    );

    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!(
        "unboxed_vehicle size on stack: {} bytes",
        mem::size_of_val(&unboxed_vehicle)
    );
}
```

### Challenge

```rust
fn sum_boxes<T: std::ops::Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    Box::new(*a + *b)
}

fn main() {
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(sum_boxes(one, two), Box::new(3));

    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi, e), 5.85987);

    println!("Tests passed!");
}
```

## Traits

- Interfaces in OOP

### Implement traits

```rust
struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32, // miles
}

trait Description {
    fn describe(&self) -> String;
}

impl Description for Satellite {
    fn describe(&self) -> String {
        format!(
            "the {} flying at {} miles per second!",
            self.name, self.velocity
        )
    }
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!(
            "the {} flying {} miles high with {} crew members aboard!",
            self.name, self.altitude, self.crew_size
        )
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254,
    };
    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());
}
```

### Default trait

```rust
struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32, // miles
}

trait Description {
    fn describe(&self) -> String {
        String::from("an object flying through space!")
    }
}

impl Description for Satellite {}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!(
            "the {} flying {} miles high with {} crew members aboard!",
            self.name, self.altitude, self.crew_size
        )
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254,
    };
    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());
}
```

### Derive traits

- Default implementation of most common used traits

```rust
#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42,
    };
    println!("hubble == gps is {}", hubble == gps);
    println!("hubble > gps is {}", hubble > gps);
}
```

### Trait bounds

```rust
#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42,
    };
    println!("hubble == gps is {}", hubble == gps);
    println!("hubble > gps is {}", hubble > gps);
}
```

### Multiple traits bound

```rust
use std::fmt;

// fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U) {
fn compare_and_print<T, U>(a: T, b: U)
where
    T: fmt::Display + PartialEq + From<U>,
    U: fmt::Display + PartialEq + Copy,
{
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is NOT equal to {}", a, b);
    }
}

fn main() {
    // compare_and_print(1.0, "one");
    compare_and_print(1.1, 1);
}
```

### Return values

```rust
use std::fmt;

fn get_displayable(choice: bool) -> impl fmt::Display {
    if choice {
        13
    } else {
        "thirteen"
    }
}

fn main() {
    println!("output is {}", get_displayable(true));
}
```

### Challenge

```rust
use std::fmt;

struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} flying at {} miles per hour",
            self.name, self.velocity
        )
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    println!("hubble is {}", hubble);
}
```

## Lifetimes

### Borrow checker

- Compares scopes to determine whether all borrows are valid


```rust
fn main() {
    let propellant;
    {
        let rp1 = String::from("RP-1");
        propellant = &rp1;
        println!("propellant is {}", propellant); // correct
    }
    //    println!("propellant is {}", propellant); -> error!
}
```

### Lifetime annotation

- Explicity defines a lifetime for function paramenters
- Must starts with *'*
- Conventionally single letters in lowercase

```rust
fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 = String::from("LNG");
        result = best_fuel(&propellant1, &propellant2);
    }
    println!("result is {}", result);
}
```

### Multiple lifetimes

```rust
fn best_fuel<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x
    }
}

fn main() {
    let result;
    let propellant1 = String::from("RP-1");
    {
        let propellant2 = String::from("LNG");
        result = best_fuel(&propellant1, &propellant2);
    }
    println!("result is {}", result);
}
```

### Lifetime elision

- Rules for the compiler in order to handle lifetimes

```rust
fn main() {
    let message = String::from("Greetings from Earth!");
    let first_word = get_first_word(&message);
    println!("first_word is {}", first_word);
}

fn get_first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space!
        }
    }

    &s // no spaces found; input is a single word
}
```

### Struct lifetime

```rust
struct Shuttle<'a> {
    name: &'a str,
}

impl<'a, 'b> Shuttle<'a> {
    fn send_transmission(&'a self, msg: &'b str) -> &'b str {
        println!("Transmitting message: {}", msg);
        msg
    }
}

fn main() {
    let vehicle = Shuttle { name: "Endeavour" };

    let sender = vehicle.send_transmission("Greetings from orbit!");
    println!("sender is {}", sender);
}
```

### Static lifetime

- References available for entire duration of a program

```rust
let s: &'static str = "Static lifetime";
```

## Enums

### Define enum

```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),             // radius
    Rectangle(f64, f64),     // width, height
    Triangle(f64, f64, f64), // sides a, b, c
}

fn main() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {:?}", my_shape);
}
```

### Match

```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),             // radius
    Rectangle(f64, f64),     // width, height
    Triangle(f64, f64, f64), // sides a, b, c
}

fn main() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {:?}", my_shape);

    match my_shape {
        Shape::Circle(r) => println!("Circle with radius {}", r),
        Shape::Rectangle(w, h) => println!("{} x {} Rectangle", w, h),
        Shape::Triangle(a, b, c) => println!("Triangle with sides {}, {}, {}", a, b, c),
    }
}
```

### Match with default placeholder

```rust
fn main() {
    let my_number = 3u8;

    let result = match my_number {
        0 => "zero",
        _ => "one",
        2 => "two",
        3 => "three",
        _ => {
            println!("{} did not match", my_number);
            "something else"
        }
    };
    println!("result is {}", result);
}
```

### Enum methods

```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),             // radius
    Rectangle(f64, f64),     // width, height
    Triangle(f64, f64, f64), // sides a, b, c
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}

fn main() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {:?}", my_shape);

    let perimeter = my_shape.get_perimeter();
    println!("perimeter is {}", perimeter);
}
```

### Enum methods

```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),             // radius
    Rectangle(f64, f64),     // width, height
    Triangle(f64, f64, f64), // sides a, b, c
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}

fn main() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {:?}", my_shape);

    let perimeter = my_shape.get_perimeter();
    println!("perimeter is {}", perimeter);
}
```

### Option<T> enum

```rust
fn main() {
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(5);
    let number = number.unwrap_or(&0) + 1;
    println!("number is {:?}", number);
}
```

### Matching Option<T>

```rust
fn main() {
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(2);
    let number = match number {
        Some(number) => number + 1,
        None => 0
    };
    println!("number is {:?}", number);
}
```

### If let

```rust
fn main() {
    let number = None;

    if let Some(13) = number {
        println!("thirteen");
    }
}
```

### Challenge location

```rust
enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64) // latitude, longitude
}

impl Location {
    fn display(&self) {
        match *self {
            Location::Unknown => println!("Unknown Location"),
            Location::Anonymous => println!("Anonymous Location"),
            Location::Known(lat, lon) => println!("{}, {}", lat, lon)
        }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();
}
```

## Errors

### Unrecoverable errors

```rust
fn main() {
    //panic!("Houston, we've had a problem.");

    let countdown = [5, 4, 3, 2, 1, 0];

    for count in countdown.iter() {
        println! {"T-minus {}", count};
        let x = 1 / count; // this won't end well
    }
}
```

### Result<T, E> enum

```rust
use std::fs;

fn main() {
    let contents = fs::read_to_string("the_ultimate_question.txt")
        .expect("Nobody know the ultimate question!");
    println!("contents is: {:?}", contents);
}
```

###
