fn main() {
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first_letter is {first_letter}");

    let numbers: [i32; 5];
    // Initialize an array of 5 integers, all set to 0
    numbers = [0; 5];
    // usize is an unsigned integer type, used for indexing
    let index: usize = numbers.len();
    println!("last number is {}", numbers[index - 1]);
}
