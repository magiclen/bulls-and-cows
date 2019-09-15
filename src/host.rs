use std::collections::HashSet;
use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;

use crate::rand::seq::SliceRandom;

#[derive(PartialEq, Eq)]
/// The possible errors for the `Host` struct.
pub enum HostError<T: Eq + Hash + Clone> {
    /// The length of letters for a Bulls and Cows game must be at least 1.
    LettersEmpty,
    /// The length of the answer is incorrect.
    AnswerLengthIncorrect,
    /// There is an incorrect letter in the answer.
    AnswerContainsIncorrectLetter(T),
    /// There is an duplicated letter in the answer.
    AnswerContainsDuplicatedLetter(T),
}

impl<T: Eq + Hash + Clone> Debug for HostError<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        impl_debug_for_enum!(HostError::{LettersEmpty, AnswerLengthIncorrect, (AnswerContainsIncorrectLetter(_): (let .0 = "AnswerContainsIncorrectLetter")), (AnswerContainsDuplicatedLetter(_): (let .0 = "AnswerContainsDuplicatedLetter"))}, f, self);
    }
}

/// The game host for generating the question and answering for the question.
pub struct Host<T: Eq + Hash + Clone> {
    letters: HashSet<T>,
    answer: Vec<T>,
}

impl<T: Debug + Eq + Hash + Clone> Debug for Host<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        impl_debug_for_struct!(Host, f, self, .letters, .answer);
    }
}

impl<T: Eq + Hash + Clone> Host<T> {
    pub fn get_letters(&self) -> &HashSet<T> {
        &self.letters
    }

    pub fn get_answer_length(&self) -> usize {
        self.answer.len()
    }

    pub fn get_answer(&self) -> &[T] {
        &self.answer
    }
}

impl<T: Eq + Hash + Clone> Host<T> {
    /// Build a bulls-and-cows game host with a fixed answer.
    pub fn build(letters: HashSet<T>, answer_length: usize) -> Result<Host<T>, HostError<T>> {
        if letters.is_empty() {
            Err(HostError::LettersEmpty)
        } else {
            let letters_len = letters.len();

            if answer_length == 0 || answer_length > letters_len {
                Err(HostError::AnswerLengthIncorrect)
            } else {
                let answer: Vec<T> = letters.iter().take(answer_length).cloned().collect();

                Ok(Host {
                    letters,
                    answer,
                })
            }
        }
    }

    /// Build a bulls-and-cows game host with a random answer.
    pub fn build_with_random_answer(
        letters: HashSet<T>,
        answer_length: usize,
    ) -> Result<Host<T>, HostError<T>> {
        if letters.is_empty() {
            Err(HostError::LettersEmpty)
        } else {
            let mut host = Host {
                letters,
                answer: Vec::new(),
            };

            host.renew_with_random_answer(answer_length)?;

            Ok(host)
        }
    }

    /// Build a bulls-and-cows game host with a known answer.
    pub fn build_with_known_answer(
        letters: HashSet<T>,
        answer: Vec<T>,
    ) -> Result<Host<T>, HostError<T>> {
        if letters.is_empty() {
            Err(HostError::LettersEmpty)
        } else {
            let mut host = Host {
                letters,
                answer: Vec::new(),
            };

            host.renew_with_known_answer(answer)?;

            Ok(host)
        }
    }

    /// Build a bulls-and-cows game host with a known answer unsafely.
    pub unsafe fn build_with_known_answer_unsafe(letters: HashSet<T>, answer: Vec<T>) -> Host<T> {
        Host {
            letters,
            answer,
        }
    }

    /// Renew this host with a random answer.
    pub fn renew_with_random_answer(&mut self, answer_length: usize) -> Result<(), HostError<T>> {
        let letters = &self.letters;

        let letters_len = letters.len();

        if answer_length == 0 || answer_length > letters_len {
            Err(HostError::AnswerLengthIncorrect)
        } else {
            let mut answer: Vec<T> = Vec::with_capacity(answer_length);

            {
                let letters_vec: Vec<&T> = letters.iter().collect();

                let mut indices: Vec<usize> = Vec::with_capacity(answer_length);

                for i in 0..answer_length {
                    indices.push(i);
                }

                let mut rng = &mut rand::thread_rng();
                indices.shuffle(&mut rng);

                for i in indices {
                    answer.push(letters_vec[i].clone());
                }
            }

            self.answer = answer;

            Ok(())
        }
    }

    /// Renew this host with a known answer.
    pub fn renew_with_known_answer(&mut self, answer: Vec<T>) -> Result<(), HostError<T>> {
        let letters = &self.letters;

        let letters_len = letters.len();
        let answer_length = answer.len();

        if answer_length == 0 || answer_length > letters_len {
            Err(HostError::AnswerLengthIncorrect)
        } else {
            let mut answer_2: Vec<T> = Vec::with_capacity(answer_length);

            for letter in answer {
                if !letters.contains(&letter) {
                    return Err(HostError::AnswerContainsIncorrectLetter(letter));
                }
                if answer_2.contains(&letter) {
                    return Err(HostError::AnswerContainsDuplicatedLetter(letter));
                }

                answer_2.push(letter);
            }

            self.answer = answer_2;

            Ok(())
        }
    }

    /// Renew this host with a known answer unsafely.
    pub unsafe fn renew_with_known_answer_unsafe(&mut self, answer: Vec<T>) {
        self.answer = answer;
    }
}

impl<T: Eq + Hash + Clone> Host<T> {
    /// Answer for the question. If the format of the input answer is correct, it returns the number of bulls and the number of cows.
    pub fn answer(&self, answer: &[T]) -> Result<(usize, usize), HostError<T>> {
        let answer_length = answer.len();

        if answer_length != self.get_answer_length() {
            Err(HostError::AnswerLengthIncorrect)
        } else {
            let mut answer_2: Vec<&T> = Vec::with_capacity(answer_length);

            let mut bulls = 0;
            let mut cows = 0;

            for (i, letter) in answer.iter().enumerate() {
                if !self.letters.contains(&letter) {
                    return Err(HostError::AnswerContainsIncorrectLetter(letter.clone()));
                }
                if answer_2.contains(&letter) {
                    return Err(HostError::AnswerContainsDuplicatedLetter(letter.clone()));
                }

                if self.answer[i].eq(letter) {
                    bulls += 1;
                } else if self.answer.contains(letter) {
                    cows += 1;
                }

                answer_2.push(letter);
            }

            Ok((bulls, cows))
        }
    }
}
