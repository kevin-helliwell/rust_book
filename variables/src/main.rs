fn main() {
    // Example 1 (x is immutable by default, hence mut keyword)

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Example 2 (inner scope compared to outer scope)

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}")
    }

    println!("The value of x is: {x}");

    // Example 3 (doesn't compile; mismatched types)

    // let mut spaces = "   "; // string
    // spaces = spaces.len(); // number
}
