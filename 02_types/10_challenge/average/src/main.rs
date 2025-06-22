fn main() {
    let a: f64 = 10.0;
    let b: f64 = 20.0;
    let c: f64 = 30.0;
    let average = (a + b + c) / 3.0;
    println!("The average of {a}, {b}, and {c} is {average}");

    assert_eq!(average, 20.0, "The average should be 20.0");
    println!("Test passed!");
}
