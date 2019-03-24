Bulls and Cows
====================

[![Build Status](https://travis-ci.org/magiclen/bulls-and-cows.svg?branch=master)](https://travis-ci.org/magiclen/bulls-and-cows)
[![Build status](https://ci.appveyor.com/api/projects/status/t6o5r0j2yl11chsd/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/bulls-and-cows/branch/master)

This crate provides a framework for building bulls-and-cows games (1A2B) for any data type and any stages.

Typically, Bulls and Cows is a game that has 2 players, a questioner and a guesser. The questioner needs to decide a secret 4-digit (0 to 9) number in his or her mind and asks the guesser to guess the number. If the secret number is 4271 and the guess is 1234, then the questioner needs to answer `1A2B`. `1A2B` will be a new clue for the guesser to make the next guess better.

Beside 4 digits, players can choose to play on any other length of digits. The 4-digit numbers can be changed to 4-letter words and the number of players can also be more than 2.

## Usage

The `host` struct can be used independently for generating the question and answering for the question.

```rust
extern crate bulls_and_cows;

let host = bulls_and_cows::Host::build_with_known_answer(bulls_and_cows::Letters::generate_numeric_letters(), vec![1, 2, 3, 4]).unwrap();

assert_eq!((4, 0), host.answer(&vec![1, 2, 3, 4]).unwrap());
assert_eq!((2, 2), host.answer(&vec![1, 2, 4, 3]).unwrap());
assert_eq!((0, 4), host.answer(&vec![4, 3, 2, 1]).unwrap());
assert_eq!((0, 0), host.answer(&vec![5, 6, 7, 8]).unwrap());
```

If you want to build up a more complete game stage, use the `play` module. You can see the example `custom_2_players` to learn how to do that.

## Crates.io

https://crates.io/crates/bulls-and-cows

## Documentation

https://docs.rs/bulls-and-cows

## License

[MIT](LICENSE)