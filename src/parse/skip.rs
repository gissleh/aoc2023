use std::marker::PhantomData;
use crate::parse::{Parser, ParseResult};
use crate::utils::gather_target::GatherTarget;

struct Skip<P, T> {
    parser: P,
    spooky_ghost: PhantomData<T>,
}

impl<P, T> Copy for Skip<P, T> where P: Copy {}

impl<P, T> Clone for Skip<P, T> where P: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self { parser: self.parser.clone(), spooky_ghost: Default::default() }
    }
}

impl<'i, P, T> Parser<'i, Option<T>> for Skip<P, T> where P: Parser<'i, T> {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, Option<T>> {
        match self.parser.parse(input) {
            ParseResult::Good(v, input) => ParseResult::Good(Some(v), input),
            ParseResult::Bad(_) => ParseResult::Good(None, input),
        }
    }
}

#[inline]
pub fn skip<'i, P, T>(parser: P) -> impl Parser<'i, Option<T>> where P: Parser<'i, T> {
    Skip { parser, spooky_ghost: Default::default() }
}

pub struct ThenSkip<PV, PS, TV, TS> {
    value_parser: PV,
    skip_parser: PS,
    spooky_ghost: PhantomData<(TV, TS)>,
}

impl<PV, PS, TV, TS> ThenSkip<PV, PS, TV, TS> {
    #[inline]
    pub(crate) fn new(value_parser: PV, skip_parser: PS) -> Self {
        Self {
            value_parser,
            skip_parser,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PV, PS, TV, TS> Copy for ThenSkip<PV, PS, TV, TS> where PS: Parser<'i, TS>, PV: Parser<'i, TV> {}

impl<'i, PV, PS, TV, TS> Clone for ThenSkip<PV, PS, TV, TS> where PS: Parser<'i, TS>, PV: Parser<'i, TV> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            value_parser: self.value_parser,
            skip_parser: self.skip_parser,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PV, PS, TV, TS> Parser<'i, TV> for ThenSkip<PV, PS, TV, TS> where PV: Parser<'i, TV>, PS: Parser<'i, TS> {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, TV> {
        match self.value_parser.parse(input) {
            ParseResult::Good(tv, input) => match self.skip_parser.parse(input) {
                ParseResult::Good(_, input) => ParseResult::Good(tv, input),
                ParseResult::Bad(_) => ParseResult::Good(tv, input)
            }
            ParseResult::Bad(err) => ParseResult::Bad(err)
        }
    }
}

pub struct SkipAll<PV, PS, TV, TS> {
    value_parser: PV,
    skip_parser: PS,
    spooky_ghost: PhantomData<(TV, TS)>,
}

impl<PV, PS, TV, TS> SkipAll<PV, PS, TV, TS> {
    #[inline]
    pub(crate) fn new(value_parser: PV, skip_parser: PS) -> Self {
        Self {
            value_parser,
            skip_parser,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PV, PS, TV, TS> Copy for SkipAll<PV, PS, TV, TS> where PS: Parser<'i, TS>, PV: Parser<'i, TV> {}

impl<'i, PV, PS, TV, TS> Clone for SkipAll<PV, PS, TV, TS> where PS: Parser<'i, TS>, PV: Parser<'i, TV> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            value_parser: self.value_parser,
            skip_parser: self.skip_parser,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PV, PS, TV, TS> Parser<'i, TV> for SkipAll<PV, PS, TV, TS> where PV: Parser<'i, TV>, PS: Parser<'i, TS> {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, TV> {
        match self.value_parser.parse(input) {
            ParseResult::Good(tv, mut input) => {
                while let ParseResult::Good(_, new_input) = self.skip_parser.parse(input) {
                    input = new_input
                }

                ParseResult::Good(tv, input)
            }
            ParseResult::Bad(err) => ParseResult::Bad(err)
        }
    }

    #[inline]
    fn parse_into<G>(&self, input: &'i [u8], target: &mut G, index: usize) -> ParseResult<'i, bool> where G: GatherTarget<TV> {
        match self.value_parser.parse_into(input, target, index) {
            ParseResult::Good(full, mut input) => {
                while let ParseResult::Good(_, new_input) = self.skip_parser.parse(input) {
                    input = new_input
                }

                ParseResult::Good(full, input)
            }
            ParseResult::Bad(err) => ParseResult::Bad(err)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::unsigned_int;
    use super::*;

    #[test]
    fn skip_skips_as_it_should() {
        assert_eq!(
            b"Hello".then_skip(b',').then_skip(b' ').parse(b"Hello, World"),
            ParseResult::Good(b"Hello".as_slice(), b"World"),
        );
        assert_eq!(
            b"Hello,".then_skip_all(b' ').parse(b"Hello,    World"),
            ParseResult::Good(b"Hello,".as_slice(), b"World"),
        );
    }

    #[test]
    fn skip_preserves_error() {
        assert_eq!(
            b"Hello".then_skip(b' ').parse(b"Greetings, Earth"),
            b"Hello".parse(b"Greetings, Earth"),
        );
        assert_eq!(
            b"Hello".then_skip_all(b' ').parse(b"Greetings, Earth"),
            b"Hello".parse(b"Greetings, Earth"),
        );
        assert_eq!(
            unsigned_int::<i16>().then_skip(b' ').parse(b"Sixteen"),
            unsigned_int::<i16>().parse(b"Sixteen"),
        );
        assert_eq!(
            unsigned_int::<i16>().then_skip_all(b' ').parse(b"Sixteen"),
            unsigned_int::<i16>().parse(b"Sixteen"),
        );
    }
}
