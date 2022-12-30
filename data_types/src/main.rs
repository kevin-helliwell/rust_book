use std::io;

fn main() {
    // // Numeric Operations

    let x = 2.0; // f64 (floating point 64 bit: double-precision float)

    let y: f32 = 3.0; // f32 (floating point 32 bit: single-precision float)

    let sum = 5 + 10; // addition

    let difference = 95.5 - 4.3; // subtraction

    let product = 4 * 30; // multiplication

    let quotient = 56.7 / 32.2; // division
    let truncated = -5 / 3; // Results in -1

    let remainder = 43 % 5; // mod

    // // The Boolean Type

    let t = true;

    let f: bool = false; // with explicit type annotation

    // // The Character Type

    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // // Compound Types

    // // Tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // // The Array Type

    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // identical to a = [3, 3, 3, 3, 3];

    // // Invalid Array Element Access

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    // This code will result in a runtime error if you pick an index outside the array (example of Rust memory safety)
}
