use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("src {secret_number}");

    loop {
        println!("Please input your guess:");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Plese Enter valid input");
                continue;
            }
        };
        println!("You Guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
            Ordering::Less => println!("Too small!"),
        }
    }
}
