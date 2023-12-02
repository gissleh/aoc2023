use common::aoc::Day;
use common::parse;
use common::parse::{any_byte, Parser};

pub fn main(day: &mut Day, input: &[u8]) {
    let list = day.prep("Parse", || parse_list(input));

    day.note("Input games", list.len());
    day.note("Amount of cubes", list.iter().map(|g| g.cubes.len()).sum::<usize>());

    day.part("P1", || p1(&list));
    day.part("P2", || p2(&list));
}

fn p1(input: &[Game]) -> u32 {
    const LIMITS: [u8; 3] = [12, 13, 14];

    input.iter()
        .filter(|g| g.doable(LIMITS))
        .map(|g| g.id)
        .sum()
}

fn p2(input: &[Game]) -> u32 {
    input.iter()
        .map(|g| g.min_cubes())
        .sum()
}

fn parse_list(input: &[u8]) -> Vec<Game> {
    b"Game "
        .and_instead(parse::unsigned_int())
        .and_discard(b": ")
        .and(
            parse::unsigned_int::<u8>()
                .and_discard(b' ')
                .and(parse::word().or(parse::everything()).map(|w| match w[0] {
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

impl Game {
    fn doable(&self, limits: [u8; 3]) -> bool {
        let mut current = limits;

        for (amount, color, last) in self.cubes.iter().copied() {
            if current[color as usize] < amount {
                return false;
            }

            if last {
                current = limits;
            } else {
                current[color as usize] -= amount;
            }
        }

        true
    }

    fn min_cubes(&self) -> u32 {
        let mut highest = [0, 0, 0];
        let mut current = [0, 0, 0];

        for (amount, color, last) in self.cubes.iter().copied() {
            current[color as usize] += amount;
            if current[color as usize] > highest[color as usize] {
                highest[color as usize] = current[color as usize];
            }

            if last {
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