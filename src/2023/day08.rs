use common::aoc::Day;
use common::parse;
use common::parse::Parser;
use num::Integer;

const AAA: u16 = to_name_num(*b"AAA");
const ZZZ: u16 = to_name_num(*b"ZZZ");

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || Map::parser().parse(input).unwrap());

    day.note("Instructions", input.instructions.len());
    day.note("Paths", input.paths.len());
    day.note(
        "A-Paths",
        input
            .paths
            .iter()
            .filter(|Path(name, _, _)| *name % 26 == 0)
            .count(),
    );
    day.note(
        "Z-Paths",
        input
            .paths
            .iter()
            .filter(|Path(name, _, _)| *name % 26 == 25)
            .count(),
    );

    day.part("Part 1", || input.follow());
    day.part("Part 2", || input.follow_ghostily());
}

struct Map {
    instructions: Vec<bool>,
    paths: Vec<Path>,
}

impl Map {
    fn follow_ghostily(&self) -> u64 {
        let mut fac = 1u64;

        for (i, Path(name, _, _)) in self.paths.iter().enumerate() {
            if *name % 26 != 0 {
                continue;
            }

            let c = self.follow_from(i, |n| n % 26 == 25);
            fac = fac.lcm(&c);
        }

        fac
    }

    fn follow_from<F: Fn(u16) -> bool>(&self, start: usize, f: F) -> u64 {
        let mut curr = start;
        let mut steps = 0;

        for go_right in self.instructions.iter().cycle() {
            let Path(name, left, right) = self.paths[curr];
            if f(name) {
                break;
            }

            curr = if *go_right { right } else { left };
            steps += 1;
        }

        steps
    }

    fn follow(&self) -> u64 {
        let curr = self
            .paths
            .iter()
            .position(|Path(c, _, _)| *c == AAA)
            .unwrap();
        self.follow_from(curr, |n| n == ZZZ)
    }

    fn parser<'i>() -> impl Parser<'i, Self> {
        parse::choice((b'L'.map_to(false), b'R'.map_to(true)))
            .repeat()
            .and_discard(b"\n\n")
            .and(
                parse::n_bytes::<3>()
                    .map(to_name_num)
                    .and_discard(b" = (")
                    .and(parse::n_bytes::<3>().map(to_name_num))
                    .and_discard(b", ")
                    .and(parse::n_bytes::<3>().map(to_name_num))
                    .and_discard(b")\n")
                    .map(|((a, b), c)| (a, b, c))
                    .repeat(),
            )
            .map(
                |(instructions, paths): (Vec<bool>, Vec<(u16, u16, u16)>)| Self {
                    instructions,
                    paths: paths
                        .iter()
                        .map(|(c, l, r)| {
                            Path(
                                *c,
                                paths.iter().position(|(o, _, _)| *l == *o).unwrap(),
                                paths.iter().position(|(o, _, _)| *r == *o).unwrap(),
                            )
                        })
                        .collect(),
                },
            )
    }
}

struct Path(u16, usize, usize);

const fn to_name_num(n: [u8; 3]) -> u16 {
    (n[0] - b'A') as u16 * 676 + (n[1] - b'A') as u16 * 26 + (n[2] - b'A') as u16
}

#[cfg(test)]
mod tests {
    use super::*;
}
