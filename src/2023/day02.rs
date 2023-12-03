use common::aoc::Day;
use common::parse;
use common::parse::{any_byte, choice, Parser};

pub fn main(day: &mut Day, input: &[u8]) {
    let list = day.prep("Parse", || parse_games(input));

    day.note("Amount of games", list.len());
    day.note(
        "Amount of ops",
        list.iter().map(|g| g.cubes.len()).sum::<usize>(),
    );

    day.part("Part 1", || p1(&list));
    day.part("Part 2", || p2(&list));
}

fn p1(input: &[Game]) -> u32 {
    const LIMITS: [u8; 3] = [12, 13, 14];

    input
        .iter()
        .filter(|g| g.doable(LIMITS))
        .map(|g| g.id)
        .sum()
}

fn p2(input: &[Game]) -> u32 {
    input.iter().map(|g| g.min_cubes()).sum()
}

fn parse_games(input: &[u8]) -> Vec<Game> {
    b"Game "
        .and_instead(parse::unsigned_int())
        .and_discard(b": ")
        .and(
            parse::unsigned_int::<u8>()
                .and_discard(b' ')
                .and(choice((
                    b"red".map_to(0),
                    b"green".map_to(1),
                    b"blue".map_to(2),
                )))
                .and(any_byte().map(|d| d != b',').or_return(true))
                .map(|((amount, color), end_of_round)| (amount, color, end_of_round))
                .then_skip(b' ')
                .repeat()
                .capped_by(b'\n'),
        )
        .map(|(id, cubes)| Game { id, cubes })
        .repeat()
        .parse(input)
        .unwrap()
}

#[derive(Debug)]
struct Game {
    id: u32,
    cubes: Vec<(u8, u8, bool)>,
}

impl Game {
    fn doable(&self, limits: [u8; 3]) -> bool {
        let mut current = limits;

        for (amount, color, end_of_round) in self.cubes.iter().copied() {
            let ci = color as usize;
            if current[ci] < amount {
                return false;
            }

            if end_of_round {
                current = limits;
            } else {
                current[ci] -= amount;
            }
        }

        true
    }

    fn min_cubes(&self) -> u32 {
        let mut highest = [0, 0, 0];
        let mut current = [0, 0, 0];

        for (amount, color, end_of_round) in self.cubes.iter().copied() {
            let ci = color as usize;

            current[ci] += amount;
            if current[ci] > highest[ci] {
                highest[ci] = current[ci];
            }

            if end_of_round {
                current = [0, 0, 0];
            }
        }

        (highest[0] as u32) * (highest[1] as u32) * (highest[2] as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
