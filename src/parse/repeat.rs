use std::marker::PhantomData;
use crate::parse::{Parser, ParseResult};
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
                            return ParseResult::new_bad("Container was full before amount was met.");
                        }

                        break;
                    } else if index == self.amount {
                        break;
                    }
                }

                ParseResult::Good(target, current_input)
            }
            ParseResult::Bad(err) => ParseResult::wrap_bad(err, "Failed to parse first in Repeat")
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

    fn parse_at_index(&self, input: &'i [u8], index: usize) -> ParseResult<'i, TB> {
        if index > 0 {
            match self.parser_delim.parse(input) {
                ParseResult::Good(_, new_input) => {
                    match self.parser_body.parse_at_index(new_input, index) {
                        ParseResult::Bad(err) => ParseResult::wrap_bad(err, "Failed to parse body after delimiter"),
                        good_res => good_res,
                    }
                }
                ParseResult::Bad(err) => ParseResult::wrap_bad(err, "Delimiter not found")
            }
        } else {
            self.parser_body.parse_at_index(input, 0)
        }
    }
}

pub struct Count<P, T> {
    parser: P,
    spooky_ghost: PhantomData<T>,
}

impl<'i, P, T> Count<P, T> where P: Parser<'i, T> {
    pub fn new(parser: P) -> Self {
        Self { parser, spooky_ghost: Default::default() }
    }
}

impl<P, T> Copy for Count<P, T> where P: Copy {}

impl<P, T> Clone for Count<P, T> where P: Clone {
    fn clone(&self) -> Self {
        Self { parser: self.parser.clone(), spooky_ghost: Default::default() }
    }
}

impl<'i, P, T> Parser<'i, usize> for Count<P, T> where P: Parser<'i, T> {
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, usize> {
        match self.parser.parse_at_index(input, 0) {
            ParseResult::Good(_, mut current_input) => {
                let mut count = 1;
                while let ParseResult::Good(_, new_input) = self.parser.parse_at_index(current_input, count) {
                    current_input = new_input;
                    count += 1;
                }

                ParseResult::Good(count, current_input)
            }
            ParseResult::Bad(err) => ParseResult::wrap_bad(err, "Failed to parse first in Count")
        }
    }
}

pub struct RepeatFold<P, FI, FS, TP, TA> {
    parser: P,
    init_func: FI,
    step_func: FS,
    spooky_ghost: PhantomData<(TP, TA)>,
}

impl<P, FI, FS, TP, TA> RepeatFold<P, FI, FS, TP, TA>
    where FI: Fn() -> TA + Copy,
          FS: Fn(TA, TP) -> TA + Copy {
    pub fn new(parser: P, init_func: FI, step_func: FS) -> Self {
        Self{parser, init_func, step_func, spooky_ghost: Default::default()}
    }
}

impl<P, FI, FS, TP, TA> Copy for RepeatFold<P, FI, FS, TP, TA> where P: Copy, FI: Copy, FS: Copy {}

impl<P, FI, FS, TP, TA> Clone for RepeatFold<P, FI, FS, TP, TA> where P: Clone, FI: Clone, FS: Clone {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            init_func: self.init_func.clone(),
            step_func: self.step_func.clone(),
            spooky_ghost: self.spooky_ghost.clone(),
        }
    }
}

impl<'i, P, FI, FS, TP, TA> Parser<'i, TA> for RepeatFold<P, FI, FS, TP, TA>
    where P: Copy + Parser<'i, TP>,
          FI: Fn() -> TA + Copy,
          FS: Fn(TA, TP) -> TA + Copy {
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, TA> {
        let mut acc = (self.init_func)();

        match self.parser.parse_at_index(input, 0) {
            ParseResult::Good(res, mut current_input) => {
                let mut index = 1;

                acc = (self.step_func)(acc, res);
                while let ParseResult::Good(res, new_input) = self.parser.parse_at_index(current_input, index) {
                    acc = (self.step_func)(acc, res);
                    current_input = new_input;
                    index += 1;
                }

                ParseResult::Good(acc, current_input)
            }
            ParseResult::Bad(err) => ParseResult::wrap_bad(err, "Failed to parse first in RepeatFold")
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

        assert_eq!(v, ParseResult::Good(([1, 2, 8, 64, 234, 221, 0, 0], 6), b""));
    }
}