use common::aoc::Day;
use common::grid::Grid;
use common::search::{dijkstra, Search, WithCost};

const BOUNDARY: u8 = 0;

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || parse(input));

    day.note("Input Width", input.width() - 2);
    day.note("Input Height", input.height() - 2);

    day.part("Part 1", || p1(&input));
    day.part("Part 2", || p2(&input));
}

fn p1(grid: &Grid<u8, Vec<u8>>) -> u32 {
    let goal = (grid.width() - 2, grid.height() - 2);

    dijkstra()
        .with_initial_state(WithCost(((1usize, 1usize), Direction::Right, 3u8), 0u32))
        .find(|s, WithCost(((x, y), dir, steps_left), cost)| {
            let curr = grid[(x, y)];
            if curr == BOUNDARY {
                return None;
            }
            if (x, y) == goal {
                return Some(cost);
            }

            let (left, right) = dir.left_right();
            if steps_left > 0 {
                let pos_ahead = dir.next(x, y);
                let cost_ahead = cost + grid[pos_ahead] as u32;
                s.add_state(WithCost((pos_ahead, dir, steps_left - 1), cost_ahead));
            }

            let pos_left = left.next(x, y);
            let cost_left = cost + grid[pos_left] as u32;
            let pos_right = right.next(x, y);
            let cost_right = cost + grid[pos_right] as u32;

            s.add_state(WithCost((pos_left, left, 2), cost_left));
            s.add_state(WithCost((pos_right, right, 2), cost_right));

            None
        })
        .unwrap()
}

fn p2(grid: &Grid<u8, Vec<u8>>) -> u32 {
    let goal = (grid.width() - 2, grid.height() - 2);

    dijkstra()
        .with_initial_state(WithCost(((1usize, 1usize), Direction::Right, 0u8), 0u32))
        .and_additional_state(WithCost(((1usize, 1usize), Direction::Down, 0u8), 0u32))
        .find(|s, WithCost(((x, y), dir, steps_since), cost)| {
            let curr = grid[(x, y)];
            if curr == BOUNDARY {
                return None;
            }
            if (x, y) == goal {
                if steps_since < 4 {
                    return None;
                }

                return Some(cost);
            }

            let (left, right) = dir.left_right();
            if steps_since < 10 {
                let pos_ahead = dir.next(x, y);
                let cost_ahead = cost + grid[pos_ahead] as u32;
                s.add_state(WithCost((pos_ahead, dir, steps_since + 1), cost_ahead));
            }

            if steps_since > 3 {
                let pos_left = left.next(x, y);
                let cost_left = cost + grid[pos_left] as u32;
                let pos_right = right.next(x, y);
                let cost_right = cost + grid[pos_right] as u32;

                s.add_state(WithCost((pos_left, left, 1), cost_left));
                s.add_state(WithCost((pos_right, right, 1), cost_right));
            }

            None
        })
        .unwrap()
}

fn parse(input: &[u8]) -> Grid<u8, Vec<u8>> {
    Grid::parse_padded_map(input, BOUNDARY, |f| f - b'0')
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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

    fn left_right(&self) -> (Direction, Direction) {
        match self {
            Direction::Up => (Direction::Left, Direction::Right),
            Direction::Left => (Direction::Down, Direction::Up),
            Direction::Right => (Direction::Up, Direction::Down),
            Direction::Down => (Direction::Right, Direction::Left),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_EXAMPLE: &[u8] = b"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    const P2_EXAMPLE: &[u8] = b"111111111111
999999999991
999999999991
999999999991
999999999991
";

    #[test]
    fn p1_works_on_example() {
        assert_eq!(p1(&parse(P1_EXAMPLE)), 102);
    }

    #[test]
    fn p2_works_on_examples() {
        assert_eq!(p2(&parse(P1_EXAMPLE)), 94);
        assert_eq!(p2(&parse(P2_EXAMPLE)), 71);
    }
}
