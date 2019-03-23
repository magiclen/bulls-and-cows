use std::hash::Hash;

/// A player who asks questions in the game.
pub trait Questioner<T: Eq + Hash + Clone> {
    type Error;

    /// Make a new question.
    fn make_new_question(&mut self);

    /// Answer for the question.
    fn answer(&self, answer: &[T]) -> Result<(usize, usize), Self::Error>;
}

/// A player who make guesses in the game.
pub trait Guesser<T: Eq + Hash + Clone> {
    type Error;

    /// Get guess times of this player.
    fn get_guess_times(&self) -> usize;

    /// Set guess times for this player.
    fn set_guess_times(&mut self, guess_times: usize);

    /// Add a condition.
    fn add_condition(&mut self, guess: &[T], reply: (usize, usize));

    /// Make a guess.
    fn guess(&self) -> Result<Vec<T>, Self::Error>;
}