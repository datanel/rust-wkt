use std::iter::Peekable;

use tokenizer::{Token, Tokenizer};
use types::coord::Coord;


pub struct Point {
    pub coord: Coord
}

impl Point {
    pub fn from_tokens(tokens: &mut Peekable<Token, Tokenizer>) ->  Result<Self, &'static str> {
        let coord = match Coord::from_tokens(tokens) {
            Ok(c) => c,
            Err(s) => return Err(s),
        };
        Ok(Point {coord: coord})
    }
}