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
