use colored::{self, Colorize};
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the Number!");
    let secret_number = rand::thread_rng().gen_range(1, 1001);
    // println!("The secret number is {}", secret_number);
    println!("Please input your guess.");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Falied to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("{}", "Too Small".red()),
            std::cmp::Ordering::Greater => println!("{}", "Too Big".red()),
            std::cmp::Ordering::Equal => {
                println!("{}", "You Win!".green());
                break;
            }
        }
    }
}
