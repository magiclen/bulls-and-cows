extern crate bulls_and_cows;

use std::io::{self, Write};

const DIGIT_LENGTH: usize = 4; // the reasonable range is 1~10

fn main() {
    let mut host = bulls_and_cows::Host::build_with_random_answer(bulls_and_cows::Letters::generate_numeric_letters(), DIGIT_LENGTH).expect("Failed to initialize a new game");

    let mut guess = String::new();

    loop {
        println!("A new 1A2B question is done. You can make a guess now.");

        loop {
            print!("> ");

            io::stdout().flush().unwrap();

            guess.clear();

            io::stdin().read_line(&mut guess).expect("Failed to read line");

            let guess = guess.trim();

            if guess.len() != DIGIT_LENGTH {
                println!("Wrong format!");

                continue;
            }

            let mut answer = Vec::with_capacity(DIGIT_LENGTH);

            for i in 0..DIGIT_LENGTH {
                match guess.get(i..(i + 1)) {
                    Some(o) => {
                        match o.parse() {
                            Ok(n) => answer.push(n),
                            Err(_) => {
                                println!("Wrong format!");

                                continue;
                            }
                        }
                    }
                    None => {
                        println!("Wrong format!");

                        continue;
                    }
                }
            }

            match host.answer(&answer) {
                Ok((a, b)) => {
                    println!("{}A{}B", a, b);

                    if a == DIGIT_LENGTH {
                        println!("Congratulations!");
                        println!();
                        break;
                    }
                }
                Err(_) => {
                    println!("Wrong format!");

                    continue;
                }
            }
        }

        host.renew_with_random_answer(DIGIT_LENGTH).expect("Failed to renew");
    }
}