use super::{ParseResult, Parser};

#[derive(Copy, Clone)]
struct Everything;

impl<'i> Parser<'i, &'i [u8]> for Everything {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, &'i [u8]> {
        let len = input.len();
        if len > 0 {
            ParseResult::Good(input, &input[len..])
        } else {
            ParseResult::new_bad("Nothing is the only thing that does not match Everything")
        }
    }
}

#[derive(Copy, Clone)]
struct Anything;

impl<'i> Parser<'i, &'i [u8]> for Anything {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, &'i [u8]> {
        let len = input.len();
        ParseResult::Good(input, &input[len..])
    }
}

#[derive(Copy, Clone)]
struct AnyByte;

impl<'i> Parser<'i, u8> for AnyByte {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, u8> {
        if input.len() >= 1 {
            ParseResult::Good(input[0], &input[1..])
        } else {
            ParseResult::new_bad("Empty input")
        }
    }
}

#[derive(Copy, Clone)]
struct NBytes<const N: usize>;

impl<'i, const N: usize> Parser<'i, [u8; N]> for NBytes<N> {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, [u8; N]> {
        if input.len() >= N {
            let mut res = [0u8; N];
            res.copy_from_slice(&input[..N]);

            ParseResult::Good(res, &input[N..])
        } else {
            ParseResult::new_bad("Empty input")
        }
    }
}

#[derive(Copy, Clone)]
struct BytesUntil(u8, bool);

impl<'i> Parser<'i, &'i [u8]> for BytesUntil {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, &'i [u8]> {
        match input.iter().position(|v| *v == self.0) {
            Some(pos) => ParseResult::Good(&input[..pos], &input[pos + (self.1 as usize)..]),
            None => ParseResult::new_bad("Byte not found"),
        }
    }
}

#[derive(Copy, Clone)]
struct BytesUntilEither<const N: usize>([u8; N], bool);

impl<'i, const N: usize> Parser<'i, &'i [u8]> for BytesUntilEither<N> {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, &'i [u8]> {
        match input.iter().position(|v| self.0.contains(v)) {
            Some(pos) => ParseResult::Good(&input[..pos], &input[pos + (self.1 as usize)..]),
            None => ParseResult::new_bad("Either byte not found"),
        }
    }
}

#[derive(Copy, Clone)]
struct Word;

impl<'i> Parser<'i, &'i [u8]> for Word {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, &'i [u8]> {
        let len = input
            .iter()
            .take_while(|c| match **c {
                b'a'..=b'z' | b'A'..=b'Z' => true,
                _ => false,
            })
            .count();

        if len > 0 {
            ParseResult::Good(&input[..len], &input[len..])
        } else {
            ParseResult::new_bad("not a word")
        }
    }
}

#[inline]
pub fn everything<'i>() -> impl Parser<'i, &'i [u8]> {
    Everything
}

#[inline]
pub fn anything<'i>() -> impl Parser<'i, &'i [u8]> {
    Anything
}

#[inline]
pub fn any_byte<'i>() -> impl Parser<'i, u8> {
    AnyByte
}

#[inline]
pub fn n_bytes<'i, const N: usize>() -> impl Parser<'i, [u8; N]> {
    NBytes::<N>
}

#[inline]
pub fn bytes_until<'i>(b: u8, eat: bool) -> impl Parser<'i, &'i [u8]> {
    BytesUntil(b, eat)
}

#[inline]
pub fn word<'i>() -> impl Parser<'i, &'i [u8]> {
    Word
}

#[inline]
pub fn line<'i>() -> impl Parser<'i, &'i [u8]> {
    BytesUntil(b'\n', true)
}
