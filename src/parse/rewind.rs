use super::Parser;
use crate::parse::ParseResult;
use std::marker::PhantomData;

pub struct Rewind<P, T> {
    parser: P,
    spooky_ghost: PhantomData<T>,
}

impl<P, T> Rewind<P, T> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, P, T> Parser<'i, T> for Rewind<P, T>
where
    P: Parser<'i, T>,
{
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        match self.parser.parse(input) {
            ParseResult::Good(v, _) => ParseResult::Good(v, input),
            ParseResult::Bad(err) => ParseResult::Bad(err),
        }
    }

    fn can_parse(&self, input: &'i [u8]) -> ParseResult<'i, ()> {
        match self.parser.can_parse(input) {
            ParseResult::Good(..) => ParseResult::Good((), input),
            ParseResult::Bad(err) => ParseResult::Bad(err),
        }
    }
}

impl<P, T> Copy for Rewind<P, T> where P: Copy {}

impl<P, T> Clone for Rewind<P, T>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            spooky_ghost: Default::default(),
        }
    }
}
