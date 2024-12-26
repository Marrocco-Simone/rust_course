use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

/// guess the random name created
fn main() {
    println!("Guess the number!");
    println!("Type quit to quit, or solution to know the response");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        print!("Please enter a number: ");
        io::stdout().flush().expect("Couldn't flush stdout");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.as_str().trim() {
            "quit" => {
                println!("Bye!");
                break;
            }
            "solution" => {
                println!("The secret number was {secret_number}");
                break;
            }
            _ => (),
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("Numbers only!");
                println!();
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }

        println!();
    }
}
