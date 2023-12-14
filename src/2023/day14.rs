use rustc_hash::FxHashMap;
use common::aoc::Day;
use common::grid::Grid;

const WALL: u8 = b'#';
const FLOOR: u8 = b'.';
const STONE: u8 = b'O';

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || parse(input));

    day.note("Input Width", input.width() - 2);
    day.note("Input Height", input.height() - 2);

    day.part("Part 1", || p1(&input));
    day.part("Part 2", || p2(&input));
}

fn p1(grid: &Grid<u8, Vec<u8>>) -> usize {
    let mut grid = grid.clone();

    tilt(&mut grid, |x, y| (x, y - 1));
    load(&grid)
}

fn p2(grid: &Grid<u8, Vec<u8>>) -> usize {
    let mut grid = grid.clone();
    let mut seen = FxHashMap::default();

    let mut loads = Vec::with_capacity(256);
    let mut cycle_len = 0;
    let mut cycle_start = 0;

    for n in 0..1000000000 {
        tilt(&mut grid, |x, y| (x, y - 1));
        tilt(&mut grid, |x, y| (x - 1, y));
        tilt_rev(&mut grid, |x, y| (x, y + 1));
        tilt_rev(&mut grid, |x, y| (x + 1, y));

        loads.push(load(&grid));

        if let Some(n2) = seen.insert(grid.clone(), n) {
            cycle_len = n - n2;
            cycle_start = n2;
            break;
        }
    }

    let pos_in_cycle = (1000000000 - cycle_start) % cycle_len;
    loads[cycle_start + pos_in_cycle - 1]
}

fn load(grid: &Grid<u8, Vec<u8>>) -> usize {
    let mut sum = 0;

    for x in 1..grid.width() - 1 {
        for y in 1..grid.height() - 1 {
            if grid[(x, y)] == STONE {
                sum += (grid.height() - y) - 1;
            }
        }
    }

    sum
}

fn tilt<M: Fn(usize, usize) -> (usize, usize)>(grid: &mut Grid<u8, Vec<u8>>, m: M) -> u32 {
    let mut movements = 0;

    for y in 1..grid.height() - 1 {
        for x in 1..grid.width() - 1 {
            if grid[(x, y)] == STONE {
                let mut x = x;
                let mut y = y;
                let (mut nx, mut ny) = m(x, y);

                while grid[(nx, ny)] == FLOOR {
                    grid[(nx, ny)] = STONE;
                    grid[(x, y)] = FLOOR;

                    movements += 1;

                    (x, y) = (nx, ny);
                    (nx, ny) = m(x, y);
                }
            }
        }
    }

    movements
}

fn tilt_rev<M: Fn(usize, usize) -> (usize, usize)>(grid: &mut Grid<u8, Vec<u8>>, m: M) -> u32 {
    let mut movements = 0;

    for y in (1..grid.height() - 1).rev() {
        for x in (1..grid.width() - 1).rev() {
            if grid[(x, y)] == STONE {
                let mut x = x;
                let mut y = y;
                let (mut nx, mut ny) = m(x, y);

                while grid[(nx, ny)] == FLOOR {
                    grid[(nx, ny)] = STONE;
                    grid[(x, y)] = FLOOR;

                    movements += 1;

                    (x, y) = (nx, ny);
                    (nx, ny) = m(x, y);
                }
            }
        }
    }

    movements
}

fn parse(input: &[u8]) -> Grid<u8, Vec<u8>> {
    let width = input.iter().position(|v| *v == b'\n').unwrap();
    let height = input.len() / (width + 1);
    let mut grid = Grid::new_with_value(width + 2, height + 2, WALL);

    let mut x = 1;
    let mut y = 1;
    for v in input.iter() {
        if *v == b'\n' {
            y += 1;
            x = 1;
        } else {
            grid[(x, y)] = *v;
            x += 1;
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_EXAMPLE: &[u8] = b"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn p1_works_on_example() {
        assert_eq!(p1(&parse(P1_EXAMPLE)), 136);
    }

    #[test]
    fn p2_works_on_example() {
        assert_eq!(p2(&parse(P1_EXAMPLE)), 64);
    }
}