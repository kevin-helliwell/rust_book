// // Functions

// // snake-case convention for function/variable names

// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }

// // Parameters/Arguments

// fn main() {
//     another_function(5);
// }

// // in function signatures (x: i32) you have to declare the type of each parameter

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }


// // when defining multiple parameters, separate the declarations with commas like below (print_labeled_measurement):

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// // Statements & Expressions

// // this entire function block is a statement

// fn main() {
//     let y = 6; // this line is also a statement
// }
// // important: statements do not return values!

// // important: expressions evaluate to a value! expressions can also be part of statements!
// // Calling a macro or a function is an example of an expression
// // A new scope block created with curly brackets is also an expression: example below!

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     }; // expression

//     println!("The value of y is: {y}");
// }

// // Functions with Return Values

// // We don’t name return values, but we must declare their type after an arrow (->).
// // In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. 
// // You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.
// // Example of a function that returns a value below:

// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // adding a semicolon (;) here will make the code error out due to mismatched types
}