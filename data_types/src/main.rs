use std::io;

fn main() {
    // // Numeric Operations

    let _x = 2.0; // f64 (floating point 64 bit: double-precision float)

    let _y: f32 = 3.0; // f32 (floating point 32 bit: single-precision float)

    let _sum = 5 + 10; // addition

    let _difference = 95.5 - 4.3; // subtraction

    let _product = 4 * 30; // multiplication

    let _quotient = 56.7 / 32.2; // division
    let _truncated = -5 / 3; // Results in -1

    let _remainder = 43 % 5; // mod

    // // The Boolean Type

    let _t = true;

    let _f: bool = false; // with explicit type annotation

    // // The Character Type

    let _c = 'z';
    let _z: char = 'Z'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // // Compound Types

    // // Tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, _y, _z) = tup;

    println!("The value of y is: {_y}");

    let _x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = _x.0;

    let _six_point_four = _x.1;

    let _one = _x.2;

    // // The Array Type

    let _a = [1, 2, 3, 4, 5];

    let _months = [
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

    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    let _a = [3; 5]; // identical to a = [3, 3, 3, 3, 3];

    // // Invalid Array Element Access

    let _a = [1, 2, 3, 4, 5];

    let _first = _a[0];
    let _second = _a[1];

    let _a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = _a[index];

    println!("The value of the element at index {index} is: {element}");

    // This code will result in a runtime error if you pick an index outside the array (example of Rust memory safety)
}
