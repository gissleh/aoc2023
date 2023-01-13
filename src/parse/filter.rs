use std::marker::PhantomData;
use std::ops::{Bound, RangeBounds};
use crate::parse::{Parser, ParseResult};

pub struct Filter<P, F, T> {
    parser: P,
    callback: F,
    spooky_ghost: PhantomData<T>,
}


impl<P, F, T> Copy for Filter<P, F, T> where P: Copy, F: Copy {}

impl<P, F, T> Clone for Filter<P, F, T> where P: Clone, F: Clone {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            callback: self.callback.clone(),
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, P, F, T> Filter<P, F, T> where P: Parser<'i, T>, F: Fn(&T) -> bool {
    #[inline]
    pub fn new(parser: P, callback: F) -> Self {
        Self { parser, callback, spooky_ghost: Default::default() }
    }
}

impl<'i, P, F, T> Parser<'i, T> for Filter<P, F, T> where P: Parser<'i, T>, F: Fn(&T) -> bool + Copy {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        match self.parser.parse(input) {
            ParseResult::Good(t, new_input) => {
                if (self.callback)(&t) {
                    ParseResult::Good(t, new_input)
                } else {
                    ParseResult::new_bad("Filter rejected parsed result")
                }
            }
            bad_result => bad_result,
        }
    }
}

pub struct InRange<P, T> {
    parser: P,
    lower: Bound<T>,
    upper: Bound<T>,
}

impl<'i, P, T> InRange<P, T> where P: Parser<'i, T>, T: Copy {
    #[inline]
    pub fn new<R>(parser: P, range: R) -> Self where R: RangeBounds<T> {
        Self { parser, lower: range.start_bound().cloned(), upper: range.end_bound().cloned() }
    }
}

impl<P, T> Copy for InRange<P, T> where P: Copy, T: Copy {}

impl<P, T> Clone for InRange<P, T> where P: Clone, T: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self { parser: self.parser.clone(), lower: self.lower.clone(), upper: self.upper.clone() }
    }
}

impl<'i, P, T> Parser<'i, T> for InRange<P, T> where P: Parser<'i, T>, T: Ord + Copy {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        match self.parser.parse(input) {
            ParseResult::Good(v, new_input) => {
                match self.lower {
                    Bound::Unbounded => {}
                    Bound::Excluded(l) => {
                        if v <= l {
                            return ParseResult::new_bad("Value too low");
                        }
                    }
                    Bound::Included(l) => {
                        if v < l {
                            return ParseResult::new_bad("Value too low");
                        }
                    }
                }

                match self.upper {
                    Bound::Unbounded => {}
                    Bound::Excluded(l) => {
                        if v >= l {
                            return ParseResult::new_bad("Value too high");
                        }
                    }
                    Bound::Included(l) => {
                        if v > l {
                            return ParseResult::new_bad("Value too high");
                        }
                    }
                }

                ParseResult::Good(v, new_input)
            }
            ParseResult::Bad(err) => ParseResult::wrap_bad(err, "Parser failed before range was involved"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::filter::InRange;
    use crate::parse::{Parser, ParseResult, signed_int};

    #[test]
    fn only_if_onlies_and_iffs() {
        let even_parser = signed_int::<i32>().only_if(|v| *v > 0 && *v & 1 == 0);

        assert_eq!(even_parser.parse(b"16"), ParseResult::Good(16, b""));
        assert_eq!(even_parser.parse(b"554"), ParseResult::Good(554, b""));
        assert_eq!(even_parser.parse(b"one"), signed_int::<i32>().parse(b"one"), "Should return the same as just passing it through.");
        assert_eq!(even_parser.parse(b"13"), ParseResult::new_bad("Filter rejected parsed result"));
    }

    #[test]
    fn range_ranges() {
        let r = InRange::new(signed_int::<i32>(), 0..64);

        assert_eq!(r.parse(b"42"), ParseResult::Good(42, b""));
        assert_eq!(r.parse(b"-119"), ParseResult::new_bad("Value too low"));
        assert_eq!(r.parse(b"-1"), ParseResult::new_bad("Value too low"));
        assert_eq!(r.parse(b"0"), ParseResult::Good(0, b""));
        assert_eq!(r.parse(b"64"), ParseResult::new_bad("Value too high"));
        assert_eq!(r.parse(b"65"), ParseResult::new_bad("Value too high"));

        let r = InRange::new(signed_int::<i32>(), 0..);
        assert_eq!(r.parse(b"532"), ParseResult::Good(532, b""));
        assert_eq!(r.parse(b"-1"), ParseResult::new_bad("Value too low"));

        let r = InRange::new(signed_int::<i32>(), ..=0);
        assert_eq!(r.parse(b"-117"), ParseResult::Good(-117, b""));
        assert_eq!(r.parse(b"0"), ParseResult::Good(0, b""));
    }
}