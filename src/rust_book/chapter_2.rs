use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    println!("CHAPTER 2");
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // With proper error handling, sort of
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
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
}
