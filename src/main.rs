use std::io::{self, Read};

fn main() {
    let x = 5;
    println!("the value of x is {x}");
    // x = 6
    // the above code wouldn't compile since by default x is immutable

    // Defining a mutable variable
    let mut y = 5;
    println!("the value of y is {y}");
    y = 7;
    println!("the new value of y is {y}");

    // constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("They are {THREE_HOURS_IN_SECONDS} seconds in three hours");

    // Shadowing
    {
        let x = x * 2;
        println!("the value of x is {x}")
    }
    // since the scope has ended, the value should return to being the original value of x.
    println!("The value of x is {x}");

    // Changing the type of variable while shadowing
    let spaces = "    ";
    println!("the value of variable space {spaces}");
    let spaces: usize = spaces.len();
    // The bellow re-assignment would failure since they are of different type
    // spaces = spaces.len()
    println!("The value of variable spaces after shadowing {spaces}");

    //// DATA TYPE
    // We can convert a string into a number when possible
    // let guess: u32 = "45".parse().expect("Not a number");

    // // MATHEMATICAL OPERATIONS
    // // addition
    // let sum = 5 + 10;

    // // subtraction
    // let different = 5 - 10;
    // // multiplication
    //let product = 300 * 5988;

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient {quotient}");
    let truncated = -5 / 3;
    println!("truncated value {truncated}");

    // reminder
    let remainder = 43 % 5;
    println!("remainder {remainder}");

    // Tuple: a group of variety of types: has a fixed length once declared.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // get element of tuple by using pattern matching destructure
    let (x, y, z) = tup;
    println!("In our tuple we have {x}, {y}, {z}");

    // We can also access tuple by using period (.) followed by index of the value
    let first_value = tup.0;
    println!("At index zero we have {first_value}");

    // ARRAY
    let a = [1, 3, 4, 5, 6];
    // unlike other programming language, mixing type in an array is not allowed in Rust
    //et b = [3, 4, "abayo"];
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

    // You can define an array like this
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // i32 it the type of elements in the array
    // 5 is the length of the array

    let a = [3; 5]; // this is the same as write let a = [3,3,3,3,3]

    // reading element of the array
    println!(
        "The value at position zero {first_number}",
        first_number = a[0]
    );

    // trying to access element of an array past the end of the array

    let a = [1, 2, 3, 4, 5, 6, 7];

    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    println!(
        "The value of the element at index {} is: {}",
        index, a[index]
    )
}
