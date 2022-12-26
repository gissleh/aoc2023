use crate::parse::repeat::Repeat;

pub mod repeat;

pub trait Parser<'i, T>: Sized + Copy {
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, T>;

    fn repeat<R>(self) -> Repeat<Self, T, R> {
        Repeat::new(self)
    }

    fn repeat_n<R>(self, amount: usize) -> Repeat<Self, T, R> {
        Repeat::with_amount(self, amount)
    }
}

pub struct ParseError<'i> {
    reason: &'static str,
    sub_reason: Option<&'static str>,
    input: &'i [u8],
}

impl<'i> ParseError<'i> {
    fn new(reason: &'static str, input: &'i [u8]) -> Self {
        Self { reason, input, sub_reason: None }
    }

    fn wrap(&self, reason: &'static str) -> Self {
        Self { reason, input: self.input, sub_reason: Some(self.reason) }
    }
}

pub enum ParseResult<'i, T> {
    Good(T, &'i [u8]),
    Bad(ParseError<'i>),
}

impl<'i, T> ParseResult<'i, T> {
    pub fn unwrap(self) -> T {
        match self {
            ParseResult::Good(v, _) => v,
            ParseResult::Bad(err) => panic!("Unwrap on failed parse: {}", err.reason)
        }
    }
}

impl<'i> Parser<'i, u8> for u8 {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, u8> {
        if input.first().copied() == Some(*self) {
            ParseResult::Good(*self, &input[1..])
        } else {
            ParseResult::Bad(ParseError::new("u8 not matched", input))
        }
    }
}
