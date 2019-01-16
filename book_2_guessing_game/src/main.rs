use std::cmp::Ordering;
use std::io;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    println!("Guess the number!");
    println!("Please input your guess");

    let secret_number = thread_rng().gen_range(1, 101);
    let mut old_guess = 0;
    let mut suggest;

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number!");
                continue;
            }
        };

        //    io::stdin().read_line(&guess)  //Here using &guess instead of &mut guess will fail
        //        .expect("Failed to read line");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{} is too small!", guess);
                suggest = (guess - old_guess).abs() / 2 + guess;
                println!("I would suggest {}", suggest);
                old_guess = guess;
            }        //if suggest == old_guess; suggest += 1
            Ordering::Greater => {
                println!("{} is too big!", guess);
                suggest = guess - (guess - old_guess).abs() / 2;
                println!("I would suggest {}", suggest);
                old_guess = guess;
            }       // if sugget == old_guess; suggest -=1
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
