extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!"); 
    let mut repeat = String::new(); // Create a variable of string type
    let repeat_check = String::from("y\n");//Create a variable of string type containing 

    //println!("The secret number is: {}", secret_number);
    loop {
        let secret_number = rand::thread_rng().gen_range(1, 101);
        loop {
            println!("Please input your guess: ");
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }

        println!("Press y to play again and any other key to exit: ");

        io::stdin()
            .read_line(&mut repeat)
            .expect("Failed to read line");
        if repeat != repeat_check {
            break;
        }
    }
}
