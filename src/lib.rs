extern crate rand;

mod host;
mod letters;

pub use self::host::{Host, HostError};
pub use self::letters::Letters;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
