use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_number = thread_rng().gen_range(1..101);
    loop {
        println!("guess the number you scummy prick");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: i32 = guess.trim().parse().expect("Please type number");

        println!("you guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You guessed the right one");
                break;
            }
        }
    }
}
