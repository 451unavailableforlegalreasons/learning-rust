use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;


fn main() {

    println!("Number guessing game");
    let secret_number: u32 = rand::thread_rng().gen_range(1..100);
    println!("Secret number: {}", secret_number);

    let mut string_guess: String = String::new();
    loop {
        println!("Please enter a number !");
        string_guess.clear();
        io::stdin().read_line(&mut string_guess).expect("Invalid line parsing error");
        let guess: u32 = match string_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&secret_number) {
            Ordering::Less => {println!("{}", "Too small!".red())},
            Ordering::Greater => {println!("{}", "Too big!".red())},
            Ordering::Equal => {
                println!("{}", "You won!".green());
                break;
            },

        }




    }

    println!("Bye bye");
}
