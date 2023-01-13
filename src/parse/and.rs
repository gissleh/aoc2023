use std::marker::PhantomData;
use crate::parse::{Parser, ParseResult};

pub struct And<PL, TL, PR, TR> {
    parse_left: PL,
    parse_right: PR,
    spooky_ghost: PhantomData<(TL, TR)>,
}

impl<'i, PL, TL, PR, TR> And<PL, TL, PR, TR> where PL: Parser<'i, TL>, PR: Parser<'i, TR> {
    #[inline]
    pub(crate) fn new(parse_left: PL, parse_right: PR) -> Self {
        Self { parse_left, parse_right, spooky_ghost: Default::default() }
    }
}

impl<PL, TL, PR, TR> Copy for And<PL, TL, PR, TR> where PL: Copy, PR: Copy {}

impl<PL, TL, PR, TR> Clone for And<PL, TL, PR, TR> where PL: Clone, PR: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            parse_left: self.parse_left.clone(),
            parse_right: self.parse_right.clone(),
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PL, TL, PR, TR> Parser<'i, (TL, TR)> for And<PL, TL, PR, TR> where PL: Parser<'i, TL>, PR: Parser<'i, TR> {
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, (TL, TR)> {
        match self.parse_left.parse(input) {
            ParseResult::Good(vl, input) => match self.parse_right.parse(input) {
                ParseResult::Good(vr, input) => ParseResult::Good((vl, vr), input),
                ParseResult::Bad(err) => ParseResult::wrap_bad(err, "Right in And failed")
            },
            ParseResult::Bad(err) => ParseResult::wrap_bad(err, "Left in And failed")
        }
    }
}

pub struct AndDiscard<PL, TL, PR, TR> {
    parse_left: PL,
    parse_right: PR,
    spooky_ghost: PhantomData<(TL, TR)>,
}

impl<'i, PL, TL, PR, TR> AndDiscard<PL, TL, PR, TR> where PL: Parser<'i, TL>, PR: Parser<'i, TR> {
    #[inline]
    pub(crate) fn new(parse_left: PL, parse_right: PR) -> Self {
        Self { parse_left, parse_right, spooky_ghost: Default::default() }
    }
}

impl<PL, TL, PR, TR> Copy for AndDiscard<PL, TL, PR, TR> where PL: Copy, PR: Copy {}

impl<PL, TL, PR, TR> Clone for AndDiscard<PL, TL, PR, TR> where PL: Clone, PR: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            parse_left: self.parse_left.clone(),
            parse_right: self.parse_right.clone(),
            spooky_ghost: Default::default(),
        }
    }
}


impl<'i, PL, TL, PR, TR> Parser<'i, TL> for AndDiscard<PL, TL, PR, TR> where PL: Parser<'i, TL>, PR: Parser<'i, TR> {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, TL> {
        match self.parse_left.parse(input) {
            ParseResult::Good(vl, input) => match self.parse_right.parse(input) {
                ParseResult::Good(_, input) => ParseResult::Good(vl, input),
                ParseResult::Bad(err) => ParseResult::wrap_bad(err, "Right in And failed")
            },
            ParseResult::Bad(err) => ParseResult::wrap_bad(err, "Left in And failed")
        }
    }
}

pub struct AndReplace<PL, TL, PR, TR> {
    parse_left: PL,
    parse_right: PR,
    spooky_ghost: PhantomData<(TL, TR)>,
}

impl<'i, PL, TL, PR, TR> AndReplace<PL, TL, PR, TR> where PL: Parser<'i, TL>, PR: Parser<'i, TR> {
    #[inline]
    pub(crate) fn new(parse_left: PL, parse_right: PR) -> Self {
        Self { parse_left, parse_right, spooky_ghost: Default::default() }
    }
}

impl<PL, TL, PR, TR> Copy for AndReplace<PL, TL, PR, TR> where PL: Copy, PR: Copy {}

impl<PL, TL, PR, TR> Clone for AndReplace<PL, TL, PR, TR> where PL: Clone, PR: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            parse_left: self.parse_left.clone(),
            parse_right: self.parse_right.clone(),
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PL, TL, PR, TR> Parser<'i, TR> for AndReplace<PL, TL, PR, TR> where PL: Parser<'i, TL>, PR: Parser<'i, TR> {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, TR> {
        match self.parse_left.parse(input) {
            ParseResult::Good(_, input) => match self.parse_right.parse(input) {
                ParseResult::Good(vr, input) => ParseResult::Good(vr, input),
                ParseResult::Bad(err) => ParseResult::wrap_bad(err, "Right in And failed")
            },
            ParseResult::Bad(err) => ParseResult::wrap_bad(err, "Left in And failed")
        }
    }
}
