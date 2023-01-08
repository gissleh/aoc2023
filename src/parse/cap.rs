use std::marker::PhantomData;
use crate::parse::{ParseError, Parser, ParseResult};

pub struct RightCap<PB, TB, PC, TC> {
    body_parser: PB,
    cap_parser: PC,
    spooky_ghost: PhantomData<(TB, TC)>,
}

impl<'i, PB, TB, PC, TC> RightCap<PB, TB, PC, TC> where PB: Parser<'i, TB>, PC: Parser<'i, TC> {
    pub fn new(body_parser: PB, cap_parser: PC) -> Self {
        RightCap { body_parser, cap_parser, spooky_ghost: PhantomData::default() }
    }
}

impl<PB, TB, PC, TC> Copy for RightCap<PB, TB, PC, TC> where PB: Copy, PC: Copy {}

impl<PB, TB, PC, TC> Clone for RightCap<PB, TB, PC, TC> where PB: Clone, PC: Clone {
    fn clone(&self) -> Self {
        Self {
            body_parser: self.body_parser.clone(),
            cap_parser: self.cap_parser.clone(),
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PB, TB, PC, TC> Parser<'i, TB> for RightCap<PB, TB, PC, TC> where PB: Parser<'i, TB>, PC: Parser<'i, TC> {
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, TB> {
        match self.cap_parser.first_parsable_in(input) {
            ParseResult::Good((_, pos), after_cap) => match self.body_parser.parse(&input[..pos]) {
                ParseResult::Good(v, new_input) => if new_input.len() == 0 {
                    ParseResult::Good(v, after_cap)
                } else {
                    ParseResult::Bad(ParseError::new("RightCap's content was not consumed", input))
                }
                ParseResult::Bad(err) => ParseResult::Bad(err)
            }
            ParseResult::Bad(err) => ParseResult::Bad(err.wrap("Until parser result not found", input))
        }
    }
}