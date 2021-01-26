use rand::Rng;
use std::cmp::Ordering;
// the input/output functionality from the standard library
use std::io;

fn main() {
    println!("Guess the number!");

    // generates a random number 1-100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        // print statement
        println!("Please input your guess.");

        // Creates a mutable variable that is currently bound to a new, empty instance of a String
        let mut guess = String::new();

        // stdin() function handles standard input
        io::stdin()
        /* calls the read_line method on standard input handle to get input from the user. The string argument needs to be mutable so the method can change the string’s content by adding the user input. The & indicates the argument is a reference */
            .read_line(&mut guess)
            // built in error handling 
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        
        // create different outcomes based on guess
        // compares guess and secret_number
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
