use std::fmt::{Display, Formatter};
use repeat::{Repeat, RepeatDelimited};
use skip::{Skip, SkipAll};

mod repeat;
mod skip;
mod int;

pub use int::{signed_int, unsigned_int, digit};


pub trait Parser<'i, T>: Sized + Copy {
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, T>;

    /// Repeat this parser until the collection fills or a non-first result fails.
    /// If the first result fails, it will fail as well.
    ///
    /// Supported collections (`_` can be used in place of `T`)
    /// * `[T; N]` – Fixed size array, will stop at end
    /// * `Vec<T>` – Ordinary vector, won't stop
    /// * `([T; N], usize)` – Array and counter.
    /// * `ArrayVec<T, N>` – ArrayVec, will stop if full
    /// * `SmallVec<[T; N]>` – SmallVec, will be treated as vector
    /// * `(T, T) | (T, T, T) | (T, T, T, T)` – Collect it into a tuple, unfilled will have default value
    fn repeat<R>(self) -> Repeat<Self, T, R> {
        Repeat::new(self)
    }

    /// Same as repeat, but it requires a delimiter between values.
    fn repeat_delimited<R, TD, PD>(self, delimiter_parser: PD) -> RepeatDelimited<Self, T, PD, TD, R> where PD: Parser<'i, TD> {
        RepeatDelimited::new(self, delimiter_parser)
    }

    /// Same as repeat, but it requires a specified amount.
    fn repeat_n<R>(self, amount: usize) -> Repeat<Self, T, R> {
        Repeat::with_amount(self, amount)
    }

    /// Same as repeat_delimited, but it requires a specified amount.
    fn repeat_delimited_n<R, TD, PD>(self, delimiter_parser: PD, amount: usize) -> RepeatDelimited<Self, T, PD, TD, R> where PD: Parser<'i, TD> {
        RepeatDelimited::with_amount(self, delimiter_parser, amount)
    }

    /// Parse this, then skip what comes from parser2.
    fn then_skip<P2, T2>(self, parser: P2) -> Skip<Self, P2, T, T2> where P2: Parser<'i, T2> {
        Skip::new(self, parser)
    }

    /// Parse this, then skip until the second parses fails.
    fn then_skip_all<P2, T2>(self, parser: P2) -> SkipAll<Self, P2, T, T2> where P2: Parser<'i, T2> {
        SkipAll::new(self, parser)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ParseError<'i> {
    reason: &'static str,
    sub_reason: Option<&'static str>,
    input: &'i [u8],
}

impl<'i> Display for ParseError<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(sub_reason) = self.sub_reason {
            write!(f, "{}: {}", self.reason, sub_reason)
        } else {
            write!(f, "{}", self.reason)
        }
    }
}

impl<'i> ParseError<'i> {
    fn new(reason: &'static str, input: &'i [u8]) -> Self {
        Self { reason, input, sub_reason: None }
    }

    fn wrap(&self, reason: &'static str, input: &'i [u8]) -> Self {
        Self { reason, input, sub_reason: self.sub_reason.or(Some(self.reason)) }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum ParseResult<'i, T> {
    Good(T, &'i [u8]),
    Bad(ParseError<'i>),
}

impl<'i, T> ParseResult<'i, T> {
    pub fn unwrap(self) -> T {
        match self {
            ParseResult::Good(v, _) => v,
            ParseResult::Bad(err) => panic!("Unwrap on failed parse: {}", err.reason)
        }
    }
}

impl<'i> Parser<'i, u8> for u8 {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, u8> {
        if input.first().copied() == Some(*self) {
            ParseResult::Good(*self, &input[1..])
        } else {
            ParseResult::Bad(ParseError::new("u8 not matched", input))
        }
    }
}

impl<'i, 's> Parser<'i, &'i [u8]> for &'s [u8] {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, &'i [u8]> {
        if input.len() >= self.len() {
            let (head, tail) = input.split_at(self.len());
            if &head == self {
                ParseResult::Good(head, tail)
            } else {
                ParseResult::Bad(ParseError::new("String does not match", input))
            }
        } else {
            ParseResult::Bad(ParseError::new("String is too short", input))
        }
    }
}

impl<'i, 's, const N: usize> Parser<'i, &'i [u8]> for &'s [u8; N] {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, &'i [u8]> {
        if input.len() >= N {
            let (head, tail) = input.split_at(self.len());
            if &head == self {
                ParseResult::Good(head, tail)
            } else {
                ParseResult::Bad(ParseError::new("String does not match", input))
            }
        } else {
            ParseResult::Bad(ParseError::new("String is too short", input))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_match_works() {
        assert_eq!(
            b'H'.parse(b"Hello, World"),
            ParseResult::Good(b'H', b"ello, World"),
        );
        assert_eq!(
            b'H'.parse(b"Jello World"),
            ParseResult::Bad(ParseError::new("u8 not matched", b"Jello World"))
        );
    }

    #[test]
    fn slice_match_works() {
        assert_eq!(
            b"Hello, ".parse(b"Hello, World"),
            ParseResult::Good(b"Hello, ".as_slice(), b"World"),
        );
        assert_eq!(
            b"stuff(".as_slice().parse(b"stuff(32)"),
            ParseResult::Good(b"stuff(".as_slice(), b"32)"),
        );
        assert_eq!(
            b"Hello, ".parse(b"Hallo, Welt"),
            ParseResult::Bad(ParseError::new("String does not match", b"Hallo, Welt"))
        );
        assert_eq!(
            b"Hello, ".as_slice().parse(b"Hallo, Welt"),
            ParseResult::Bad(ParseError::new("String does not match", b"Hallo, Welt"))
        );
        assert_eq!(
            b"Hello, ".parse(b"Hell"),
            ParseResult::Bad(ParseError::new("String is too short", b"Hell"))
        );
        assert_eq!(
            b"Hello, ".as_slice().parse(b"Hell"),
            ParseResult::Bad(ParseError::new("String is too short", b"Hell"))
        );
    }
}