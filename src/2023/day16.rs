use common::aoc::Day;
use common::grid::Grid;

const EMPTY: u8 = b'.';
const MIRROR_TR_BL: u8 = b'/';
const MIRROR_TL_BR: u8 = b'\\';
const SPLITTER_H: u8 = b'-';
const SPLITTER_V: u8 = b'|';
const EDGE: u8 = b'E';

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || Grid::parse_padded(input, EDGE));

    day.note("Input Width", input.width() - 2);
    day.note("Input Height", input.height() - 2);

    day.part("Part 1", || check_from(&input, (1, 1), Direction::Right));
    day.part("Part 2", || find_best(&input));
}

fn find_best(input: &Grid<u8, Vec<u8>>) -> usize {
    let mut highest = 0;

    for x in 1..input.width() - 1 {
        let top = check_from(input, (x, 1), Direction::Down);
        let bottom = check_from(input, (x, input.height() - 2), Direction::Up);

        if top > highest {
            highest = top;
        }
        if bottom > highest {
            highest = top;
        }
    }

    for y in 1..input.width() - 1 {
        let leftmost = check_from(input, (1, y), Direction::Right);
        let rightmost = check_from(input, (input.width() - 2, y), Direction::Left);

        if leftmost > highest {
            highest = leftmost;
        }
        if rightmost > highest {
            highest = leftmost;
        }
    }

    highest
}

fn check_from(input: &Grid<u8, Vec<u8>>, start: (usize, usize), dir: Direction) -> usize {
    let mut seen = vec![0u8; input.height() * input.width()];
    let mut stack = Vec::with_capacity(8);

    stack.push((start, dir));

    while let Some(((x, y), dir)) = stack.pop() {
        let curr = input[(x, y)];
        if curr == EDGE {
            continue;
        }

        let i = (y * input.height()) + x;
        if seen[i] & 1 << dir as u8 != 0 {
            continue;
        }
        seen[i] |= 1 << dir as u8;

        match curr {
            MIRROR_TL_BR => {
                let next_dir = dir.reflect_tl_br();
                stack.push((next_dir.next(x, y), next_dir))
            }

            MIRROR_TR_BL => {
                let next_dir = dir.reflect_tr_bl();
                stack.push((next_dir.next(x, y), next_dir))
            }

            EMPTY => stack.push((dir.next(x, y), dir)),

            SPLITTER_H => {
                if let Some((dir1, dir2)) = dir.split(false) {
                    stack.push((dir1.next(x, y), dir1));
                    stack.push((dir2.next(x, y), dir2));
                } else {
                    stack.push((dir.next(x, y), dir))
                }
            }

            SPLITTER_V => {
                if let Some((dir1, dir2)) = dir.split(true) {
                    stack.push((dir1.next(x, y), dir1));
                    stack.push((dir2.next(x, y), dir2));
                } else {
                    stack.push((dir.next(x, y), dir))
                }
            }

            _ => panic!("Unexpected: {}", curr as char),
        }
    }

    seen.iter().filter(|v| **v > 0).count()
}

#[derive(Copy, Clone)]
enum Direction {
    Up = 0,
    Left = 1,
    Right = 2,
    Down = 3,
}

impl Direction {
    fn next(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
        }
    }

    fn reflect_tr_bl(self) -> Direction {
        // /
        match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Left,
        }
    }

    fn reflect_tl_br(self) -> Direction {
        // \
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }

    fn split(self, hor: bool) -> Option<(Direction, Direction)> {
        match (self, hor) {
            (Direction::Up, false) => Some((Direction::Left, Direction::Right)),
            (Direction::Down, false) => Some((Direction::Left, Direction::Right)),
            (Direction::Left, true) => Some((Direction::Up, Direction::Down)),
            (Direction::Right, true) => Some((Direction::Up, Direction::Down)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_EXAMPLE: &[u8] = br#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

    #[test]
    fn p1_works_on_example() {
        assert_eq!(
            check_from(
                &Grid::parse_padded(P1_EXAMPLE, EDGE),
                (1, 1),
                Direction::Right
            ),
            46
        );
    }
}
