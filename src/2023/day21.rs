use common::aoc::Day;
use common::geo::Point;
use common::grid::Grid;
use common::parse;
use common::parse::Parser;
use rustc_hash::FxHashSet;

const WALL: u8 = b'#';
const START: u8 = b'S';

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || parse(input));

    day.note("Input Width", input.width());
    day.note("Input Height", input.height());

    day.part("Part 1", || p1(&input, 64));
    day.part("Part 2", || p2(&input));
}

fn p1(input: &Grid<u8, Vec<u8>>, target: u8) -> u32 {
    let mut seen: Grid<u128, Vec<u128>> = Grid::new_with_value(input.width(), input.height(), 0);
    let mut stack = Vec::with_capacity(256);
    let mut count = 0;

    stack.push((input.find(&START).unwrap(), 0));

    while let Some((pos, steps)) = stack.pop() {
        let seen = &mut seen[pos];
        let mask = 1 << steps;
        if *seen & mask != 0 {
            continue;
        }
        *seen |= mask;

        if steps == target {
            count += 1;
        } else {
            for adjacent in pos.cardinals_offset_underflow(1) {
                let [x, y] = adjacent.coords();
                if *x == usize::MAX || *y == usize::MAX {
                    continue;
                }

                if input[adjacent] != WALL {
                    stack.push((adjacent, steps + 1))
                }
            }
        }
    }

    count
}

fn p2(input: &Grid<u8, Vec<u8>>) -> u64 {
    let mut seen: FxHashSet<(Point<usize>, u32)> = FxHashSet::default();
    let mut count = (0, 0, 0);

    let w = input.width() as u32;
    let targets = (w / 2, (w / 2) + w, (w / 2) + (w * 2));
    let size = Point::new(input.width(), input.height());
    let start = input.find(&START).unwrap() + Point::new(input.width() * 4, input.height() * 4);

    let mut stack = Vec::with_capacity(4096);
    stack.push((start, 0));

    while let Some((pos, steps)) = stack.pop() {
        if seen.contains(&(pos, steps)) {
            continue;
        }

        if steps == targets.2 {
            count.2 += 1;
        } else {
            if steps == targets.0 {
                count.0 += 1;
            }
            if steps == targets.1 {
                count.1 += 1;
            }

            for adjacent in pos.cardinals() {
                let lookup = adjacent % size;

                if input[lookup] != WALL {
                    stack.push((adjacent, steps + 1))
                }
            }
        }

        seen.insert((pos, steps));
    }

    let n = 26501365 / input.width() as u64;
    let x = count.0;
    let y = count.1 - count.0;
    let z = count.2 - count.1;

    x + (y * n) + (((n * (n - 1)) / 2) * (z - y))
}

fn parse(input: &[u8]) -> Grid<u8, Vec<u8>> {
    Grid::parser(parse::any_byte()).parse(input).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_EXAMPLE: &[u8] = b"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[test]
    fn p1_works_on_example() {
        assert_eq!(p1(&parse(P1_EXAMPLE), 6), 16);
    }
}
