extern crate bulls_and_cows;

use std::io::{self, Write};

use bulls_and_cows::Host;

const LETTER_LENGTH: usize = 4; // the reasonable range is 1~10

fn main() {
    let mut host = Host::build_with_random_answer(bulls_and_cows::Letters::generate_numeric_letters(), LETTER_LENGTH).expect("Failed to initialize a new game");

    let mut guess = String::new();

    loop {
        println!("A new question is done. The guesser can make a guess now.");

        loop {
            print!("> ");

            io::stdout().flush().expect("Failed to flush");

            guess.clear();

            io::stdin().read_line(&mut guess).expect("Failed to read line");

            let guess = guess.trim();

            if guess.len() != LETTER_LENGTH {
                println!("Wrong format!");

                continue;
            }

            let mut answer = Vec::with_capacity(LETTER_LENGTH);

            for i in 0..LETTER_LENGTH {
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

                    if a == LETTER_LENGTH {
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

        host.renew_with_random_answer(LETTER_LENGTH).expect("Failed to renew");
    }
}