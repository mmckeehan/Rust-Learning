use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Generate random number between 1-100
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);
    
    // println!("The secret number is {}", secret_number);
    // intake players guess
    loop{
        println!("Please input your guess:");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
            let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
    
        println!("You guessed: {}", guess);
    
        // Compare to number generated
        match guess.cmp(&secret_number) {
            // let player knwo if too high, low, or correct
            // Generate congrats message.
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!!!!!");
                break;
            }
        }
    }


}
