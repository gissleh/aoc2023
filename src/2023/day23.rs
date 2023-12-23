use std::ops::{Add, Sub};
use arrayvec::ArrayVec;
use common::aoc::Day;
use common::ds::Graph;
use common::grid::Grid;
use common::search;
use common::search::{Search, WithCost};

const WALL: u8 = b'#';
const GOAL: u8 = b'*';
const SLIP_UP: u8 = b'^';
const SLIP_LEFT: u8 = b'<';
const SLIP_RIGHT: u8 = b'>';
const SLIP_DOWN: u8 = b'v';

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || parse(input));

    day.note("Input Width", input.width());
    day.note("Input Height", input.height());

    day.part("Part 1", || p1(&input));
    //day.part("Part 2", || 0);
}

fn parse(input: &[u8]) -> Grid<u8, Vec<u8>> {
    let mut grid = Grid::parse_padded(input, WALL);
    let (gw, gh) = (grid.width(), grid.height());
    grid[(gw - 3, gh - 2)] = GOAL;

    grid
}

fn p1(grid: &Grid<u8, Vec<u8>>) -> u32 {
    // Find the points of interest
    let mut knots = Vec::with_capacity(64);
    knots.push((2, 1));
    knots.push((grid.width() - 3, grid.height() - 2));
    for y in 2..grid.height()-2 {
        for x in 2..grid.width()-2 {
            if grid[(x, y)] == WALL {
                continue;
            }

            let count = (grid[(x, y - 1)] != WALL) as u8
                + (grid[(x - 1, y)] != WALL) as u8
                + (grid[(x + 1, y)] != WALL) as u8
                + (grid[(x, y + 1)] != WALL) as u8;

            if count > 2 {
                #[cfg(test)]
                println!("{} {} {} {} {}",
                    grid[(x, y)] as char,
                    grid[(x, y - 1)] as char,
                    grid[(x - 1, y)] as char,
                    grid[(x + 1, y)] as char,
                    grid[(x, y + 1)] as char,
                );

                knots.push((x, y));
            }
        }
    }

    // Set up the graph
    let mut graph = Graph::<(usize, usize), bool, u32, 4>::with_capacity(knots.len());
    for (i, (x, y)) in knots.iter().enumerate() {
        graph.create_node((*x, *y), i == 1);
    }

    // Measure out the graph
    let mut dfs = search::dfs();
    for i in 0..knots.len() {
        dfs.reset(WithCost(knots[i], 0u32));
        let neighbors: ArrayVec<(usize, u32), 4> = dfs.gather(|s, WithCost((x, y), cost)| {
            let cell = grid[(x, y)];
            if cell == GOAL {
                return Some(1);
            }

            let positions = [(x, y-1), (x-1, y), (x+1, y), (x, y+1)];
            let cells = positions.map(|pos| grid[pos]);
            let walls = cells.iter().filter(|c| **c == WALL).count();

            if walls < 2 {
                let index = knots.iter().position(|p| p.eq(&(x, y))).unwrap();
                Some(index)
            } else {
                for i in 0..positions.len() {
                    
                }
            }
        });

        for ((neighbor, cost)) in neighbors {
            graph.connect(i, neighbor, cost);
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
}