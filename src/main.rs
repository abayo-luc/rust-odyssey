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
    println!("The value of variable spaces after shadowing {spaces}")
}
