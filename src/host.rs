use std::{
    collections::HashSet,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    hash::Hash,
};

use rand::seq::IteratorRandom;

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
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        debug_helper::impl_debug_for_enum!(HostError::{LettersEmpty, AnswerLengthIncorrect, (AnswerContainsIncorrectLetter(_): (let .0 = "AnswerContainsIncorrectLetter")), (AnswerContainsDuplicatedLetter(_): (let .0 = "AnswerContainsDuplicatedLetter"))}, f, self);
    }
}

impl<T: Eq + Hash + Clone> Display for HostError<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            HostError::LettersEmpty => {
                f.write_str("The length of letters for a Bulls and Cows game must be at least 1.")
            },
            HostError::AnswerLengthIncorrect => {
                f.write_str("The length of the answer is incorrect.")
            },
            HostError::AnswerContainsIncorrectLetter(_) => {
                f.write_str("There is an incorrect letter in the answer.")
            },
            HostError::AnswerContainsDuplicatedLetter(_) => {
                f.write_str("There is an duplicated letter in the answer.")
            },
        }
    }
}

impl<T: Eq + Hash + Clone> Error for HostError<T> {}

/// The game host for generating the question and answering for the question.
pub struct Host<T: Eq + Hash + Clone> {
    letters: HashSet<T>,
    answer:  Vec<T>,
}

impl<T: Debug + Eq + Hash + Clone> Debug for Host<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        debug_helper::impl_debug_for_struct!(Host, f, self, .letters, .answer);
    }
}

impl<T: Eq + Hash + Clone> Host<T> {
    /// Get all letters that can be used in an answer or a guess.
    pub fn get_letters(&self) -> &HashSet<T> {
        &self.letters
    }

    /// Get the length of the answer.
    pub fn get_answer_length(&self) -> usize {
        self.answer.len()
    }

    /// Get the current answer.
    pub fn get_answer(&self) -> &[T] {
        &self.answer
    }
}

impl<T: Eq + Hash + Clone> Host<T> {
    /// Build a bulls-and-cows game host with an arbitrary initial answer.
    ///
    /// The answer is selected from the iteration order of the provided
    /// [`HashSet`]. That order is not stable and should not be treated as a
    /// random answer or a reproducible known answer.
    ///
    /// Use [`Host::build_with_random_answer`] if you need a random answer, and
    /// use [`Host::build_with_known_answer`] if you need to specify the answer.
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
    ///
    /// The answer is made of `answer_length` distinct letters randomly chosen
    /// from `letters`.
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
    ///
    /// The answer must be non-empty, no longer than `letters`, contain only
    /// letters from `letters`, and contain no duplicated letters.
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
    ///
    /// # Safety
    ///
    /// The caller must ensure that `letters` is not empty, `answer.len()` is in
    /// `1..=letters.len()`, every element in `answer` exists in `letters`, and
    /// `answer` contains no duplicated elements. Breaking these rules will not
    /// cause memory unsafety, but it can make game results incorrect or make
    /// callers that assume a valid host state panic.
    pub unsafe fn build_with_known_answer_unsafe(letters: HashSet<T>, answer: Vec<T>) -> Host<T> {
        Host {
            letters,
            answer,
        }
    }

    /// Renew this host with a random answer.
    ///
    /// The answer is made of `answer_length` distinct letters randomly chosen
    /// from this host's letters.
    pub fn renew_with_random_answer(&mut self, answer_length: usize) -> Result<(), HostError<T>> {
        let letters = &self.letters;

        let letters_len = letters.len();

        if answer_length == 0 || answer_length > letters_len {
            Err(HostError::AnswerLengthIncorrect)
        } else {
            let answer: Vec<T> = letters
                .iter()
                .sample(&mut rand::rng(), answer_length)
                .into_iter()
                .cloned()
                .collect();

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
            let mut answer_set: HashSet<&T> = HashSet::with_capacity(answer_length);

            for letter in &answer {
                if !letters.contains(letter) {
                    return Err(HostError::AnswerContainsIncorrectLetter(letter.clone()));
                }
                if !answer_set.insert(letter) {
                    return Err(HostError::AnswerContainsDuplicatedLetter(letter.clone()));
                }
            }

            self.answer = answer;

            Ok(())
        }
    }

    /// Renew this host with a known answer unsafely.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `answer.len()` is in
    /// `1..=self.get_letters().len()`, every element in `answer` exists in this
    /// host's letters, and `answer` contains no duplicated elements. Breaking
    /// these rules will not cause memory unsafety, but it can make game results
    /// incorrect or make callers that assume a valid host state panic.
    pub unsafe fn renew_with_known_answer_unsafe(&mut self, answer: Vec<T>) {
        self.answer = answer;
    }
}

impl<T: Eq + Hash + Clone> Host<T> {
    /// Answer for the question.
    ///
    /// If the input has the correct length, contains only known letters, and has
    /// no duplicated letters, this returns the number of bulls and cows.
    pub fn answer(&self, answer: &[T]) -> Result<(usize, usize), HostError<T>> {
        let answer_length = answer.len();

        if answer_length != self.get_answer_length() {
            Err(HostError::AnswerLengthIncorrect)
        } else {
            let answer_set: HashSet<&T> = self.answer.iter().collect();
            let mut guessed_set: HashSet<&T> = HashSet::with_capacity(answer_length);

            let mut bulls = 0;
            let mut cows = 0;

            for (i, letter) in answer.iter().enumerate() {
                if !self.letters.contains(letter) {
                    return Err(HostError::AnswerContainsIncorrectLetter(letter.clone()));
                }
                if !guessed_set.insert(letter) {
                    return Err(HostError::AnswerContainsDuplicatedLetter(letter.clone()));
                }

                if self.answer[i].eq(letter) {
                    bulls += 1;
                } else if answer_set.contains(letter) {
                    cows += 1;
                }
            }

            Ok((bulls, cows))
        }
    }
}
