use std::marker::PhantomData;
use crate::parse::{Parser, ParseResult};

pub struct Vanguard<P, T, PV, TV> {
    value_parser: P,
    vanguard_parser: PV,
    spooky_ghost: PhantomData<(T, TV)>,
}

impl<P, T, PV, TV> Vanguard<P, T, PV, TV> {
    #[inline]
    pub fn new(value_parser: P, vanguard_parser: PV) -> Self {
        Self { value_parser, vanguard_parser, spooky_ghost: Default::default() }
    }
}

impl<P, T, PV, TV> Copy for Vanguard<P, T, PV, TV> where P: Copy, PV: Copy {}

impl<P, T, PV, TV> Clone for Vanguard<P, T, PV, TV> where P: Clone, PV: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            vanguard_parser: self.vanguard_parser.clone(),
            value_parser: self.value_parser.clone(),
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, P, T, PV, TV> Parser<'i, T> for Vanguard<P, T, PV, TV> where P: Parser<'i, T>, PV: Parser<'i, TV> {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        match self.vanguard_parser.parse(input) {
            ParseResult::Good(..) => self.value_parser.parse(input),
            ParseResult::Bad(err) => ParseResult::Bad(
                err.wrap("Vanguard failed", input),
            ),
        }
    }

    #[inline]
    fn first_parsable_in(&self, input: &'i [u8]) -> ParseResult<'i, (T, usize)> {
        match self.vanguard_parser.first_parsable_in(input) {
            ParseResult::Good((_, pos), _) => match self.value_parser.first_parsable_in(&input[pos..]) {
                ParseResult::Good((v, pos2), new_input) => ParseResult::Good((v, pos + pos2), new_input),
                bad_result => bad_result,
            },
            ParseResult::Bad(err) => ParseResult::Bad(
                err.wrap("Vanguard failed", input),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::{ParseError, Parser, ParseResult, unsigned_int};
    use crate::parse::vanguard::Vanguard;

    #[test]
    fn vanguard_vanguards() {
        let vanguard = Vanguard::new(unsigned_int::<u32>(), b'4');
        let without_vanguard = vanguard.value_parser;

        assert_eq!(
            vanguard.first_parsable_in(b"There's a number in this text, it is not 27, but 42!"),
            ParseResult::Good((42, 49), b"!"),
        );
        assert_eq!(
            vanguard.parse(b"42!"),
            ParseResult::Good(42, b"!"),
        );

        assert_eq!(vanguard.parse(b"42!"), without_vanguard.parse(b"42!"));
        assert_eq!(vanguard.parse(b"three"), ParseResult::Bad(
            ParseError::new("u8 not matched", b"three")
                .wrap("Vanguard failed", b"three"),
        ));
    }
}