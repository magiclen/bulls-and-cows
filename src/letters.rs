use std::collections::HashSet;

/// To generate letters.
pub struct Letters;

impl Letters {
    /// Generate letters for numbers from 0 to 9.
    pub fn generate_numeric_letters() -> HashSet<u8> {
        let mut letters = HashSet::new();

        for i in 0..=9 {
            letters.insert(i);
        }

        letters
    }
}