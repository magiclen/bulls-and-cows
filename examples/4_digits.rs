use std::io::{self, Write};

use bulls_and_cows::Host;

const LETTER_LENGTH: usize = 4; // the reasonable range is 1~10

fn parse_guess(guess: &str) -> Option<Vec<u8>> {
    if guess.len() != LETTER_LENGTH {
        return None;
    }

    let mut answer = Vec::with_capacity(LETTER_LENGTH);

    for digit in guess.bytes() {
        if !digit.is_ascii_digit() {
            return None;
        }

        answer.push(digit - b'0');
    }

    Some(answer)
}

fn main() {
    let mut host = Host::build_with_random_answer(
        bulls_and_cows::Letters::generate_numeric_letters(),
        LETTER_LENGTH,
    )
    .expect("Failed to initialize a new game");

    let mut guess = String::new();

    loop {
        println!("A new question is done. The guesser can make a guess now.");

        loop {
            print!("> ");

            io::stdout().flush().expect("Failed to flush");

            guess.clear();

            io::stdin().read_line(&mut guess).expect("Failed to read line");

            let guess = guess.trim();

            let Some(answer) = parse_guess(guess) else {
                println!("Wrong format!");

                continue;
            };

            match host.answer(&answer) {
                Ok((a, b)) => {
                    println!("{}A{}B", a, b);

                    if a == LETTER_LENGTH {
                        println!("Congratulations!");
                        println!();
                        break;
                    }
                },
                Err(_) => {
                    println!("Wrong format!");

                    continue;
                },
            }
        }

        host.renew_with_random_answer(LETTER_LENGTH).expect("Failed to renew");
    }
}
