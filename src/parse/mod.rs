use std::fmt::{Display, Formatter};
use std::ops::RangeBounds;

use repeat::{Repeat, DelimitedBy};
use skip::{Skip, SkipAll};
use and::{And, AndReplace, AndDiscard};
use map::{Map, MapValue};
use or::Or;
use filter::{InRange, Where};
use crate::utils::gather_target::GatherTarget;

pub use int::{signed_int, unsigned_int, digit};
use crate::parse::vanguard::Vanguard;

mod repeat;
mod skip;
mod and;
mod or;
mod int;
mod map;
mod filter;
mod vanguard;

pub trait Parser<'i, T>: Sized + Copy {
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, T>;

    /// Find the first parsable result in the input.
    #[inline]
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

    // Parse into this container
    #[inline]
    fn parse_into<G>(&self, input: &'i [u8], target: &mut G, index: usize) -> ParseResult<'i, bool> where G: GatherTarget<T> {
        match self.parse(input) {
            ParseResult::Good(v, input) => ParseResult::Good(target.gather_into(index, v), input),
            ParseResult::Bad(err) => ParseResult::Bad(err)
        }
    }

    /// Process the output of this parser with this mapping function. It may not borrow
    /// anything from the closure as it must be copyable.
    #[inline]
    fn map<F, TF>(self, f: F) -> Map<Self, F, T, TF> where F: Fn(T) -> TF + Copy {
        Map::new(self, f)
    }

    /// If this parser succeeds, it will return the passed value instead
    #[inline]
    fn map_to<TV>(self, value: TV) -> MapValue<Self, T, TV> where TV: Copy {
        MapValue::new(self, value)
    }

    /// And parses the current parser, and then the one to the right, returning both values.
    /// You may want to use and_discard or and_instead to keep only one value.
    #[inline]
    fn and<PR, TR>(self, rhs: PR) -> And<Self, T, PR, TR> where PR: Parser<'i, TR> {
        And::new(self, rhs)
    }

    /// And parses the current parser, and then the one to the right, returning both values.
    /// You may want to use and_discard or and_instead to keep only one value.
    #[inline]
    fn and_instead<PR, TR>(self, rhs: PR) -> AndReplace<Self, T, PR, TR> where PR: Parser<'i, TR> {
        AndReplace::new(self, rhs)
    }

    /// And parses the current parser, and then the one to the right, returning both values.
    /// You may want to use and_discard or and_instead to keep only one value.
    #[inline]
    fn and_discard<PR, TR>(self, rhs: PR) -> AndDiscard<Self, T, PR, TR> where PR: Parser<'i, TR> {
        AndDiscard::new(self, rhs)
    }

    /// If this parser fails, it will instead try the other one. It must return the same
    /// type.
    #[inline]
    fn or<PR>(self, rhs: PR) -> Or<Self, PR, T> where PR: Parser<'i, T> {
        Or::new(self, rhs)
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
        Repeat::new(self, 0)
    }

    #[inline]
    fn repeat_n<R>(self, amount: usize) -> Repeat<Self, T, R> {
        Repeat::new(self, amount)
    }

    /// This parser does nothing on its own, but it will require a delimiter when paired with
    /// repeat. It is not strict, and will stop adding if it parses short of the delimiter.
    #[inline]
    fn delimited_by<PD, TD>(self, delimiter: PD) -> DelimitedBy<Self, PD, T, TD> where PD: Parser<'i, TD> {
        DelimitedBy::new(self, delimiter)
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

    /// Only allow parsing if the result is in range.
    #[inline]
    fn in_range<R>(self, range: R) -> InRange<Self, T> where R: RangeBounds<T>, T: Copy {
        InRange::new(self, range)
    }

    /// Filter filters the parsed result, failing the parser if the callback returns false.
    #[inline]
    fn filter<F>(self, callback: F) -> Where<Self, F, T> where F: Fn(&T) -> bool + Copy {
        Where::new(self, callback)
    }

    /// Optimize the parser by requiring that a 'vanguard' parser succeeds before the same input
    /// is tried on the wrapped parser. This could also be used as a pre-filter for an otherwise
    /// inexpensive parser.
    #[inline]
    fn with_vanguard<PV, TV>(self, vanguard: PV) -> Vanguard<Self, T, PV, TV> where PV: Parser<'i, TV> {
        Vanguard::new(self, vanguard)
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
    #[inline]
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
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, &'i [u8]> {
        let len = input.len();
        ParseResult::Good(input, &input[len..])
    }
}


#[derive(Copy, Clone)]
struct AnyByte;

impl<'i> Parser<'i, u8> for AnyByte {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, u8> {
        if input.len() >= 1 {
            ParseResult::Good(input[0], &input[1..])
        } else {
            ParseResult::Bad(ParseError::new("Empty input", input))
        }
    }
}

#[derive(Copy, Clone)]
struct NBytes<const N: usize>;

impl<'i, const N: usize> Parser<'i, [u8; N]> for NBytes<N> {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, [u8; N]> {
        if input.len() >= N {
            let mut res = [0u8; N];
            res.copy_from_slice(&input[..N]);

            ParseResult::Good(res, &input[N..])
        } else {
            ParseResult::Bad(ParseError::new("Empty input", input))
        }
    }
}

#[derive(Copy, Clone)]
struct BytesUntil(u8, bool);

impl<'i> Parser<'i, &'i [u8]> for BytesUntil {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, &'i [u8]> {
        match input.iter().position(|v| *v == self.0) {
            Some(pos) => ParseResult::Good(&input[..pos], &input[pos + (self.1 as usize)..]),
            None => ParseResult::Bad(ParseError::new("Byte not found", input))
        }
    }
}


#[inline]
pub fn everything<'i>() -> impl Parser<'i, &'i [u8]> {
    Everything
}

#[inline]
pub fn anything<'i>() -> impl Parser<'i, &'i [u8]> {
    Anything
}

#[inline]
pub fn any_byte<'i>() -> impl Parser<'i, u8> {
    AnyByte
}

#[inline]
pub fn n_bytes<'i, const N: usize>() -> impl Parser<'i, [u8; N]> { NBytes::<N> }

#[inline]
pub fn bytes_until<'i>(b: u8, eat: bool) -> impl Parser<'i, &'i [u8]> { BytesUntil(b, eat) }

#[inline]
pub fn word<'i>() -> impl Parser<'i, &'i [u8]> { BytesUntil(b' ', true).or(line()) }

#[inline]
pub fn line<'i>() -> impl Parser<'i, &'i [u8]> { BytesUntil(b'\n', true) }

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