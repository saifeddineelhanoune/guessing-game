use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {

    println!("{}", "Welcome to the guessing game!".on_bright_blue());

    let secret = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", secret.green());

    loop {
        println!("{}", "Please input your guess.".on_blue());
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let guess : u32 = guess.trim().parse().expect("Please Type a number");
        println!("you guessed {}", guess);
        match guess.cmp(&secret) {
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Equal => {
                println!("{}", "You Win".green());
                break;
            }
        }  
    }

}
