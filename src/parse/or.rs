use std::marker::PhantomData;
use crate::parse::{Parser, ParseResult};

pub struct Or<PL, PR, T> {
    parse_left: PL,
    parse_right: PR,
    spooky_ghost: PhantomData<T>,
}

impl<PL, PR, T> Or<PL, PR, T> {
    #[inline]
    pub fn new(parse_left: PL, parse_right: PR) -> Self {
        Self { parse_left, parse_right, spooky_ghost: Default::default() }
    }
}

impl<PL, PR, T> Copy for Or<PL, PR, T> where PL: Copy, PR: Copy {}

impl<PL, PR, T> Clone for Or<PL, PR, T> where PL: Clone, PR: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            parse_left: self.parse_left.clone(),
            parse_right: self.parse_right.clone(),
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PL, PR, T> Parser<'i, T> for Or<PL, PR, T> where PL: Parser<'i, T>, PR: Parser<'i, T> {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        if let ParseResult::Good(t, new_input) = self.parse_left.parse(input) {
            ParseResult::Good(t, new_input)
        } else {
            self.parse_right.parse(input)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::{everything, ParseError, signed_int};
    use super::*;

    #[test]
    fn or_works_as_it_should() {
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        enum TestData<'i> {
            Noop,
            Add(i32, i32),
            Print(&'i [u8]),
        }

        let parser = b"noop".map_to(TestData::Noop)
            .or(b"add ".and_instead(signed_int())
                .and_discard(b' ')
                .and(signed_int())
                .map(|(a, b)| TestData::Add(a, b)))
            .or(b"print ".and_instead(everything()).map(|s| TestData::Print(s)));

        assert_eq!(parser.parse(b"noop"), ParseResult::Good(TestData::Noop, b""));
        assert_eq!(parser.parse(b"add -47 112"), ParseResult::Good(TestData::Add(-47, 112), b""));
        assert_eq!(parser.parse(b"print Hello World"), ParseResult::Good(TestData::Print(b"Hello World"), b""));
        assert_eq!(parser.parse(b"mul 183 929"), ParseResult::Bad(
            ParseError::new("String does not match", b"mul 183 929")
                .wrap("Left in And failed", b"mul 183 929")
        ));
    }
}