use std::marker::PhantomData;
use arrayvec::ArrayVec;
use smallvec::{Array, SmallVec};
use crate::parse::{ParseError, Parser, ParseResult};

pub struct Repeat<P, T, R> {
    amount: usize,
    parser: P,
    spooky_ghost: PhantomData<(T, R)>,
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
            spooky_ghost: PhantomData::default(),
        }
    }
}

impl<P, T, R> Copy for Repeat<P, T, R> where P: Copy {}

impl<P, T, R> Clone for Repeat<P, T, R> where P: Clone {
    fn clone(&self) -> Self {
        Self { parser: self.parser.clone(), amount: self.amount, spooky_ghost: Default::default() }
    }
}

impl<'i, P, R, T> Parser<'i, R> for Repeat<P, T, R> where P: Parser<'i, T>, R: RepeatInto<T> {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, R> {
        match self.parser.parse(input) {
            ParseResult::Good(res, mut current_input) => {
                let mut target = R::create_collection(self.amount);
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
            ParseResult::Bad(err) => ParseResult::Bad(err.wrap("Repeat failed on first", input)),
        }
    }
}

pub struct RepeatDelimited<PV, TV, PD, TD, R> {
    amount: usize,
    value_parser: PV,
    delimiter_parser: PD,
    spooky_ghost: PhantomData<(TV, TD, R)>,
}

impl<PV, TV, PD, TD, R> RepeatDelimited<PV, TV, PD, TD, R> {
    #[inline]
    pub(crate) fn new(value_parser: PV, delimiter_parser: PD) -> Self {
        Self::with_amount(value_parser, delimiter_parser, 0)
    }

    #[inline]
    pub(crate) fn with_amount(value_parser: PV, delimiter_parser: PD, amount: usize) -> Self {
        Self {
            amount,
            value_parser,
            delimiter_parser,
            spooky_ghost: PhantomData::default(),
        }
    }
}

impl<'i, PV, TV, PD, TD, R> Copy for RepeatDelimited<PV, TV, PD, TD, R> where PV: Parser<'i, TV>, PD: Parser<'i, TD> {}

impl<'i, PV, TV, PD, TD, R> Clone for RepeatDelimited<PV, TV, PD, TD, R> where PV: Parser<'i, TV>, PD: Parser<'i, TD> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            amount: self.amount,
            value_parser: self.value_parser.clone(),
            delimiter_parser: self.delimiter_parser.clone(),
            spooky_ghost: PhantomData::default(),
        }
    }
}

impl<'i, PV, TV, PD, TD, R> Parser<'i, R> for RepeatDelimited<PV, TV, PD, TD, R> where PV: Parser<'i, TV>, PD: Parser<'i, TD>, R: RepeatInto<TV> {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, R> {
        match self.value_parser.parse(input) {
            ParseResult::Good(res, mut current_input) => {
                let mut target = R::create_collection(self.amount);
                let mut index = 1;
                target.add_parsed_value(0, res);

                match self.delimiter_parser.parse(current_input) {
                    ParseResult::Good(_, new_input) => {
                        current_input = new_input;
                        while let ParseResult::Good(res, new_input) = self.value_parser.parse(current_input) {
                            let full = target.add_parsed_value(index, res);
                            current_input = new_input;
                            index += 1;

                            if full || index == self.amount {
                                break;
                            }

                            match self.delimiter_parser.parse(current_input) {
                                ParseResult::Good(_, new_input) => { current_input = new_input; }
                                ParseResult::Bad(_) => { break; }
                            }
                        }
                    }
                    ParseResult::Bad(_) => {}
                }

                if self.amount > 0 && index != self.amount {
                    return ParseResult::Bad(ParseError::new("Target amount in Repeat not met", input));
                }

                ParseResult::Good(target, current_input)
            }
            ParseResult::Bad(err) => ParseResult::Bad(err.wrap("Repeat failed on first", input)),
        }
    }
}

trait RepeatInto<T> {
    fn create_collection(size_hint: usize) -> Self;
    fn add_parsed_value(&mut self, index: usize, value: T) -> bool;
}

impl<T> RepeatInto<T> for Vec<T> {
    fn create_collection(size_hint: usize) -> Self {
        Vec::with_capacity(size_hint)
    }

    fn add_parsed_value(&mut self, _index: usize, value: T) -> bool {
        self.push(value);
        false
    }
}

impl<const N: usize, T> RepeatInto<T> for [T; N] where T: Default + Copy {
    fn create_collection(_size_hint: usize) -> Self {
        [T::default(); N]
    }

    fn add_parsed_value(&mut self, index: usize, value: T) -> bool {
        self[index] = value;
        index == self.len() - 1
    }
}

impl<const N: usize, T> RepeatInto<T> for ([T; N], usize) where T: Default + Copy {
    fn create_collection(_size_hint: usize) -> Self {
        ([T::default(); N], 0)
    }

    fn add_parsed_value(&mut self, index: usize, value: T) -> bool {
        self.0[index] = value;
        self.1 = index + 1;
        index == self.0.len() - 1
    }
}

impl<const N: usize, T> RepeatInto<T> for ArrayVec<T, N> {
    fn create_collection(_size_hint: usize) -> Self {
        ArrayVec::new()
    }

    fn add_parsed_value(&mut self, _index: usize, value: T) -> bool {
        self.push(value);
        self.is_full()
    }
}

impl<A, T> RepeatInto<T> for SmallVec<A> where A: Array<Item=T> {
    fn create_collection(_size_hint: usize) -> Self {
        SmallVec::new()
    }

    fn add_parsed_value(&mut self, _index: usize, value: T) -> bool {
        self.push(value);
        false
    }
}

impl<T> RepeatInto<T> for (T, T) where T: Default {
    fn create_collection(_size_hint: usize) -> Self { (T::default(), T::default()) }

    fn add_parsed_value(&mut self, index: usize, value: T) -> bool {
        match index {
            0 => self.0 = value,
            1 => self.1 = value,
            _ => {}
        }
        index == 1
    }
}

impl<T> RepeatInto<T> for (T, T, T) where T: Default {
    fn create_collection(_size_hint: usize) -> Self { (T::default(), T::default(), T::default()) }

    fn add_parsed_value(&mut self, index: usize, value: T) -> bool {
        match index {
            0 => self.0 = value,
            1 => self.1 = value,
            2 => self.2 = value,
            _ => {}
        }
        index == 2
    }
}

impl<T> RepeatInto<T> for (T, T, T, T) where T: Default {
    fn create_collection(_size_hint: usize) -> Self { (T::default(), T::default(), T::default(), T::default()) }

    fn add_parsed_value(&mut self, index: usize, value: T) -> bool {
        match index {
            0 => self.0 = value,
            1 => self.1 = value,
            2 => self.2 = value,
            3 => self.3 = value,
            _ => {}
        }
        index == 3
    }
}


#[cfg(test)]
mod tests {
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
            // This is ugly, but this generic parameter should be entirely inferred.
            unsigned_int::<u32>().repeat_delimited::<(_, _, _), _, _>(b',').parse(b"473,1123,5932"),
            ParseResult::Good((473, 1123, 5932), b""),
        );

        // This is the way it should be done.
        let v: ParseResult<([u32; 8], usize)> = unsigned_int::<u32>()
            .repeat_delimited(b',')
            .parse(b"1,2,8,64,234,221");

        assert_eq!(v, ParseResult::Good(([1,2,8,64,234,221,0,0], 6), b""));
    }
}