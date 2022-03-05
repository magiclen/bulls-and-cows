use std::io::{self, Write};

use bulls_and_cows::parser::ABParser;
use bulls_and_cows::play::players::{ComputerGuesser, ComputerQuestioner, Guesser, Questioner};
use bulls_and_cows::{Host, HostError};

use once_cell::sync::Lazy;

const COM_THINKING_DELAY: u64 = 750;

static AB_PARSER: Lazy<ABParser> = Lazy::new(ABParser::new);

pub enum GameError {
    HostError(HostError<u8>),
    ABIncorrect((usize, usize)),
    ABError(String),
    GuessIncorrect,
}

pub struct CLIUserQuestioner {
    letter_length: usize,
}

impl CLIUserQuestioner {
    pub fn new(letter_length: usize) -> CLIUserQuestioner {
        CLIUserQuestioner {
            letter_length,
        }
    }
}

impl Questioner<u8> for CLIUserQuestioner {
    type Error = GameError;

    fn make_new_question(&mut self) {
        print!("Please make a question in your mind. Press Enter to continue...");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut String::new()).expect("Failed to read line");
    }

    fn answer(&self, _answer: &[u8]) -> Result<(usize, usize), Self::Error> {
        print!("> ");

        let mut reply = String::new();

        io::stdout().flush().expect("Failed to flush");

        io::stdin().read_line(&mut reply).expect("Failed to read line");

        match AB_PARSER.parse(&reply) {
            Some((a, b)) => {
                if a + b > self.letter_length {
                    Err(GameError::ABIncorrect((a, b)))
                } else {
                    Ok((a, b))
                }
            }
            None => Err(GameError::ABError(reply)),
        }
    }
}

pub struct CLIUserGuesser {
    guess_times: usize,
    letter_length: usize,
}

impl CLIUserGuesser {
    pub fn new(letter_length: usize) -> CLIUserGuesser {
        CLIUserGuesser {
            guess_times: 0,
            letter_length,
        }
    }
}

impl Guesser<u8> for CLIUserGuesser {
    type Error = GameError;

    fn get_guess_times(&self) -> usize {
        self.guess_times
    }

    fn set_guess_times(&mut self, guess_times: usize) {
        self.guess_times = guess_times
    }

    fn add_condition(&mut self, _guess: &[u8], _reply: (usize, usize)) {
        // do nothing
    }

    fn guess(&self) -> Result<Vec<u8>, Self::Error> {
        let mut answer = Vec::with_capacity(self.letter_length);

        print!("> ");

        let mut guess = String::new();

        io::stdout().flush().expect("Failed to flush");

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess = guess.trim();

        if guess.len() != self.letter_length {
            return Err(GameError::GuessIncorrect);
        }

        for i in 0..self.letter_length {
            match guess.get(i..(i + 1)) {
                Some(o) => {
                    match o.parse() {
                        Ok(n) => answer.push(n),
                        Err(_) => {
                            return Err(GameError::GuessIncorrect);
                        }
                    }
                }
                None => {
                    return Err(GameError::GuessIncorrect);
                }
            }
        }

        Ok(answer)
    }
}

pub enum QuestioningPlayer {
    CLIUser(CLIUserQuestioner),
    Computer(ComputerQuestioner<u8>),
}

impl Questioner<u8> for QuestioningPlayer {
    type Error = GameError;

    fn make_new_question(&mut self) {
        match self {
            QuestioningPlayer::CLIUser(p) => p.make_new_question(),
            QuestioningPlayer::Computer(p) => p.make_new_question(),
        }
    }

    fn answer(&self, answer: &[u8]) -> Result<(usize, usize), Self::Error> {
        match self {
            QuestioningPlayer::CLIUser(p) => p.answer(answer),
            QuestioningPlayer::Computer(p) => p.answer(answer).map_err(GameError::HostError),
        }
    }
}

pub enum GuessingPlayer {
    CLIUser(CLIUserGuesser),
    Computer(ComputerGuesser<u8>),
    NotSet,
}

impl Guesser<u8> for GuessingPlayer {
    type Error = GameError;

    fn get_guess_times(&self) -> usize {
        match self {
            GuessingPlayer::CLIUser(p) => p.get_guess_times(),
            GuessingPlayer::Computer(p) => p.get_guess_times(),
            _ => unreachable!(),
        }
    }

    fn set_guess_times(&mut self, guess_times: usize) {
        match self {
            GuessingPlayer::CLIUser(p) => p.set_guess_times(guess_times),
            GuessingPlayer::Computer(p) => p.set_guess_times(guess_times),
            _ => unreachable!(),
        }
    }

    fn add_condition(&mut self, guess: &[u8], reply: (usize, usize)) {
        match self {
            GuessingPlayer::CLIUser(p) => p.add_condition(guess, reply),
            GuessingPlayer::Computer(p) => p.add_condition(guess, reply),
            _ => unreachable!(),
        }
    }

    fn guess(&self) -> Result<Vec<u8>, Self::Error> {
        match self {
            GuessingPlayer::CLIUser(p) => p.guess(),
            GuessingPlayer::Computer(p) => p.guess().map_err(GameError::HostError),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let mut line = String::new();

    loop {
        let letter_length: usize;

        loop {
            print!("Decide the digit length [1-10]: ");

            io::stdout().flush().expect("Failed to flush");

            line.clear();

            io::stdin().read_line(&mut line).expect("Failed to read line");

            let line = line.trim();

            letter_length = match line.parse() {
                Ok(letter_length) => {
                    if !(1..=10).contains(&letter_length) {
                        continue;
                    }

                    letter_length
                }
                Err(_) => continue,
            };

            break;
        }

        let host = Host::build(bulls_and_cows::Letters::generate_numeric_letters(), letter_length)
            .expect("Failed to initialize a new game");

        let mut qp;
        let mut gp;

        loop {
            println!("What are you want to be?\n    1. the questioner\n    2. the guesser\n    3. a spectator");

            print!("[1-3] ");

            io::stdout().flush().expect("Failed to flush");

            line.clear();

            io::stdin().read_line(&mut line).expect("Failed to read line");

            let line = line.trim();

            match line.parse() {
                Ok(line) => {
                    match line {
                        1 => {
                            qp = QuestioningPlayer::CLIUser(CLIUserQuestioner::new(letter_length));
                            gp = GuessingPlayer::Computer(ComputerGuesser::new(
                                &host,
                                COM_THINKING_DELAY,
                            ));
                        }
                        2 => {
                            qp = QuestioningPlayer::Computer(ComputerQuestioner::new(
                                host,
                                COM_THINKING_DELAY,
                            ));
                            gp = GuessingPlayer::CLIUser(CLIUserGuesser::new(letter_length));
                        }
                        3 => {
                            gp = GuessingPlayer::Computer(ComputerGuesser::new(
                                &host,
                                COM_THINKING_DELAY,
                            ));
                            qp = QuestioningPlayer::Computer(ComputerQuestioner::new(
                                host,
                                COM_THINKING_DELAY,
                            ));
                        }
                        _ => continue,
                    }
                }
                Err(_) => continue,
            }

            break;
        }

        qp.make_new_question();

        println!("A new question is done. The guesser can guess now.");

        loop {
            let (guess, ab) = 'guess: loop {
                match gp.guess() {
                    Ok(guess) => {
                        let guess_times = gp.get_guess_times();
                        gp.set_guess_times(guess_times + 1);

                        let mut s = String::with_capacity(letter_length);

                        for e in guess.iter() {
                            s.push((e + b'0') as char);
                        }

                        println!("Guesser: {}", s);

                        io::stdout().flush().expect("Failed to flush");

                        break loop {
                            match qp.answer(&guess) {
                                Ok(ab) => break (guess, ab),
                                Err(b) => {
                                    match b {
                                        GameError::HostError(_) => unreachable!(),
                                        GameError::GuessIncorrect => {
                                            println!("Questioner: Are you kidding?");
                                            continue 'guess;
                                        }
                                        GameError::ABIncorrect((a, b)) => {
                                            println!("Questioner: {}A{}B... just kidding.", a, b);
                                        }
                                        GameError::ABError(s) => {
                                            println!("Questioner: {}... just kidding.", s.trim());
                                        }
                                    }

                                    continue;
                                }
                            }
                        };
                    }
                    Err(_) => {
                        println!("Wrong format!");

                        continue;
                    }
                }
            };

            println!("Questioner: {}A{}B", ab.0, ab.1);

            if ab.0 == letter_length {
                let times = gp.get_guess_times();

                if times > 1 {
                    println!("The guesser guessed {} times and finally wins!", times);
                } else {
                    println!("The guesser guessed only one time and wins!");
                }

                break;
            }

            gp.add_condition(&guess, ab)
        }
    }
}
