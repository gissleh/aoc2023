use crate::parse::{ParseResult, Parser};

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

        ParseResult::new_bad("No choices matched")
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
            ParseResult::new_bad("No choices matched")
        }
    }
}

impl<'i, T, P1: Parser<'i, T>, P2: Parser<'i, T>, P3: Parser<'i, T>> Choices<'i, T>
    for (P1, P2, P3)
{
    #[inline]
    fn parse_choice(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        if let ParseResult::Good(v, input) = self.0.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.1.parse(input) {
            ParseResult::Good(v, input)
        } else if let ParseResult::Good(v, input) = self.2.parse(input) {
            ParseResult::Good(v, input)
        } else {
            ParseResult::new_bad("No choices matched")
        }
    }
}

impl<'i, T, P1: Parser<'i, T>, P2: Parser<'i, T>, P3: Parser<'i, T>, P4: Parser<'i, T>>
    Choices<'i, T> for (P1, P2, P3, P4)
{
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
            ParseResult::new_bad("No choices matched")
        }
    }
}

impl<
        'i,
        T,
        P1: Parser<'i, T>,
        P2: Parser<'i, T>,
        P3: Parser<'i, T>,
        P4: Parser<'i, T>,
        P5: Parser<'i, T>,
    > Choices<'i, T> for (P1, P2, P3, P4, P5)
{
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
            ParseResult::new_bad("No choices matched")
        }
    }
}

impl<
        'i,
        T,
        P1: Parser<'i, T>,
        P2: Parser<'i, T>,
        P3: Parser<'i, T>,
        P4: Parser<'i, T>,
        P5: Parser<'i, T>,
        P6: Parser<'i, T>,
    > Choices<'i, T> for (P1, P2, P3, P4, P5, P6)
{
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
            ParseResult::new_bad("No choices matched")
        }
    }
}

impl<
        'i,
        T,
        P1: Parser<'i, T>,
        P2: Parser<'i, T>,
        P3: Parser<'i, T>,
        P4: Parser<'i, T>,
        P5: Parser<'i, T>,
        P6: Parser<'i, T>,
        P7: Parser<'i, T>,
    > Choices<'i, T> for (P1, P2, P3, P4, P5, P6, P7)
{
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
            ParseResult::new_bad("No choices matched")
        }
    }
}

impl<
        'i,
        T,
        P1: Parser<'i, T>,
        P2: Parser<'i, T>,
        P3: Parser<'i, T>,
        P4: Parser<'i, T>,
        P5: Parser<'i, T>,
        P6: Parser<'i, T>,
        P7: Parser<'i, T>,
        P8: Parser<'i, T>,
    > Choices<'i, T> for (P1, P2, P3, P4, P5, P6, P7, P8)
{
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
            ParseResult::new_bad("No choices matched")
        }
    }
}

impl<
        'i,
        T,
        P1: Parser<'i, T>,
        P2: Parser<'i, T>,
        P3: Parser<'i, T>,
        P4: Parser<'i, T>,
        P5: Parser<'i, T>,
        P6: Parser<'i, T>,
        P7: Parser<'i, T>,
        P8: Parser<'i, T>,
        P9: Parser<'i, T>,
    > Choices<'i, T> for (P1, P2, P3, P4, P5, P6, P7, P8, P9)
{
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
        } else if let ParseResult::Good(v, input) = self.8.parse(input) {
            ParseResult::Good(v, input)
        } else {
            ParseResult::new_bad("No choices matched")
        }
    }
}

struct Choice<C>(C);

impl<C> Copy for Choice<C> where C: Copy {}

impl<C> Clone for Choice<C>
where
    C: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'i, C, T> Parser<'i, T> for Choice<C>
where
    C: Choices<'i, T>,
{
    fn parse(&self, input: &'i [u8]) -> ParseResult<'i, T> {
        self.0.parse_choice(input)
    }
}

pub fn choice<'i, T, C: Choices<'i, T>>(choices: C) -> impl Parser<'i, T> {
    Choice(choices)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{everything, unsigned_int};

    #[test]
    fn choice_choices() {
        #[derive(Clone, Eq, PartialEq, Debug)]
        enum Instruction<'i> {
            Set(u8, i64),
            Select(u8),
            Add(i64),
            Sub(i64),
            Print(u8),
            PrintText(&'i [u8]),
        }

        let sep = b','.then_skip_all(b' ');
        let function = choice((
            b"set".and_instead(
                unsigned_int()
                    .and_discard(sep)
                    .and(unsigned_int())
                    .quoted_by(b'(', b')')
                    .map(|(a, v)| Instruction::Set(a, v)),
            ),
            b"select".and_instead(
                unsigned_int()
                    .quoted_by(b'(', b')')
                    .map(|a| Instruction::Select(a)),
            ),
            b"add".and_instead(
                unsigned_int()
                    .quoted_by(b'(', b')')
                    .map(|v| Instruction::Add(v)),
            ),
            b"sub".and_instead(
                unsigned_int()
                    .quoted_by(b'(', b')')
                    .map(|v| Instruction::Sub(v)),
            ),
            b"print".and_instead(
                unsigned_int()
                    .quoted_by(b'(', b')')
                    .map(|a| Instruction::Print(a)),
            ),
            b"print_text".and_instead(
                everything()
                    .quoted_by(b'"', b'"')
                    .quoted_by(b'(', b')')
                    .map(|text| Instruction::PrintText(text)),
            ),
        ));
        let parser = function
            .delimited_by(b';'.then_skip_all(b' ').then_skip_all(b'\n'))
            .repeat::<Vec<Instruction>>();

        assert_eq!(
            parser.parse(b"set(0, 64); set(1, 32); select(0); add(43); select(1); sub(39); print_text(\"a is: \"); print(0)"),
            ParseResult::Good(
                vec![
                    Instruction::Set(0, 64),
                    Instruction::Set(1, 32),
                    Instruction::Select(0),
                    Instruction::Add(43),
                    Instruction::Select(1),
                    Instruction::Sub(39),
                    Instruction::PrintText(b"a is: "),
                    Instruction::Print(0),
                ],
                b""
            )
        )
    }
}
