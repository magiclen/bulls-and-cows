use std::collections::HashSet;

/// Helpers for generating common letter sets.
pub struct Letters;

impl Letters {
    /// Generate letters for numeric digits from `0` to `9`.
    pub fn generate_numeric_letters() -> HashSet<u8> {
        let mut letters = HashSet::new();

        for i in 0..=9 {
            letters.insert(i);
        }

        letters
    }
}
