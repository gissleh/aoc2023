use common::aoc::Day;
use common::parse;
use common::parse::Parser;

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || Scratchcard::parse_list(input));

    day.note("Scratch Cards", input.len());

    day.part("Part 1", || p1(&input));
    day.part("Part 2", || p2(&input));
}

fn p1(input: &[Scratchcard]) -> u32 {
    input.iter().map(|c| c.points()).sum()
}

fn p2(input: &[Scratchcard]) -> u32 {
    let mut multiplier = vec![1; input.len()];
    let mut count = 0;

    for i in 0..input.len() {
        count += multiplier[i];
        let winning_numbers = input[i].winning_numbers() as usize;

        for n in 1..=winning_numbers {
            multiplier[i + n] += multiplier[i];
        }
    }

    count
}

#[derive(Debug, Clone)]
struct Scratchcard {
    winners: u128,
    numbers: u128,
}

impl Scratchcard {
    #[inline]
    fn winning_numbers(&self) -> u32 {
        (self.numbers & self.winners).count_ones()
    }

    #[inline]
    fn points(&self) -> u32 {
        let count = (self.numbers & self.winners).count_ones();
        if count > 0 {
            1 << (count - 1)
        } else {
            0
        }
    }

    fn parser<'i>() -> impl Parser<'i, Self> {
        b"Card "
            .then_skip_all(b' ')
            .then_skip_all(parse::digit::<u8>())
            .and_discard(b':')
            .and_instead(
                b' '
                    .then_skip(b' ')
                    .and_instead(parse::unsigned_int::<u32>())
                    .repeat_fold(|| 0u128, |acc, curr| acc | 1 << curr)
                    .capped_by(b" |")
            )
            .and(
                b' '
                    .then_skip(b' ')
                    .and_instead(parse::unsigned_int::<u32>())
                    .repeat_fold(|| 0u128, |acc, curr| acc | 1 << curr)
            )
            .map(|(w, n)| Self{
                numbers: n,
                winners: w,
            })
            .then_skip(b'\n')
    }

    fn parse_list(input: &[u8]) -> Vec<Scratchcard> {
        Self::parser().repeat().parse(input).unwrap()
    }
}
