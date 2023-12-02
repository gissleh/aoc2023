use common::aoc::Day;
use common::parse;
use common::parse::{any_byte, Parser};

pub fn main(day: &mut Day, input: &[u8]) {
    let list = day.prep("Parse", || parse_list(input));

    day.part("P1", || p1(&list));

    day.note("Input games", list.len());
    day.note("Amount of cubes", list.iter().map(|g| g.cubes.len()).sum::<usize>());
}

fn p1(input: &[Game]) -> u32 {
    const LIMITS: [u8; 3] = [12, 13, 14];

    let cubes = LIMITS;
}

fn parse_list(input: &[u8]) -> Vec<Game> {
    b"Game "
        .and_instead(parse::unsigned_int())
        .and_discard(b": ")
        .and(
            parse::unsigned_int::<u8>()
                .and_discard(b' ')
                .and(parse::word_only().or(parse::everything()).map(|w| match w[0] {
                    b'r' => 0u8,
                    b'g' => 1u8,
                    b'b' => 2u8,
                    _ => panic!("Unknown word {}", String::from_utf8_lossy(w))
                }))
                .and(any_byte().map(|d| d != b',').or_return(true))
                .map(|((amount, color), semi)| (amount, color, semi))
                .then_skip(b' ')
                .repeat()
                .capped_by(b'\n')
        )
        .map(|(id, cubes)| Game{ id, cubes })
        .repeat()
        .parse(input)
        .unwrap()
}

#[derive(Debug)]
struct Game {
    id: u32,
    cubes: Vec<(u8, u8, bool)>
}

#[cfg(test)]
mod tests {
    use super::*;
}