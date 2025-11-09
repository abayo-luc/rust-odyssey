use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // print to the console
    println!("Welcome to Guess The Number game!");

    // capture unsigned number that occupy 32 bit of space
    // Randomly generate the winning secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess:");

        // create a mutable string variable named guess and assign to to an empty string
        let mut guess = String::new();

        // assign the input to the guess string
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        // check if it is an number and use switch case syntax
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        // Use switch case to prompt a user to enter more value
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
