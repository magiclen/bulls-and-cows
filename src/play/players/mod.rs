//! Player traits and built-in computer player implementations.
//!
//! [`Questioner`] represents the side that owns the answer and returns bull/cow
//! counts. [`Guesser`] represents the side that produces guesses and learns
//! from previous replies.

mod computer;
mod player;

pub use self::{computer::*, player::*};
