use common::aoc::Day;
use common::grid::Grid;
use common::parse;
use common::parse::Parser;
use common::search::bfs;
use common::search::Search;
use std::ops::{Add, Sub};

pub fn main(day: &mut Day, input: &[u8]) {
    let digs = day.prep("Parse", || Dig::parse_list(input));

    day.note("Dig Count", digs.len());
    day.note(
        "Dig Distance (P1)",
        digs.iter().map(|d| d.steps as u32).sum::<u32>(),
    );

    day.part("Part 1 (Flood Fill)", || p1_bfs(&digs));
    day.branch_from("Parse");
    day.part("Part 1 (Shoelace)", || p1_shoelace(&digs));
    day.part("Part 2 (Shoelace)", || p2_shoelace(&digs));
}

fn p1_bfs(input: &[Dig]) -> u32 {
    let (mut min_x, mut min_y) = (0, 0);
    let (mut max_x, mut max_y) = (0, 0);
    let (mut x, mut y) = (0, 0);
    for dig in input.iter() {
        (x, y) = dig.dir.next(x, y, dig.steps as i32);

        if x < min_x {
            min_x = x
        };
        if y < min_y {
            min_y = y
        };
        if x > max_x {
            max_x = x
        };
        if y > max_y {
            max_y = y
        };
    }

    let grid_width = (max_x - min_x) as usize + 2;
    let grid_height = (max_y - min_y) as usize + 2;
    let mut x = (x - min_x) as usize + 1;
    let mut y = (y - min_y) as usize + 1;
    let mut grid: Grid<u8, Vec<u8>> = Grid::new_with_value(grid_width, grid_height, b'.');
    grid[(x, y)] = b'#';
    let mut dug = 0;
    for dig in input.iter() {
        for _ in 0..dig.steps {
            (x, y) = dig.dir.next(x, y, 1);
            grid[(x, y)] = b'#';
            dug += 1;
        }
    }

    let point_inside = grid
        .rows()
        .find_map(|(y, row)| {
            if let Some(first_trench) = row.iter().position(|v| *v == b'#') {
                if first_trench == 0 {
                    if row[1] == b'.' {
                        Some((1, y))
                    } else {
                        None
                    }
                } else if first_trench < grid_width - 1
                    && row[first_trench - 1] == b'.'
                    && row[first_trench + 1] == b'.'
                {
                    Some((first_trench + 1, y))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap();

    bfs()
        .with_initial_state(point_inside)
        .gather::<u32, _, _>(|s, (x, y)| {
            let curr = grid[(x, y)];
            if curr == b'.' {
                s.add_state((x, y - 1));
                s.add_state((x - 1, y));
                s.add_state((x + 1, y));
                s.add_state((x, y + 1));
                Some(())
            } else {
                None
            }
        })
        + dug
}

fn p1_shoelace(input: &[Dig]) -> i64 {
    let (mut x, mut y) = (0, 0);
    shoelace(input.iter().map(|dig| {
        (x, y) = dig.dir.next(x, y, dig.steps as i64);
        (x, y, dig.steps as i64)
    }))
}

fn p2_shoelace(input: &[Dig]) -> i64 {
    let (mut x, mut y) = (0, 0);
    shoelace(input.iter().map(|dig| {
        let steps = (dig.color >> 4) as i64;
        let dir = match dig.color % 4 {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => unreachable!(),
        };

        (x, y) = dir.next(x, y, steps);
        (x, y, steps)
    }))
}

fn shoelace(iter: impl Iterator<Item = (i64, i64, i64)>) -> i64 {
    let (mut px, mut py) = (0, 0);
    let mut inner_area = 0;
    let mut outer_area = 1;
    let mut inner_halves = 0;

    for (x, y, steps) in iter {
        outer_area += steps;

        let determinant = (px * y) - (x * py);
        inner_area += determinant / 2;
        inner_halves += determinant % 2;

        (px, py) = (x, y);
    }

    assert_eq!(
        inner_halves % 2,
        0,
        "My code takes for granted that there are an even number of remainders"
    );

    inner_area + (inner_halves / 2) + (outer_area / 2) + 1
}

struct Dig {
    dir: Direction,
    steps: u8,
    color: u32,
}

impl Dig {
    fn parse_list(input: &[u8]) -> Vec<Self> {
        Self::parser()
            .delimited_by(b'\n')
            .repeat()
            .parse(input)
            .unwrap()
    }

    fn parser<'i>() -> impl Parser<'i, Self> {
        parse::choice((
            b'U'.map_to(Direction::Up),
            b'L'.map_to(Direction::Left),
            b'R'.map_to(Direction::Right),
            b'D'.map_to(Direction::Down),
        ))
        .and_discard(b' ')
        .and(parse::unsigned_int())
        .and_discard(b" (#")
        .and(
            parse::hex_digit()
                .repeat_n(6)
                .map(|v: [u32; 6]| v.iter().fold(0, |c, n| c * 16 + *n)),
        )
        .and_discard(b')')
        .map(|((dir, steps), color)| Self { dir, steps, color })
    }
}
#[derive(Copy, Clone, Debug)]
enum Direction {
    Up = 0,
    Left = 1,
    Right = 2,
    Down = 3,
}

impl Direction {
    #[inline]
    fn next<T: Copy + Add<T, Output = T> + Sub<T, Output = T>>(
        &self,
        x: T,
        y: T,
        steps: T,
    ) -> (T, T) {
        match self {
            Direction::Up => (x, y - steps),
            Direction::Left => (x - steps, y),
            Direction::Right => (x + steps, y),
            Direction::Down => (x, y + steps),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_EXAMPLE: &[u8] = b"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn p1_works_on_example() {
        assert_eq!(p1_bfs(&Dig::parse_list(P1_EXAMPLE)), 62);
        assert_eq!(p1_shoelace(&Dig::parse_list(P1_EXAMPLE)), 62);
    }

    #[test]
    fn p2_works_on_example() {
        assert_eq!(p2_shoelace(&Dig::parse_list(P1_EXAMPLE)), 952408144115);
    }
}
