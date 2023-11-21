use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {

    guess_number_game();
}

fn guess_number_game() {
    

    let screat_numer = rand::thread_rng().gen_range(1..=100);

    loop {
        
        println!("please guess a number: ");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        let guess: usize = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed number: {}", guess);

        match guess.cmp(&screat_numer) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }

    }

}