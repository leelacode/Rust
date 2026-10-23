use std::{cmp::Ordering, io};

use rand::RngExt;

fn main() {
    println!("Hello, world!");

    let magic_number = rand::rng().random_range(1..=100);
    println!("Random number is {}.", magic_number);

    loop {
        println!("Input your guess : ");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = guess.trim().parse().expect("Please type a valid number!");

        match guess.cmp(&magic_number) {
            Ordering::Less => println!("too small!!"),
            Ordering::Greater => println!("too big!!"),
            Ordering::Equal => {
                println!("you are correct!!");
                break;
            }
        }
    }
}
