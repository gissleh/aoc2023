use crate::parse::{ParseResult, Parser};
use std::marker::PhantomData;

pub struct QuotedBy<PB, TB, PL, TL, PR, TR> {
    body_parser: PB,
    lq_parser: PL,
    rq_parser: PR,
    spooky_ghost: PhantomData<(TB, TL, TR)>,
}

impl<'i, PB, TB, PL, TL, PR, TR> QuotedBy<PB, TB, PL, TL, PR, TR>
where
    PB: Parser<'i, TB>,
    PL: Parser<'i, TL>,
    PR: Parser<'i, TR>,
{
    pub fn new(body_parser: PB, lq_parser: PL, rq_parser: PR) -> Self {
        Self {
            body_parser,
            lq_parser,
            rq_parser,
            spooky_ghost: Default::default(),
        }
    }
}

impl<PB, TB, PL, TL, PR, TR> Copy for QuotedBy<PB, TB, PL, TL, PR, TR>
where
    PB: Copy,
    PL: Copy,
    PR: Copy,
{
}

impl<PB, TB, PL, TL, PR, TR> Clone for QuotedBy<PB, TB, PL, TL, PR, TR>
where
    PB: Clone,
    PL: Clone,
    PR: Clone,
{
    fn clone(&self) -> Self {
        Self {
            body_parser: self.body_parser.clone(),
            lq_parser: self.lq_parser.clone(),
            rq_parser: self.rq_parser.clone(),
            spooky_ghost: PhantomData::default(),
        }
    }
}

impl<'i, PB, TB, PL, TL, PR, TR> Parser<'i, TB> for QuotedBy<PB, TB, PL, TL, PR, TR>
where
    PB: Parser<'i, TB>,
    PL: Parser<'i, TL>,
    PR: Parser<'i, TR>,
{
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, TB> {
        match self.lq_parser.parse(input) {
            ParseResult::Good(_, new_input) => match self.rq_parser.first_parsable_in(new_input) {
                ParseResult::Good((_, cap_pos), input_after) => {
                    match self.body_parser.parse(&new_input[..cap_pos]) {
                        ParseResult::Good(v, new_input) => {
                            if new_input.len() == 0 {
                                ParseResult::Good(v, input_after)
                            } else {
                                ParseResult::new_bad("QuotedBy body was not exhausted")
                            }
                        }
                        ParseResult::Bad(err) => ParseResult::wrap_bad(err, "QuotedBy Body Failed"),
                    }
                }
                ParseResult::Bad(err) => ParseResult::wrap_bad(err, "QuotedBy RQ Failed"),
            },
            ParseResult::Bad(err) => ParseResult::wrap_bad(err, "QuotedBy LQ Failed"),
        }
    }
}

pub struct CappedBy<PB, TB, PC, TC> {
    body_parser: PB,
    cap_parser: PC,
    spooky_ghost: PhantomData<(TB, TC)>,
}

impl<'i, PB, TB, PC, TC> CappedBy<PB, TB, PC, TC>
where
    PB: Parser<'i, TB>,
    PC: Parser<'i, TC>,
{
    pub fn new(body_parser: PB, cap_parser: PC) -> Self {
        CappedBy {
            body_parser,
            cap_parser,
            spooky_ghost: PhantomData::default(),
        }
    }
}

impl<PB, TB, PC, TC> Copy for CappedBy<PB, TB, PC, TC>
where
    PB: Copy,
    PC: Copy,
{
}

impl<PB, TB, PC, TC> Clone for CappedBy<PB, TB, PC, TC>
where
    PB: Clone,
    PC: Clone,
{
    fn clone(&self) -> Self {
        Self {
            body_parser: self.body_parser.clone(),
            cap_parser: self.cap_parser.clone(),
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PB, TB, PC, TC> Parser<'i, TB> for CappedBy<PB, TB, PC, TC>
where
    PB: Parser<'i, TB>,
    PC: Parser<'i, TC>,
{
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, TB> {
        match self.cap_parser.first_parsable_in(input) {
            ParseResult::Good((_, pos), after_cap) => match self.body_parser.parse(&input[..pos]) {
                ParseResult::Good(v, new_input) => {
                    if new_input.len() == 0 {
                        ParseResult::Good(v, after_cap)
                    } else {
                        ParseResult::new_bad("RightCap's content was not consumed")
                    }
                }
                ParseResult::Bad(err) => ParseResult::Bad(err),
            },
            ParseResult::Bad(err) => ParseResult::wrap_bad(err, "Until parser result not found"),
        }
    }

    fn can_parse(&self, input: &'i [u8]) -> ParseResult<'i, ()> {
        match self.cap_parser.first_parsable_in(input) {
            ParseResult::Good((_, pos), after_cap) => {
                match self.body_parser.can_parse(&input[..pos]) {
                    ParseResult::Good(_, new_input) => {
                        if new_input.len() == 0 {
                            ParseResult::Good((), after_cap)
                        } else {
                            ParseResult::new_bad("RightCap's content was not consumed")
                        }
                    }
                    ParseResult::Bad(err) => ParseResult::Bad(err),
                }
            }
            ParseResult::Bad(err) => ParseResult::wrap_bad(err, "Until parser result not found"),
        }
    }
}
