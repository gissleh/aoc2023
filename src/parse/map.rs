use std::marker::PhantomData;
use crate::parse::{Parser, ParseResult};

pub struct Map<P, F, TP, TF> {
    parser: P,
    mapper_fn: F,
    spooky_ghost: PhantomData<(TP, TF)>,
}

impl<P, F, TP, TF> Map<P, F, TP, TF> {
    pub(crate) fn new(parser: P, mapper_fn: F) -> Self {
        Self { parser, mapper_fn, spooky_ghost: PhantomData::default() }
    }
}

impl<P, F, TP, TF> Copy for Map<P, F, TP, TF> where P: Copy, F: Copy {}

impl<P, F, TP, TF> Clone for Map<P, F, TP, TF> where P: Clone, F: Clone {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            mapper_fn: self.mapper_fn.clone(),
            spooky_ghost: PhantomData::default(),
        }
    }
}

impl<'i, P, F, TP, TF> Parser<'i, TF> for Map<P, F, TP, TF>
    where P: Parser<'i, TP>,
          F: Fn(TP) -> TF + Copy {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, TF> {
        match self.parser.parse(input) {
            ParseResult::Good(vp, input) => ParseResult::Good((self.mapper_fn)(vp), input),
            ParseResult::Bad(err) => ParseResult::Bad(err),
        }
    }
}

pub struct MapValue<P, TP, TV> {
    parser: P,
    mapped_value: TV,
    spooky_ghost: PhantomData<TP>,
}

impl<P, TP, TV> Copy for MapValue<P, TP, TV> where P: Copy, TV: Copy {}

impl<P, TP, TV> Clone for MapValue<P, TP, TV> where P: Clone, TV: Clone {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            mapped_value: self.mapped_value.clone(),
            spooky_ghost: PhantomData::default(),
        }
    }
}

impl<'i, P, TP, TV> Parser<'i, TV> for MapValue<P, TP, TV> where P: Parser<'i, TP>, TV: Copy {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, TV> {
        match self.parser.parse(input) {
            ParseResult::Good(_, input) => ParseResult::Good(self.mapped_value, input),
            ParseResult::Bad(err) => ParseResult::Bad(err),
        }
    }
}

impl<P, TP, TV> MapValue<P, TP, TV> {
    pub(crate) fn new(parser: P, mapped_value: TV) -> Self {
        Self { parser, mapped_value, spooky_ghost: PhantomData::default() }
    }
}

pub struct FilterMap<P, F, TP, TF> {
    parser: P,
    mapper_fn: F,
    spooky_ghost: PhantomData<(TP, TF)>,
}

impl<P, F, TP, TF> FilterMap<P, F, TP, TF> {
    #[inline]
    pub(crate) fn new(parser: P, mapper_fn: F) -> Self {
        Self { parser, mapper_fn, spooky_ghost: PhantomData::default() }
    }
}

impl<P, F, TP, TF> Copy for FilterMap<P, F, TP, TF> where P: Copy, F: Copy {}

impl<P, F, TP, TF> Clone for FilterMap<P, F, TP, TF> where P: Clone, F: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            mapper_fn: self.mapper_fn.clone(),
            spooky_ghost: PhantomData::default(),
        }
    }
}

impl<'i, P, F, TP, TF> Parser<'i, TF> for FilterMap<P, F, TP, TF>
    where P: Parser<'i, TP>,
          F: Fn(TP) -> Option<TF> + Copy {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, TF> {
        match self.parser.parse(input) {
            ParseResult::Good(vp, new_input) => match (self.mapper_fn)(vp) {
                Some(vf) => ParseResult::Good(vf, new_input),
                None => ParseResult::new_bad("FilterMap function returned None")
            }
            ParseResult::Bad(err) => ParseResult::Bad(err),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::geo::Point;
    use crate::parse::{any_byte, digit, signed_int};
    use super::*;

    #[test]
    fn filter_map_filters_and_maps_like_a_filter_map_does() {
        let parser_1 = any_byte().filter_map(|v| match v {
            b'0'..=b'9' => Some((v - b'0') as u32),
            b'a'..=b'z' => Some((v - b'a') as u32 + 100),
            b'A'..=b'Z' => Some((v - b'A') as u32 + 200),
            b'#' => Some(999),
            _ => None,
        });

        assert_eq!(parser_1.parse(b"X"), ParseResult::Good(223, b""));
        assert_eq!(parser_1.parse(b"."), ParseResult::new_bad("FilterMap function returned None"));

        assert_eq!(
            parser_1.repeat().parse(b"123#abZ.aa"),
            ParseResult::Good(
                vec![1, 2, 3, 999, 100, 101, 225],
                b".aa"
            )
        );
    }

    #[test]
    fn map_maps_mappily_ever_after() {
        assert_eq!(
            b'-'.and_instead(digit::<u8>()).map(|v| -(v as i8)).parse(b"-4"),
            ParseResult::Good(-4, b"")
        );
        assert_eq!(
            b'a'.map_to(174).parse(b"abcde"),
            ParseResult::Good(174, b"bcde")
        );
        assert_eq!(
            b'<'.and_instead(
                signed_int::<i32>()
                    .delimited_by(b','.then_skip_all(b' '))
                    .repeat::<(_, _)>()
                    .and_discard(b'>')
                    .map(|(x, y)| Point::new(x, y)))
                .delimited_by(b','.then_skip_all(b' '))
                .repeat()
                .parse(b"<-117, 640>, <96, -32>,  <900,   800>"),
            ParseResult::Good([
                                  Point::new(-117, 640), Point::new(96, -32), Point::new(900, 800)
                              ], b""),
            "More complex mapping case for a loose point parser."
        )
    }
}