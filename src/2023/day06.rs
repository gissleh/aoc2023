use arrayvec::ArrayVec;
use common::aoc::Day;
use common::parse;
use common::parse::{Parser};
use common::search::{find_first_number};

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || Races::parser().parse(input).unwrap());

    day.note("Input distances", input.distances.len());
    day.note("Input times", input.times.len());

    day.part("Part 1", || input.ways_factor());
    day.part("Part 2", || input.big_race());
}

struct Races {
    times: ArrayVec<u64, 8>,
    distances: ArrayVec<u64, 8>
}

impl Races {
    fn ways_factor(&self) -> u64 {
        self.times.iter()
            .zip(self.distances.iter())
            .map(|(time, distance)| {
                (1..*time)
                    .filter(|hold_time| *hold_time * (*time - *hold_time) > *distance)
                    .count() as u64
            })
            .product::<u64>()
    }

    fn big_race(&self) -> u64 {
        let time = self.times.iter().skip(1).fold(self.times[0], |c, n| add_digits(c, *n));
        let distance = self.distances.iter().skip(1).fold(self.distances[0], |c, n| add_digits(c, *n));

        let initial_step = time / 8;

        let first = find_first_number(1, time, initial_step, 10, |hold_time| hold_time * (time - hold_time) > distance).unwrap();

        (time - (first * 2)) + 1
    }

    fn parser<'i>() -> impl Parser<'i, Self> {
        b"Time:"
            .and_instead(
                b' '
                    .then_skip_all(b' ')
                    .and_instead(parse::unsigned_int())
                    .repeat()
            )
            .and_discard(b"\nDistance:")
            .and(
                b' '
                    .then_skip_all(b' ')
                    .and_instead(parse::unsigned_int())
                    .repeat()
            )
            .map(|(times, distances)| Races{ times, distances })
    }
}

fn add_digits(a: u64, b: u64) -> u64 {
    match b {
        0..=9 => (a * 10) + b,
        10..=99 => (a * 100) + b,
        100..=999 => (a * 1000) + b,
        1000..=9999 => (a * 10000) + b,
        _ => panic!("number too big")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_p2() {
        let races = Races::parser().parse(EXAMPLE).unwrap();
        assert_eq!(races.big_race(), 71503);
    }
}