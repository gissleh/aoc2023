use common::aoc::Day;
use common::parse;
use common::parse::Parser;
use std::cmp::min;

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || parse(input));

    day.note("Patterns", input.len());
    day.note("Max Width", input.iter().map(|p| p.width).max().unwrap());
    day.note("Max Height", input.iter().map(|p| p.height).max().unwrap());

    day.part("Part 1", || p1(&input));
    day.part("Part 2", || p2(&input));
}

fn p1(input: &[Pattern]) -> u32 {
    let mut sum = 0;
    for pattern in input.iter() {
        if let Some(v) = pattern.reflects(false) {
            sum += v;
        }
        if let Some(v) = pattern.reflects(true) {
            sum += v * 100;
        }
    }

    sum
}

fn p2(input: &[Pattern]) -> u32 {
    let mut sum = 0;
    for pattern in input.iter() {
        if let Some(v) = pattern.reflects_smugdily(false) {
            sum += v;
        }
        if let Some(v) = pattern.reflects_smugdily(true) {
            sum += v * 100;
        }
    }

    sum
}


fn parse(input: &[u8]) -> Vec<Pattern> {
    Pattern::parser().repeat().parse(input).unwrap()
}

struct Pattern {
    width: u32,
    height: u32,
    rows: Vec<u32>,
    cols: Vec<u32>,
}

impl Pattern {
    fn reflects(&self, row: bool) -> Option<u32> {
        let list = if row { &self.rows } else { &self.cols };
        let len = if row { self.height } else { self.width };

        for r in 0..len - 1 {
            let steps = min(r, len - r);
            let mut reflected = true;

            #[cfg(test)]
            println!("row {}", r);

            for step in 0..=steps {
                let u = r - step;
                let d = r + step + 1;
                if d >= len {
                    #[cfg(test)]
                    println!("{} {} outside", u, d);

                    break;
                }

                #[cfg(test)]
                println!("{} {} {}", u, d, list[u as usize] == list[d as usize]);

                if list[u as usize] != list[d as usize] {
                    reflected = false;
                    break;
                }
            }

            if reflected {
                return Some(r + 1);
            }
        }

        None
    }

    fn reflects_smugdily(&self, row: bool) -> Option<u32> {
        let list = if row { &self.rows } else { &self.cols };
        let len = if row { self.height } else { self.width };

        for r in 0..len - 1 {
            let steps = min(r, len - r);
            let mut reflected = true;
            let mut smudge_found = false;

            for step in 0..=steps {
                let u = r - step;
                let d = r + step + 1;
                if d >= len {

                    break;
                }

                if list[u as usize] != list[d as usize] {
                    if !smudge_found {
                        if (list[u as usize] ^ list[d as usize]).count_ones() == 1 {
                            smudge_found = true;
                        } else {
                            reflected = false;
                            break;
                        }
                    } else {
                        reflected = false;
                        break;
                    }
                }
            }

            if reflected && smudge_found {
                return Some(r + 1);
            }
        }

        None
    }

    fn parser<'i>() -> impl Parser<'i, Pattern> {
        parse::line()
            .map(|l| l.len() as u32)
            .rewind()
            .and(
                parse::line()
                    .only_if(|l| l.len() > 0)
                    .map(|l| {
                        l.iter()
                            .enumerate()
                            .map(|(i, v)| ((*v == b'#') as u32) << i)
                            .sum()
                    })
                    .repeat::<Vec<_>>(),
            )
            .map(|(width, rows)| {
                let height = rows.len() as u32;
                let mut cols = vec![0u32; width as usize];

                for (i, row) in rows.iter().copied().enumerate() {
                    for c in 0..width as usize {
                        cols[c] |= if row & 1 << c != 0 { 1 << i } else { 0 }
                    }
                }

                Self {
                    rows,
                    cols,
                    width,
                    height,
                }
            })
            .then_skip(b'\n')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reflect_works() {
        let examples = parse(P1_EXAMPLES_1);
        let examples2 = parse(P1_EXAMPLE_2);

        assert_eq!(examples[0].reflects(true), None);
        assert_eq!(examples[0].reflects(false), Some(5));
        assert_eq!(examples[1].reflects(true), Some(4));
        assert_eq!(examples[1].reflects(false), None);
        assert_eq!(examples2[0].reflects(true), Some(12));
        assert_eq!(examples2[0].reflects(false), None);
        assert_eq!(examples2[1].reflects(true), None);
        assert_eq!(examples2[1].reflects(false), Some(1));
    }

    #[test]
    fn p1_works_on_example() {
        let examples = parse(P1_EXAMPLES_1);
        assert_eq!(p1(&examples), 405);
    }

    const P1_EXAMPLES_1: &[u8] = b"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    const P1_EXAMPLE_2: &[u8] = b"..#####
..#....
..#####
##...#.
.#.....
.####.#
.####.#
##.....
##...#.
..#####
..#....
..#####
..#####

....#..
###....
...#.##
###....
....##.
##.#.##
..##.##
###.#..
..#.###
";
}
