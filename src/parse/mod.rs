use arrayvec::ArrayVec;
use std::marker::PhantomData;
use std::ops::RangeBounds;

use crate::utils::gather_target::GatherTarget;

pub use bytes::*;
pub use choice::choice;
pub use int::{digit, hex_byte, hex_digit, signed_int, unsigned_int};
pub use skip::skip;

use crate::parse::cap::{CappedBy, QuotedBy};
use crate::parse::repeat::RepeatFold;
use and::{And, AndDiscard, AndReplace};
use filter::{Filter, InRange};
use map::{FilterMap, Map, MapValue};
use or::Or;
use repeat::{Count, DelimitedBy, Repeat};
use rewind::Rewind;
use skip::{SkipAll, ThenSkip};
use vanguard::Vanguard;

mod and;
mod bytes;
mod cap;
mod choice;
mod filter;
mod int;
mod map;
mod or;
mod repeat;
mod rewind;
mod skip;
mod vanguard;

pub trait Parser<'i, T>: Sized + Copy {
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, T>;

    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> ParseResult<'i, ()> {
        match self.parse(input) {
            ParseResult::Good(_, input) => ParseResult::Good((), input),
            ParseResult::Bad(err) => ParseResult::Bad(err),
        }
    }

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

        ParseResult::new_bad("No parsable input found")
    }

    /// Find the last parsable result in the input.
    #[inline]
    fn last_parsable_in(&self, input: &'i [u8]) -> ParseResult<'i, (T, usize)> {
        let input = input;
        let mut offset = input.len() - 1;
        loop {
            if let ParseResult::Good(v, next_input) = self.parse(&input[offset..]) {
                return ParseResult::Good((v, offset), next_input);
            }

            if offset == 0 {
                break;
            }
            offset -= 1;
        }

        ParseResult::new_bad("No parsable last input found")
    }

    #[inline]
    #[allow(unused_variables)]
    fn parse_at_index(&self, input: &'i [u8], index: usize) -> ParseResult<'i, T> {
        self.parse(input)
    }

    // Parse into this container
    #[inline]
    fn parse_into<G>(&self, input: &'i [u8], target: &mut G, index: usize) -> ParseResult<'i, bool>
    where
        G: GatherTarget<T>,
    {
        match self.parse_at_index(input, index) {
            ParseResult::Good(v, input) => ParseResult::Good(target.gather_into(index, v), input),
            ParseResult::Bad(err) => ParseResult::Bad(err),
        }
    }

    fn parse_iter(&self, input: &'i [u8]) -> ParseIterator<'i, Self, T> {
        ParseIterator {
            parser: *self,
            index: 0,
            spooky_ghost: PhantomData::default(),
            input,
        }
    }

    /// Process the output of this parser with this mapping function. It may not borrow
    /// anything from the closure as it must be copyable.
    #[inline]
    fn map<F, TF>(self, f: F) -> Map<Self, F, T, TF>
    where
        F: Fn(T) -> TF + Copy,
    {
        Map::new(self, f)
    }

    /// If this parser succeeds, it will return the passed value instead
    #[inline]
    fn map_to<TV>(self, value: TV) -> MapValue<Self, T, TV>
    where
        TV: Copy,
    {
        MapValue::new(self, value)
    }

    #[inline]
    fn filter_map<F, TF>(self, f: F) -> FilterMap<Self, F, T, TF>
    where
        F: Fn(T) -> Option<TF> + Copy,
    {
        FilterMap::new(self, f)
    }

    /// And parses the current parser, and then the one to the right, returning both values.
    /// You may want to use and_discard or and_instead to keep only one value.
    #[inline]
    fn and<PR, TR>(self, rhs: PR) -> And<Self, T, PR, TR>
    where
        PR: Parser<'i, TR>,
    {
        And::new(self, rhs)
    }

    /// And parses the current parser, and then the one to the right, returning both values.
    /// You may want to use and_discard or and_instead to keep only one value.
    #[inline]
    fn and_instead<PR, TR>(self, rhs: PR) -> AndReplace<Self, T, PR, TR>
    where
        PR: Parser<'i, TR>,
    {
        AndReplace::new(self, rhs)
    }

    /// And parses the current parser, and then the one to the right, returning both values.
    /// You may want to use and_discard or and_instead to keep only one value.
    #[inline]
    fn and_discard<PR, TR>(self, rhs: PR) -> AndDiscard<Self, T, PR, TR>
    where
        PR: Parser<'i, TR>,
    {
        AndDiscard::new(self, rhs)
    }

    /// Rewind the input after parsing this instead of continuing.
    #[inline]
    fn rewind(self) -> Rewind<Self, T> {
        Rewind::new(self)
    }

    /// Get the content until the RQ parser. The RQ is consumed, and the content must all be consumed
    #[inline]
    fn capped_by<PC, TC>(self, parser: PC) -> CappedBy<Self, T, PC, TC>
    where
        PC: Parser<'i, TC>,
    {
        CappedBy::new(self, parser)
    }

    /// Get the content between the LQ and RQ parser. The LQ and RQ are consumed, and all between
    /// must also be consumed.
    #[inline]
    fn quoted_by<PL, TL, PR, TR>(
        self,
        lq_parser: PL,
        rq_parser: PR,
    ) -> QuotedBy<Self, T, PL, TL, PR, TR>
    where
        PL: Parser<'i, TL>,
        PR: Parser<'i, TR>,
    {
        QuotedBy::new(self, lq_parser, rq_parser)
    }

    /// If this parser fails, it will instead try the other one. It must return the same
    /// type.
    #[inline]
    fn or<PR>(self, rhs: PR) -> Or<Self, PR, T>
    where
        PR: Parser<'i, T>,
    {
        Or::new(self, rhs)
    }

    /// If the parser fail, return this value instead of an error.
    fn or_return(self, value: T) -> Or<Self, Return<T>, T> {
        Or::new(self, Return(value))
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

    /// Repeat for **exactly** the specified amount of times.
    #[inline]
    fn repeat_n<R>(self, amount: usize) -> Repeat<Self, T, R> {
        Repeat::new(self, amount)
    }

    #[inline]
    fn repeat_fold<FI, FS, TA>(
        self,
        init_func: FI,
        step_func: FS,
    ) -> RepeatFold<Self, FI, FS, T, TA>
    where
        FI: Fn() -> TA + Copy,
        FS: Fn(TA, T) -> TA + Copy,
    {
        RepeatFold::new(self, init_func, step_func)
    }

    /// Count the repetitions. This returns an error if it empty.
    #[inline]
    fn count_repetitions(self) -> Count<Self, T> {
        Count::new(self)
    }

    /// This parser does nothing on its own, but it will require a delimiter when paired with
    /// repeat. It is not strict, and will stop adding if it parses short of the delimiter.
    #[inline]
    fn delimited_by<PD, TD>(self, delimiter: PD) -> DelimitedBy<Self, PD, T, TD>
    where
        PD: Parser<'i, TD>,
    {
        DelimitedBy::new(self, delimiter)
    }

    /// Parse this, then skip what comes from parser2.
    #[inline]
    fn then_skip<P2, T2>(self, parser: P2) -> ThenSkip<Self, P2, T, T2>
    where
        P2: Parser<'i, T2>,
    {
        ThenSkip::new(self, parser)
    }

    /// Parse this, then skip until the second parses fails.
    #[inline]
    fn then_skip_all<P2, T2>(self, parser: P2) -> SkipAll<Self, P2, T, T2>
    where
        P2: Parser<'i, T2>,
    {
        SkipAll::new(self, parser)
    }

    /// Only allow parsing if the result is in range.
    #[inline]
    fn in_range<R>(self, range: R) -> InRange<Self, T>
    where
        R: RangeBounds<T>,
        T: Copy,
    {
        InRange::new(self, range)
    }

    /// Filter filters the parsed result, failing the parser if the callback returns false.
    #[inline]
    fn only_if<F>(self, callback: F) -> Filter<Self, F, T>
    where
        F: Fn(&T) -> bool + Copy,
    {
        Filter::new(self, callback)
    }

    /// Optimize the parser by requiring that a 'vanguard' parser succeeds before the same input
    /// is tried on the wrapped parser. This could also be used as a pre-filter for an otherwise
    /// inexpensive parser.
    #[inline]
    fn with_vanguard<PV, TV>(self, vanguard: PV) -> Vanguard<Self, T, PV, TV>
    where
        PV: Parser<'i, TV>,
    {
        Vanguard::new(self, vanguard)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum ParseResult<'i, T> {
    Good(T, &'i [u8]),
    Bad(ArrayVec<&'static str, 4>),
}

impl<'i, T> ParseResult<'i, T> {
    pub fn unwrap(self) -> T {
        match self {
            ParseResult::Good(v, _) => v,
            ParseResult::Bad(err) => panic!("Unwrap on failed parse (errors: {:?})", err),
        }
    }

    #[cfg(test)]
    pub fn new_bad_slice(errs: &'static [&'static str]) -> Self {
        Self::Bad(ArrayVec::try_from(errs).unwrap())
    }

    pub fn new_bad(err: &'static str) -> Self {
        let mut errs = ArrayVec::new();
        errs.push(err);

        ParseResult::Bad(errs)
    }

    pub fn wrap_bad(mut err: ArrayVec<&'static str, 4>, new_err: &'static str) -> Self {
        if !err.is_full() {
            err.push(new_err)
        }

        ParseResult::Bad(err)
    }
}

impl<'i> Parser<'i, u8> for u8 {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, u8> {
        if input.first().copied() == Some(*self) {
            ParseResult::Good(*self, &input[1..])
        } else {
            ParseResult::new_bad("u8 not matched")
        }
    }

    #[inline]
    fn first_parsable_in(&self, input: &'i [u8]) -> ParseResult<'i, (u8, usize)> {
        match input.iter().position(|v| *v == *self) {
            Some(index) => ParseResult::Good((*self, index), &input[index + 1..]),
            None => ParseResult::new_bad("Byte not found in input"),
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
                ParseResult::new_bad("String does not match")
            }
        } else {
            ParseResult::new_bad("String is too short")
        }
    }

    #[inline]
    fn first_parsable_in(&self, input: &'i [u8]) -> ParseResult<'i, (&'i [u8], usize)> {
        match input
            .windows(self.len())
            .enumerate()
            .find(|(_, w)| w == self)
        {
            Some((index, data)) => ParseResult::Good((data, index), &input[index + self.len()..]),
            None => ParseResult::new_bad("Byte slice not found in input"),
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
                ParseResult::new_bad("String does not match")
            }
        } else {
            ParseResult::new_bad("String is too short")
        }
    }

    #[inline]
    fn first_parsable_in(&self, input: &'i [u8]) -> ParseResult<'i, (&'i [u8], usize)> {
        match input
            .array_windows::<N>()
            .enumerate()
            .find(|(_, w)| w == self)
        {
            Some((index, data)) => ParseResult::Good((data, index), &input[index + N..]),
            None => ParseResult::new_bad("Byte slice not found in input"),
        }
    }
}

pub struct ParseIterator<'i, P, T> {
    input: &'i [u8],
    parser: P,
    index: usize,
    spooky_ghost: PhantomData<T>,
}

impl<'i, P, T> Iterator for ParseIterator<'i, P, T>
where
    P: Parser<'i, T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.parser.parse_at_index(self.input, self.index) {
            ParseResult::Good(res, new_input) => {
                self.index += 1;
                self.input = new_input;

                Some(res)
            }
            ParseResult::Bad(_) => None,
        }
    }
}

pub struct Return<T>(T);

impl<T> Copy for Return<T> where T: Copy {}

impl<T> Clone for Return<T>
where
    T: Copy,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'i, T> Parser<'i, T> for Return<T>
where
    T: Copy,
{
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        ParseResult::Good(self.0, input)
    }
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
        assert_eq!(
            b"\n\n".first_parsable_in(b"Paragraph 1\n\nParagraph 2\n\n"),
            ParseResult::Good((b"\n\n".as_slice(), 11), b"Paragraph 2\n\n"),
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
            ParseResult::new_bad("u8 not matched")
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
            ParseResult::new_bad("String does not match")
        );
        assert_eq!(
            b"Hello, ".as_slice().parse(b"Hallo, Welt"),
            ParseResult::new_bad("String does not match")
        );
        assert_eq!(
            b"Hello, ".parse(b"Hell"),
            ParseResult::new_bad("String is too short")
        );
        assert_eq!(
            b"Hello, ".as_slice().parse(b"Hell"),
            ParseResult::new_bad("String is too short")
        );
    }
}
