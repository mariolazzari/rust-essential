fn main() {
    let result = square(13);
    println!("result is {:?}", result);
}

fn square(x: i32) -> (i32, i32) {
    println!("squaring {}", x);
    return (x, x * x);
    // println!("End of function"); // This line will never be executed because of the return statement above.
}
