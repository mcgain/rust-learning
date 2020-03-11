extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("the secret number is {}", secret_number);
    loop {
        println!("Enter your guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("You need to enter something");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue
            },
        };
        println!("You entered {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Correct");
                break
            },
            Ordering::Less => println!("Too low!"),
        };
    }

}
