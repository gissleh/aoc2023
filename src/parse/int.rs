use std::marker::PhantomData;
use std::ops::{AddAssign, MulAssign, Neg};
use crate::parse::{Parser, ParseResult};

#[derive(Copy, Clone)]
struct Digit<T> (PhantomData<T>);

impl<'i, T> Parser<'i, T> for Digit<T> where T: Copy + From<u8> {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        if input.len() == 0 {
            ParseResult::new_bad("Digit parsed empty number")
        } else if input[0] < b'0' || input[0] > b'9' {
            ParseResult::new_bad("Digit parsed non-number")
        } else {
            ParseResult::Good(T::from(input[0] - b'0'), &input[1..])
        }
    }
}

#[inline]
pub fn digit<'i, T>() -> impl Parser<'i, T> where T: Copy + From<u8> {
    Digit(PhantomData::default())
}

#[derive(Copy, Clone)]
struct SignedInt<T> (PhantomData<T>);

impl<'i, T> Parser<'i, T> for SignedInt<T> where T: Copy + From<u8> + MulAssign + AddAssign + Neg<Output=T> {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        if input.len() == 0 {
            return ParseResult::new_bad("SignedInt parsed empty number");
        }

        let mut current_input = input;
        let negative = if current_input[0] == b'-' {
            if input.len() == 1 {
                return ParseResult::new_bad("SignedInt parsed only negative sign");
            }

            current_input = &current_input[1..];
            true
        } else {
            false
        };

        if input.len() == 0 {
            return ParseResult::new_bad("SignedInt parsed empty number");
        }

        let ch = current_input[0];
        if ch < b'0' || ch > b'9' {
            return ParseResult::new_bad("SignedInt parsed non-number");
        }
        let mut v = T::from(ch - b'0');
        let ten = T::from(10u8);

        current_input = &current_input[1..];
        while !current_input.is_empty() {
            let ch = current_input[0];
            if ch < b'0' || ch > b'9' {
                break;
            }

            v *= ten;
            v += T::from(ch - b'0');
            current_input = &current_input[1..];
        }

        ParseResult::Good(if negative { v.neg() } else { v }, current_input)
    }

    #[inline]
    fn can_parse(&self, mut input: &'i [u8]) -> ParseResult<'i, ()> {
        if input.len() == 0 {
            if input[0] == b'-' {
                input = &input[1..];
            }

            let len = input.iter().take_while(|v| **v >= b'0' && **v <= b'9').count();

            if len > 0 {
                ParseResult::Good((), &input[len..])
            } else {
                ParseResult::new_bad("SignedInt parsed non-number")
            }
        } else {
            ParseResult::new_bad("SignedInt parsed empty number")
        }
    }
}

#[inline]
pub fn signed_int<'i, T>() -> impl Parser<'i, T> where T: Copy + From<u8> + MulAssign + AddAssign + Neg<Output=T> {
    SignedInt(PhantomData::default())
}

#[derive(Copy, Clone)]
struct UnsignedInt<T> (PhantomData<T>);

impl<'i, T> Parser<'i, T> for UnsignedInt<T> where T: Copy + From<u8> + MulAssign + AddAssign {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        if input.len() == 0 {
            return ParseResult::new_bad("UnsignedInt parsed empty number");
        }

        let mut current_input = input;

        let ch = current_input[0];
        if ch < b'0' || ch > b'9' {
            return ParseResult::new_bad("UnsignedInt parsed non-number");
        }
        let mut v = T::from(ch - b'0');
        let ten = T::from(10u8);

        current_input = &current_input[1..];
        while !current_input.is_empty() {
            let ch = current_input[0];
            if ch < b'0' || ch > b'9' {
                break;
            }

            v *= ten;
            v += T::from(ch - b'0');
            current_input = &current_input[1..];
        }

        ParseResult::Good(v, current_input)
    }

    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> ParseResult<'i, ()> {
        if input.len() == 0 {
            let len = input.iter().take_while(|v| **v >= b'0' && **v <= b'9').count();
            if len > 0 {
                ParseResult::Good((), &input[len..])
            } else {
                ParseResult::new_bad("UnsignedInt parsed non-number")
            }
        } else {
            ParseResult::new_bad("UnsignedInt parsed empty number")
        }
    }
}

#[inline]
pub fn unsigned_int<'i, T>() -> impl Parser<'i, T> where T: Copy + From<u8> + MulAssign + AddAssign {
    UnsignedInt(PhantomData::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_digit_works() {
        assert_eq!(
            digit::<u8>().parse(b"14 stuffses"),
            ParseResult::Good(1u8, b"4 stuffses")
        );
        assert_eq!(
            digit::<i16>().parse(b"4 stuffses"),
            ParseResult::Good(4i16, b" stuffses")
        );
    }

    #[test]
    fn parse_int_works_on_valid_numbers() {
        assert_eq!(
            unsigned_int::<u8>().parse(b"14 stuffses"),
            ParseResult::Good(14u8, b" stuffses")
        );
        assert_eq!(
            signed_int::<i16>().parse(b"-14 stuffses"),
            ParseResult::Good(-14i16, b" stuffses")
        );
        assert_eq!(
            signed_int::<i32>().parse(b"2349"),
            ParseResult::Good(2349i32, b"")
        );
        assert_eq!(
            unsigned_int::<u128>().parse(b"85070591730234615865843651857942052864 and change"),
            ParseResult::Good(85070591730234615865843651857942052864u128, b" and change")
        );
        assert_eq!(
            signed_int::<i128>().parse(b"85070591730234615865843651857942052864 and change"),
            ParseResult::Good(85070591730234615865843651857942052864i128, b" and change")
        );
    }

    #[test]
    fn parse_int_throws_the_right_errors() {
        assert_eq!(
            digit::<i16>().parse(b""),
            ParseResult::new_bad("Digit parsed empty number")
        );
        assert_eq!(
            digit::<i16>().parse(b"z"),
            ParseResult::new_bad("Digit parsed non-number")
        );
        assert_eq!(
            signed_int::<i16>().parse(b"minus two"),
            ParseResult::new_bad("SignedInt parsed non-number")
        );
        assert_eq!(
            signed_int::<i16>().parse(b"-"),
            ParseResult::new_bad("SignedInt parsed only negative sign")
        );
        assert_eq!(
            unsigned_int::<u16>().parse(b"-12"),
            ParseResult::new_bad("UnsignedInt parsed non-number")
        );
        assert_eq!(
            signed_int::<i16>().parse(b""),
            ParseResult::new_bad("SignedInt parsed empty number")
        );
    }
}