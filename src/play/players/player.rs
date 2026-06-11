use std::hash::Hash;

/// A player who asks questions in the game.
pub trait Questioner<T: Eq + Hash + Clone> {
    type Error;

    /// Make a new question.
    fn make_new_question(&mut self);

    /// Answer for the question.
    fn answer(&self, answer: &[T]) -> Result<(usize, usize), Self::Error>;
}

/// A player who makes guesses in the game.
pub trait Guesser<T: Eq + Hash + Clone> {
    type Error;

    /// Get the guess count of this player.
    ///
    /// Guess counts are controlled by the game loop. Calling [`Guesser::guess`]
    /// does not have to increase this value automatically.
    fn get_guess_times(&self) -> usize;

    /// Set the guess count of this player.
    fn set_guess_times(&mut self, guess_times: usize);

    /// Add a condition based on a previous guess and its reply.
    ///
    /// Implementations may assume that `guess` was produced by this guesser and
    /// that `reply` is the questioner's answer for that exact guess.
    fn add_condition(&mut self, guess: &[T], reply: (usize, usize));

    /// Make a guess.
    ///
    /// The guess count is not increased automatically by this method; the game
    /// loop decides when a guess should count.
    fn guess(&self) -> Result<Vec<T>, Self::Error>;
}
