use std::marker::PhantomData;
use crate::parse::{ParseError, Parser, ParseResult};

trait RepeatInto<T> {
    fn start_collecting(size_hint: usize) -> Self;
    fn add_parsed_value(&mut self, index: usize, value: T) -> bool;
}

pub struct Repeat<P, T, R> {
    amount: usize,
    parser: P,
    spoops: PhantomData<(T, R)>,
}

impl<P, T, R> Repeat<P, T, R> {
    #[inline]
    pub(crate) fn new(parser: P) -> Self {
        Self::with_amount(parser, 0)
    }

    #[inline]
    pub(crate) fn with_amount(parser: P, amount: usize) -> Self {
        Self {
            amount,
            parser,
            spoops: PhantomData::default(),
        }
    }
}

impl<P, T, R> Copy for Repeat<P, T, R> where P: Copy {}

impl<P, T, R> Clone for Repeat<P, T, R> where P: Clone {
    fn clone(&self) -> Self {
        Self { parser: self.parser.clone(), amount: self.amount, spoops: Default::default() }
    }
}

impl<'i, P, R, T> Parser<'i, R> for Repeat<P, T, R> where P: Parser<'i, T>, R: RepeatInto<T> {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, R> {
        match self.parser.parse(input) {
            ParseResult::Good(res, mut current_input) => {
                let mut target = R::start_collecting(0);
                let mut index = 1;
                target.add_parsed_value(0, res);

                while let ParseResult::Good(res, new_input) = self.parser.parse(current_input) {
                    let full = target.add_parsed_value(index, res);
                    current_input = new_input;
                    index += 1;

                    if full || index == self.amount {
                        break;
                    }
                }

                if self.amount > 0 && index != self.amount {
                    return ParseResult::Bad(ParseError::new("Target amount in Repeat not met", input));
                }

                ParseResult::Good(target, current_input)
            }
            ParseResult::Bad(err) => ParseResult::Bad(err.wrap("Repeat failed on first")),
        }
    }
}

impl<T> RepeatInto<T> for Vec<T> {
    fn start_collecting(size_hint: usize) -> Self {
        Vec::with_capacity(size_hint)
    }

    fn add_parsed_value(&mut self, _index: usize, value: T) -> bool {
        self.push(value);
        false
    }
}

impl<const N: usize, T> RepeatInto<T> for [T; N] where T: Default + Copy {
    fn start_collecting(_size_hint: usize) -> Self {
        [T::default(); N]
    }

    fn add_parsed_value(&mut self, index: usize, value: T) -> bool {
        self[index] = value;
        index == self.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_repeats() {}
}