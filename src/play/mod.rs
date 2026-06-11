//! Tools for composing full Bulls and Cows game flows.
//!
//! The player traits and built-in computer players live in [`players`]. A game
//! loop usually connects one [`players::Questioner`] with one
//! [`players::Guesser`], asks the guesser for a guess, sends that guess to the
//! questioner, and then feeds the reply back to the guesser as a condition.

pub mod players;
