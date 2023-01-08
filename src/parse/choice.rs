use crate::parse::{ParseError, Parser, ParseResult};

pub trait Choices<'i, T>: Copy + Clone {
    fn parse_choice(&self, input: &'i [u8]) -> ParseResult<'i, T>;
}

impl<'i, const N: usize, T, P: Parser<'i, T>> Choices<'i, T> for [P; N] {
    #[inline]
    fn parse_choice(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        for i in 0..N {
            if let ParseResult::Good(t, input) = self[i].parse(input) {
                return ParseResult::Good(t, input);
            }
        }

        ParseResult::Bad(ParseError::new("No choice matched", input))
    }
}

// Things to learn: Code generation

impl<'i, T, P1: Parser<'i, T>, P2: Parser<'i, T>> Choices<'i, T> for (P1, P2) {
    #[inline]
    fn parse_choice(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        if let ParseResult::Good(v, input) = self.0.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.1.parse(input) {
            ParseResult::Good(v, input)
        } else {
            ParseResult::Bad(ParseError::new("No choice matched", input))
        }
    }
}

impl<'i, T, P1: Parser<'i, T>, P2: Parser<'i, T>, P3: Parser<'i, T>> Choices<'i, T> for (P1, P2, P3) {
    #[inline]
    fn parse_choice(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        if let ParseResult::Good(v, input) = self.0.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.1.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.2.parse(input) {
            ParseResult::Good(v, input)
        } else {
            ParseResult::Bad(ParseError::new("No choice matched", input))
        }
    }
}

impl<'i, T, P1: Parser<'i, T>, P2: Parser<'i, T>, P3: Parser<'i, T>, P4: Parser<'i, T>> Choices<'i, T> for (P1, P2, P3, P4) {
    #[inline]
    fn parse_choice(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        if let ParseResult::Good(v, input) = self.0.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.1.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.2.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.3.parse(input) {
            ParseResult::Good(v, input)
        } else {
            ParseResult::Bad(ParseError::new("No choice matched", input))
        }
    }
}

impl<'i, T, P1: Parser<'i, T>, P2: Parser<'i, T>, P3: Parser<'i, T>, P4: Parser<'i, T>, P5: Parser<'i, T>> Choices<'i, T> for (P1, P2, P3, P4, P5) {
    #[inline]
    fn parse_choice(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        if let ParseResult::Good(v, input) = self.0.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.1.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.2.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.3.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.4.parse(input) {
            ParseResult::Good(v, input)
        } else {
            ParseResult::Bad(ParseError::new("No choice matched", input))
        }
    }
}

impl<'i, T, P1: Parser<'i, T>, P2: Parser<'i, T>, P3: Parser<'i, T>, P4: Parser<'i, T>, P5: Parser<'i, T>, P6: Parser<'i, T>> Choices<'i, T> for (P1, P2, P3, P4, P5, P6) {
    #[inline]
    fn parse_choice(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        if let ParseResult::Good(v, input) = self.0.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.1.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.2.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.3.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.4.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.5.parse(input) {
            ParseResult::Good(v, input)
        } else {
            ParseResult::Bad(ParseError::new("No choice matched", input))
        }
    }
}

impl<'i, T, P1: Parser<'i, T>, P2: Parser<'i, T>, P3: Parser<'i, T>, P4: Parser<'i, T>, P5: Parser<'i, T>, P6: Parser<'i, T>, P7: Parser<'i, T>> Choices<'i, T> for (P1, P2, P3, P4, P5, P6, P7) {
    #[inline]
    fn parse_choice(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        if let ParseResult::Good(v, input) = self.0.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.1.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.2.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.3.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.4.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.5.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.6.parse(input) {
            ParseResult::Good(v, input)
        } else {
            ParseResult::Bad(ParseError::new("No choice matched", input))
        }
    }
}

impl<'i, T, P1: Parser<'i, T>, P2: Parser<'i, T>, P3: Parser<'i, T>, P4: Parser<'i, T>, P5: Parser<'i, T>, P6: Parser<'i, T>, P7: Parser<'i, T>, P8: Parser<'i, T>> Choices<'i, T> for (P1, P2, P3, P4, P5, P6, P7, P8) {
    #[inline]
    fn parse_choice(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        if let ParseResult::Good(v, input) = self.0.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.1.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.2.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.3.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.4.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.5.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.6.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.7.parse(input) {
            ParseResult::Good(v, input)
        } else {
            ParseResult::Bad(ParseError::new("No choice matched", input))
        }
    }
}

struct Choice<C> (C);

impl<C> Copy for Choice<C> where C: Copy {}

impl<C> Clone for Choice<C> where C: Clone {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'i, C, T> Parser<'i, T> for Choice<C> where C: Choices<'i, T> {
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        self.0.parse_choice(input)
    }
}

pub fn choice<'i, T, C: Choices<'i, T>>(choices: C) -> impl Parser<'i, T> {
    Choice(choices)
}
