use common::aoc::{Day, ResultCarrying};
use common::geo::Point;
use common::grid::Grid;
use common::parse;
use common::parse::Parser;
use common::search;
use common::search::{Search, WithCost};

// 0bLURD

const LEFT: u8 = 0b1000;
const UP: u8 = 0b0100;
const RIGHT: u8 = 0b0010;
const DOWN: u8 = 0b0001;

const PIPE_LR: u8 = LEFT | RIGHT;
const PIPE_UD: u8 = UP | DOWN;
const PIPE_J: u8 = LEFT | UP;
const PIPE_L: u8 = RIGHT | UP;
const PIPE_7: u8 = LEFT | DOWN;
const PIPE_F: u8 = RIGHT | DOWN;
const NO_PIPE: u8 = 0b0000;
const START: u8 = 0b1111;

const DIRECTIONS: [u8; 4] = [ UP, LEFT, RIGHT, DOWN ];
const OPENINGS: [u8; 4] = [ DOWN, RIGHT, LEFT, UP ];

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || parse(input));

    day.note("Input width", input.width());
    day.note("Input height", input.height());

    let ResultCarrying(_, grid) = day.part("Part 1", || p1(&input));
    day.part("Part 2", || p2(&grid));
}

fn p1(input: &Grid<u8, Vec<u8>>) -> ResultCarrying<u32, Grid<u8, Vec<u8>>> {
    let pos = input.find(&START).unwrap();
    let tl = Point::new(0, 0);
    let br = Point::new(input.width(), input.height());

    let grid = Grid::<u8, Vec<u8>>::new_with_value(input.width(), input.height(), 0);

    let (grid, res) = search::bfs().with_initial_state(WithCost(pos, 0u32))
        .fill(grid, |s, grid, WithCost(pos, steps)| {
            let curr = input[pos];
            let mut cell = 0;

            for (i, next_pos) in pos.cardinals_within(tl, br).iter().enumerate() {
                if curr & DIRECTIONS[i] == 0 {
                    continue;
                }

                if let Some(next_pos) = next_pos {
                    let next = input[*next_pos];
                    if next & OPENINGS[i] == 0 {
                        continue;
                    }

                    cell = cell | DIRECTIONS[i];

                    s.add_state(WithCost(*next_pos, steps + 1))
                }
            }

            grid[pos] = cell;

            Some(steps)
        });

    ResultCarrying(res.unwrap(), grid)
}

fn p2(grid: &Grid<u8, Vec<u8>>) -> u32 {
    let mut count = 0;
    let mut inside = false;
    let mut expect = START;

    for (p, curr) in grid.iter() {
        let [x, _] = p.coords();
        if *x == 0 {
            inside = false;
            expect = START;
        }

        if *curr == NO_PIPE && inside {
            count += 1;
            continue;
        }

        let curr_ud = *curr & PIPE_UD;
        if curr_ud == PIPE_UD || curr_ud == expect {
            inside = !inside;
            expect = START;
        } else if curr_ud != 0 {
            if expect == START {
                expect = if curr_ud == UP { DOWN } else { UP };
            } else {
                expect = START;
            }
        }
    }

    count
}

fn parse(input: &[u8]) -> Grid<u8, Vec<u8>> {
    Grid::parser(parse::any_byte().map(|s| match s {
        b'.' => NO_PIPE,
        b'|' => PIPE_UD,
        b'-' => PIPE_LR,
        b'L' => PIPE_L,
        b'J' => PIPE_J,
        b'7' => PIPE_7,
        b'F' => PIPE_F,
        b'S' => START,
        _ => panic!("Unknown symbol {}", s),
    })).parse(input).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1_EXAMPLE_1: &[u8] = b".....
.S-7.
.|.|.
.L-J.
.....
";

    const P1_EXAMPLE_2: &[u8] = b"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    #[test]
    fn p1_works_on_examples() {
        assert_eq!(p1(&parse(P1_EXAMPLE_1)).res(), 4);
        assert_eq!(p1(&parse(P1_EXAMPLE_2)).res(), 8);
    }

    const P2_EXAMPLE_1: &[u8] = b"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    const P2_EXAMPLE_2: &[u8] = b".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const P2_EXAMPLE_3: &[u8] = b"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn p2_works_on_examples() {
        print_grid(&p1(&parse(P2_EXAMPLE_1)).carry());
        assert_eq!(p2(&p1(&parse(P2_EXAMPLE_1)).carry()), 4);
        print_grid(&p1(&parse(P2_EXAMPLE_2)).carry());
        assert_eq!(p2(&p1(&parse(P2_EXAMPLE_2)).carry()), 8);
        print_grid(&p1(&parse(P2_EXAMPLE_3)).carry());
        assert_eq!(p2(&p1(&parse(P2_EXAMPLE_3)).carry()), 10);
    }

    fn print_grid(g: &Grid<u8, Vec<u8>>) {
        for y in 0..g.height() {
            for x in 0..g.width() {
                match g[Point::new(x, y)] {
                    PIPE_LR => print!("-"),
                    PIPE_UD => print!("|"),
                    PIPE_J => print!("J"),
                    PIPE_L => print!("L"),
                    PIPE_7 => print!("7"),
                    PIPE_F => print!("F"),
                    NO_PIPE => print!("."),
                    START => print!("S"),
                    _ => panic!("{}", g[Point::new(x, y)]),
                }
            }

            println!();
        }

        println!();
    }
}