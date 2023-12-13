use arrayvec::ArrayVec;
use common::aoc::Day;
use common::parse;
use common::parse::Parser;

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || Sequence::list_parser().parse(input).unwrap());

    day.note("Input count", input.len());
    day.note(
        "Input max length",
        input.iter().map(|s| s.numbers.len()).max().unwrap(),
    );

    day.part("Part 1", || p1(&input));
    day.part("Part 2", || p2(&input));
}

fn p1(sequences: &[Sequence]) -> i64 {
    let mut buf: Vec<Sequence> = Vec::with_capacity(16);
    let mut sum = 0;

    for seq in sequences.iter() {
        sum += seq.predict_next(&mut buf);
    }

    sum
}

fn p2(sequences: &[Sequence]) -> i64 {
    let mut buf: Vec<Sequence> = Vec::with_capacity(16);
    let mut sum = 0;

    for seq in sequences.iter() {
        sum += seq.reverse().predict_next(&mut buf);
    }

    sum
}

#[derive(Clone, Debug)]
struct Sequence {
    numbers: ArrayVec<i64, 32>,
}

impl Sequence {
    fn predict_next(&self, buf: &mut Vec<Sequence>) -> i64 {
        buf.clear();
        buf.push(self.clone());

        loop {
            let (next, nonzero) = buf.last().unwrap().subseq();
            buf.push(next);

            if !nonzero {
                break;
            }
        }

        for i in (0..buf.len() - 1).rev() {
            let last = *buf[i].numbers.last().unwrap();
            let inc = *buf[i + 1].numbers.last().unwrap();
            buf[i].numbers.push(last + inc)
        }

        *buf[0].numbers.last().unwrap()
    }

    fn reverse(&self) -> Sequence {
        let mut numbers = self.numbers.clone();
        numbers.reverse();

        Sequence { numbers }
    }

    fn subseq(&self) -> (Sequence, bool) {
        let mut numbers = ArrayVec::new();
        let mut nonzero = false;
        for i in 1..self.numbers.len() {
            let v = self.numbers[i] - self.numbers[i - 1];
            if v != 0 {
                nonzero = true
            }

            numbers.push(v);
        }

        (Sequence { numbers }, nonzero)
    }

    fn list_parser<'i>() -> impl Parser<'i, Vec<Sequence>> {
        Self::parser().repeat()
    }

    fn parser<'i>() -> impl Parser<'i, Sequence> {
        parse::signed_int()
            .delimited_by(b' ')
            .repeat()
            .then_skip(b'\n')
            .map(|numbers| Sequence { numbers })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predict_next() {
        let s1 = Sequence::parser().parse(b"0 3 6 9 12 15").unwrap();
        let s2 = Sequence::parser().parse(b"1 3 6 10 15 21").unwrap();
        let s3 = Sequence::parser().parse(b"10 13 16 21 30 45").unwrap();

        let mut buf = Vec::new();

        assert_eq!(s1.predict_next(&mut buf), 18);
        assert_eq!(s2.predict_next(&mut buf), 28);
        assert_eq!(s3.predict_next(&mut buf), 68);
        assert_eq!(p1(&[s1, s2, s3]), 114);
    }
}
