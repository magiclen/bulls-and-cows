use std::hash::Hash;
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashSet;


use super::Questioner;

use ::{Host, HostError};
use ::permutohedron::Heap;
use ::random_pick;

use play::players::player::Guesser;

/// A questioner controlled by a computer.
pub struct ComputerQuestioner<T: Eq + Hash + Clone> {
    host: Host<T>,
    thinking_delay: u64,
}

impl<T: Eq + Hash + Clone> ComputerQuestioner<T> {
    /// Create a new computer player as a questioner. The `thinking_delay` is a value which simulates the time in milliseconds that a human player needs to take to think.
    pub fn new(host: Host<T>, thinking_delay: u64) -> ComputerQuestioner<T> {
        ComputerQuestioner {
            host,
            thinking_delay,
        }
    }
}

impl<T: Eq + Hash + Clone> Questioner<T> for ComputerQuestioner<T> {
    type Error = HostError<T>;

    fn make_new_question(&mut self) {
        thread::sleep(Duration::from_millis(self.thinking_delay));

        let answer_length = self.host.get_answer_length();

        self.host.renew_with_random_answer(answer_length).unwrap();
    }

    fn answer(&self, answer: &[T]) -> Result<(usize, usize), Self::Error> {
        thread::sleep(Duration::from_millis(self.thinking_delay));
        self.host.answer(answer)
    }
}

/// A guesser controlled by a computer.
pub struct ComputerGuesser<T: Eq + Hash + Clone> {
    letters: HashSet<T>,
    letter_length: usize,
    possible_elements_table: Vec<Vec<T>>,
    guess_times: usize,
    thinking_delay: u64,
}

impl<T: Eq + Hash + Clone> ComputerGuesser<T> {
    /// Create a new computer player as a guesser. The `thinking_delay` is a value which simulates the time in milliseconds that a human player needs to take to think.
    pub fn new(host: &Host<T>, thinking_delay: u64) -> ComputerGuesser<T> {
        let letters = host.get_letters().clone();
        let letter_length = host.get_answer_length();
        let possible_elements_table = ComputerGuesser::make_possible_elements_table(&letters, letter_length);

        ComputerGuesser {
            letters,
            letter_length,
            possible_elements_table,
            guess_times: 0,
            thinking_delay,
        }
    }

    fn make_possible_elements_table(letters: &HashSet<T>, letter_length: usize) -> Vec<Vec<T>> {
        let letters_length = letters.len();

        let mut capacity = 1;

        for _ in 0..letter_length {
            capacity *= letters_length - letter_length;
        }

        let mut possible_elements_table = Vec::with_capacity(capacity);

        let letters_vec: Vec<&T> = letters.iter().collect();

        let mut offset_array: Vec<usize> = Vec::with_capacity(letter_length);

        for i in 0..letter_length {
            offset_array.push(i);
        }

        'outer: loop {
            let mut a: Vec<&T> = Vec::with_capacity(letter_length);

            for i in 0..letter_length {
                a.push(letters_vec[offset_array[i]]);
            }

            let heap = Heap::new(&mut a);

            for a in heap {
                possible_elements_table.push(a.iter().map(|&e| e.clone()).collect());
            }

            let mut end = letter_length - 1;

            'inner: loop {
                offset_array[end] += 1;

                if offset_array[end] >= letters_length {
                    if end == 0 {
                        break 'outer;
                    } else {
                        end -= 1;

                        continue 'inner;
                    }
                } else {
                    for i in (end + 1)..letter_length {
                        offset_array[i] = offset_array[i - 1] + 1;

                        if offset_array[i] >= letters_length {
                            if end == 0 {
                                break 'outer;
                            } else {
                                end -= 1;

                                continue 'inner;
                            }
                        }
                    }
                    break;
                }
            }
        }


        possible_elements_table
    }
}

impl<T: Eq + Hash + Clone> Guesser<T> for ComputerGuesser<T> {
    type Error = HostError<T>;

    fn get_guess_times(&self) -> usize {
        self.guess_times
    }

    fn set_guess_times(&mut self, guess_times: usize) {
        self.guess_times = guess_times
    }

    fn add_condition(&mut self, guess: &[T], reply: (usize, usize)) {
        let now = Instant::now();

        let host = unsafe { Host::build_with_known_answer_unsafe(self.letters.clone(), guess.iter().map(|e| e.clone()).collect()) };

        for i in (0..(self.possible_elements_table.len())).rev() {
            let re = {
                let answer = &self.possible_elements_table[i];

                host.answer(answer).unwrap()
            };

            if re != reply {
                self.possible_elements_table.remove(i);
            }
        }

        if self.possible_elements_table.is_empty() {
            self.possible_elements_table = ComputerGuesser::make_possible_elements_table(&self.letters, self.letter_length);
        }

        let dt = now.elapsed().as_millis();

        if dt < self.thinking_delay as u128 {
            thread::sleep(Duration::from_millis(self.thinking_delay - dt as u64));
        }
    }

    fn guess(&self) -> Result<Vec<T>, Self::Error> {
        thread::sleep(Duration::from_millis(self.thinking_delay));

        let picked = random_pick::pick_from_slice(&self.possible_elements_table, &[1]).unwrap();

        Ok(picked.clone())
    }
}