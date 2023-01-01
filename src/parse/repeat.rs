use std::marker::PhantomData;
use crate::parse::{ParseError, Parser, ParseResult};
use crate::utils::gather_target::GatherTarget;

pub struct Repeat<P, T, G> {
    parser: P,
    amount: usize,
    spooky_ghost: PhantomData<(G, T)>,
}

impl<P, T, G> Repeat<P, T, G> {
    pub(crate) fn new(parser: P, amount: usize) -> Self {
        Self { parser, amount, spooky_ghost: PhantomData::default() }
    }
}

impl<P, T, G> Copy for Repeat<P, T, G> where P: Copy {}

impl<P, T, G> Clone for Repeat<P, T, G> where P: Clone {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            amount: self.amount,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, P, T, G> Parser<'i, G> for Repeat<P, T, G> where P: Parser<'i, T>, G: GatherTarget<T> {
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, G> {
        let mut target = G::start_gathering(self.amount);

        match self.parser.parse_into(input, &mut target, 0) {
            ParseResult::Good(full, mut current_input) => {
                if full || self.amount == 1 {
                    return ParseResult::Good(target, current_input);
                }

                let mut index = 1;
                while let ParseResult::Good(full, new_input) = self.parser.parse_into(current_input, &mut target, index) {
                    current_input = new_input;
                    index += 1;
                    if full {
                        if self.amount != 0 && index != self.amount {
                            return ParseResult::Bad(ParseError::new("Container was full before amount was met.", input));
                        }

                        break;
                    } else if index == self.amount {
                        break;
                    }
                }

                ParseResult::Good(target, current_input)
            }
            ParseResult::Bad(err) => ParseResult::Bad(err.wrap("Failed to parse first in Repeat", input))
        }
    }
}

pub struct DelimitedBy<PB, PD, TB, TD> {
    parser_body: PB,
    parser_delim: PD,
    spooky_ghost: PhantomData<(TB, TD)>,
}

impl<PB, PD, TB, TD> DelimitedBy<PB, PD, TB, TD> {
    pub(crate) fn new(parser_body: PB, parser_delim: PD) -> Self {
        Self { parser_body, parser_delim, spooky_ghost: PhantomData::default() }
    }
}

impl<PB, PD, TB, TD> Copy for DelimitedBy<PB, PD, TB, TD> where PB: Copy, PD: Copy {}

impl<PB, PD, TB, TD> Clone for DelimitedBy<PB, PD, TB, TD> where PB: Clone, PD: Clone {
    fn clone(&self) -> Self {
        Self {
            parser_body: self.parser_body.clone(),
            parser_delim: self.parser_delim.clone(),
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PB, PD, TB, TD> Parser<'i, TB> for DelimitedBy<PB, PD, TB, TD> where PB: Parser<'i, TB>, PD: Parser<'i, TD> {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, TB> {
        self.parser_body.parse(input)
    }

    #[inline]
    fn parse_into<G>(&self, input: &'i [u8], target: &mut G, index: usize) -> ParseResult<'i, bool> where G: GatherTarget<TB> {
        if index > 0 {
            match self.parser_delim.parse(input) {
                ParseResult::Good(_, new_input) => {
                    match self.parser_body.parse_into(new_input, target, index) {
                        ParseResult::Bad(err) => ParseResult::Bad(err.wrap("Failed to parse body after delimiter", input)),
                        good_res => good_res,
                    }
                }
                ParseResult::Bad(err) => ParseResult::Bad(err.wrap("Delimiter not found", input))
            }
        } else {
            self.parser_body.parse_into(input, target, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use arrayvec::ArrayVec;
    use crate::parse::unsigned_int;
    use super::*;

    #[test]
    fn repeat_repeats() {
        assert_eq!(
            b'a'.repeat::<Vec<_>>().parse(b"aaaaaabcd"),
            ParseResult::Good(b"aaaaaa".to_vec(), b"bcd"),
        );
        assert_eq!(
            b'a'.repeat::<ArrayVec<_, 4>>().parse(b"aaaaaabcd"),
            ParseResult::Good(ArrayVec::from(*b"aaaa"), b"aabcd"),
        );
        assert_eq!(
            b'a'.repeat_n::<[u8; 6]>(4).parse(b"aaaaaabcd"),
            ParseResult::Good([b'a', b'a', b'a', b'a', 0, 0], b"aabcd"),
        );
        assert_eq!(
            b'a'.repeat::<([_; 8], usize)>().parse(b"aaaaaabcd"),
            ParseResult::Good(([b'a', b'a', b'a', b'a', b'a', b'a', 0, 0], 6usize), b"bcd"),
        );
        assert_eq!(
            unsigned_int::<u32>().then_skip(b',').repeat::<(_, _, _)>().parse(b"473,1123,5932,9684"),
            ParseResult::Good((473, 1123, 5932), b"9684"),
        );
    }

    #[test]
    fn repeat_repeats_with_delimiter() {
        assert_eq!(
            // This was ugly, but now it's easier to define a parameter here.
            unsigned_int::<u32>().delimited_by(b',').repeat::<(_, _, _)>().parse(b"473,1123,5932"),
            ParseResult::Good((473, 1123, 5932), b""),
        );

        // The repeat could also have its type inferred like collect.
        let v: ParseResult<([u32; 8], usize)> = unsigned_int::<u32>()
            .delimited_by(b',')
            .repeat()
            .parse(b"1,2,8,64,234,221");

        assert_eq!(v, ParseResult::Good(([1,2,8,64,234,221,0,0], 6), b""));
    }
}