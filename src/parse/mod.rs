use std::fmt::{Display, Formatter};
use repeat::{Repeat, RepeatDelimited};
use skip::{Skip, SkipAll};
use and::{And, AndReplace, AndDiscard};

mod repeat;
mod skip;
mod and;
mod int;

pub use int::{signed_int, unsigned_int, digit};

pub trait Parser<'i, T>: Sized + Copy {
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, T>;

    /// Find the first parsable result in the input.
    fn first_parsable_in(&self, input: &'i [u8]) -> ParseResult<'i, (T, usize)> {
        let mut input = input;
        let mut offset = 0;
        while !input.is_empty() {
            if let ParseResult::Good(v, next_input) = self.parse(input) {
                return ParseResult::Good((v, offset), next_input);
            }

            input = &input[1..];
            offset += 1;
        }

        ParseResult::Bad(ParseError::new("None were found", input))
    }

    /// And parses the current parser, and then the one to the right, returning both values.
    /// You may want to use and_discard or and_instead to keep only one value.
    fn and<PR, TR>(self, parse_right: PR) -> And<Self, T, PR, TR> where PR: Parser<'i, TR> {
        And::new(self, parse_right)
    }

    /// And parses the current parser, and then the one to the right, returning both values.
    /// You may want to use and_discard or and_instead to keep only one value.
    fn and_instead<PR, TR>(self, parse_right: PR) -> AndReplace<Self, T, PR, TR> where PR: Parser<'i, TR> {
        AndReplace::new(self, parse_right)
    }

    /// And parses the current parser, and then the one to the right, returning both values.
    /// You may want to use and_discard or and_instead to keep only one value.
    fn and_discard<PR, TR>(self, parse_right: PR) -> AndDiscard<Self, T, PR, TR> where PR: Parser<'i, TR> {
        AndDiscard::new(self, parse_right)
    }

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
    #[inline]
    fn repeat<R>(self) -> Repeat<Self, T, R> {
        Repeat::new(self)
    }

    /// Same as repeat, but it requires a delimiter between values.
    #[inline]
    fn repeat_delimited<R, TD, PD>(self, delimiter_parser: PD) -> RepeatDelimited<Self, T, PD, TD, R> where PD: Parser<'i, TD> {
        RepeatDelimited::new(self, delimiter_parser)
    }

    /// Same as repeat, but it requires a specified amount.
    #[inline]
    fn repeat_n<R>(self, amount: usize) -> Repeat<Self, T, R> {
        Repeat::with_amount(self, amount)
    }

    /// Same as repeat_delimited, but it requires a specified amount.
    #[inline]
    fn repeat_delimited_n<R, TD, PD>(self, delimiter_parser: PD, amount: usize) -> RepeatDelimited<Self, T, PD, TD, R> where PD: Parser<'i, TD> {
        RepeatDelimited::with_amount(self, delimiter_parser, amount)
    }

    /// Parse this, then skip what comes from parser2.
    #[inline]
    fn then_skip<P2, T2>(self, parser: P2) -> Skip<Self, P2, T, T2> where P2: Parser<'i, T2> {
        Skip::new(self, parser)
    }

    /// Parse this, then skip until the second parses fails.
    #[inline]
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

    #[inline]
    fn first_parsable_in(&self, input: &'i [u8]) -> ParseResult<'i, (u8, usize)> {
        match input.iter().position(|v| *v == *self) {
            Some(index) => ParseResult::Good((*self, index), &input[index + 1..]),
            None => ParseResult::Bad(ParseError::new("Byte not found in input", input)),
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

    #[inline]
    fn first_parsable_in(&self, input: &'i [u8]) -> ParseResult<'i, (&'i [u8], usize)> {
        match input.windows(self.len()).enumerate().find(|(_, w)| w == self) {
            Some((index, data)) => ParseResult::Good(
                (data, index), &input[index + self.len()..],
            ),
            None => ParseResult::Bad(ParseError::new("Byte slice not found in input", input)),
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

    #[inline]
    fn first_parsable_in(&self, input: &'i [u8]) -> ParseResult<'i, (&'i [u8], usize)> {
        match input.windows(N).enumerate().find(|(_, w)| *w == self.as_slice()) {
            Some((index, data)) => ParseResult::Good((data, index), &input[index + N..]),
            None => ParseResult::Bad(ParseError::new("Byte slice not found in input", input)),
        }
    }
}

#[derive(Copy, Clone)]
struct Everything;

impl<'i> Parser<'i, &'i [u8]> for Everything {
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, &'i [u8]> {
        let len = input.len();
        if len > 0 {
            ParseResult::Good(input, &input[len..])
        } else {
            ParseResult::Bad(ParseError::new("Nothing is the only thing that does not match Everything", input))
        }
    }
}

#[derive(Copy, Clone)]
struct Anything;

impl<'i> Parser<'i, &'i [u8]> for Anything {
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, &'i [u8]> {
        let len = input.len();
        ParseResult::Good(input, &input[len..])
    }
}

pub fn everything<'i>() -> impl Parser<'i, &'i [u8]> {
    Everything
}

pub fn anything<'i>() -> impl Parser<'i, &'i [u8]> {
    Anything
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_find_first_parsable_works_for_the_basics() {
        assert_eq!(
            b'H'.first_parsable_in(b"Hello, World"),
            ParseResult::Good((b'H', 0), b"ello, World"),
        );
        assert_eq!(
            b' '.first_parsable_in(b"Hello, World"),
            ParseResult::Good((b' ', 6), b"World"),
        );
        assert_eq!(
            signed_int::<i32>().first_parsable_in(b"The number is -954923166!"),
            ParseResult::Good((-954923166, 14), b"!"),
        );
    }

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