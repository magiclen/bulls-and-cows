use regex::Regex;

/// A parser to parse `XAYB` strings.
#[derive(Debug)]
pub struct ABParser {
    regex: Regex,
}

impl ABParser {
    /// Create a new `ABParser` instance.
    pub fn new() -> ABParser {
        let regex = Regex::new(r"^(\d+)[ ]*[aA][ ]*(\d+)[ ]*[bB]$").unwrap();

        ABParser {
            regex,
        }
    }
}

impl ABParser {
    /// Parse a `XAYB` string in order to get the `X` and the `Y`.
    pub fn parse<S: AsRef<str>>(&self, s: S) -> Option<(usize, usize)> {
        let captures = self.regex.captures(s.as_ref().trim());

        match captures {
            Some(captures) => {
                let a = match captures[1].parse::<usize>() {
                    Ok(a) => a,
                    Err(_) => return None,
                };

                let b = match captures[2].parse::<usize>() {
                    Ok(b) => b,
                    Err(_) => return None,
                };

                Some((a, b))
            },
            None => None,
        }
    }
}

impl Default for ABParser {
    #[inline]
    fn default() -> Self {
        ABParser::new()
    }
}
