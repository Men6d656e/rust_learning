use rand::prelude::*;
use std::io;

fn main() {
    let guess_list = ["grapes", "banana", "orange", "apple"];
    let mut rng = rand::rng();
    let index = rng.gen_range(0..guess_list.len());
    let random_fruit = guess_list[index];
    println!("random_fruit:{}", random_fruit);
    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let fruit_selected = input.trim().to_lowercase();
                println!("Fruit_selected:{}", fruit_selected);
                if !guess_list.contains(&fruit_selected.as_str()) {
                    println!("Fruits entered does not found.")
                }
                if guess_checker(&fruit_selected, random_fruit) {
                    println!("You are winner");
                    break;
                } else {
                    println!("Retry")
                }
            }
            Err(error) => {
                println!("Error:{}", error)
            }
        }
    }
}

fn guess_checker(fruit_selected: &str, random_fruit: &str) -> bool {
    return fruit_selected == random_fruit;
}
