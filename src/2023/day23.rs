use arrayvec::ArrayVec;
use common::aoc::Day;
use common::ds::Graph;
use common::grid::Grid;
use common::search;
use common::search::{Search, WithCost};

const WALL: u8 = b'#';
const FLOOR: u8 = b'.';
const GOAL: u8 = b'*';
const SLIP_UP: u8 = b'^';
const SLIP_LEFT: u8 = b'<';
const SLIP_RIGHT: u8 = b'>';
const SLIP_DOWN: u8 = b'v';

type MazeGraph = Graph<(usize, usize), bool, (u32, bool), 4>;

pub fn main(day: &mut Day, input: &[u8]) {
    let input = day.prep("Parse", || parse(input));
    let graph = day.prep("Build", || build(&input));

    day.note("Input Width", input.width());
    day.note("Input Height", input.height());
    day.note("Graph Nodes", graph.len());

    day.part("Part 1", || p1(&graph));
    day.part("Part 2", || p2(&graph));
}

fn p1(graph: &MazeGraph) -> u32 {
    p1_step::<false>(graph, 0, 1).unwrap()
}

fn p2(graph: &MazeGraph) -> u32 {
    p1_step::<true>(graph, 0, 1).unwrap()
}

fn p1_step<const UHBW: bool>(graph: &MazeGraph, current: usize, visited: u64) -> Option<u32> {
    if current == 1 {
        return Some(0)
    }

    graph.edges_from(current)
        .filter_map(|((cost, permitted), next, _)| {
            if (!UHBW && !*permitted) || visited & 1 << *next != 0 {
                None
            } else if let Some(inner_cost) = p1_step::<UHBW>(graph, *next, visited | 1 << *next) {
                Some(*cost + inner_cost)
            } else {
                None
            }
        })
        .max()
}

fn parse(input: &[u8]) -> Grid<u8, Vec<u8>> {
    let mut grid = Grid::parse_padded(input, WALL);
    let (gw, gh) = (grid.width(), grid.height());
    grid[(gw - 3, gh - 2)] = GOAL;

    grid
}

fn build(grid: &Grid<u8, Vec<u8>>) -> MazeGraph {
    // Find the points of interest
    let mut knots = Vec::with_capacity(64);
    knots.push((2, 1));
    knots.push((grid.width() - 3, grid.height() - 2));
    for y in 2..grid.height() - 2 {
        for x in 2..grid.width() - 2 {
            if grid[(x, y)] == WALL {
                continue;
            }

            let count = (grid[(x, y - 1)] != WALL) as u8
                + (grid[(x - 1, y)] != WALL) as u8
                + (grid[(x + 1, y)] != WALL) as u8
                + (grid[(x, y + 1)] != WALL) as u8;

            if count > 2 {
                #[cfg(test)]
                println!(
                    "{} {} {} {} {}",
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
    let mut graph = Graph::with_capacity(knots.len());
    for (i, (x, y)) in knots.iter().enumerate() {
        graph.create_node((*x, *y), i == 1);
    }

    // Measure out the graph
    let mut dfs = search::dfs();
    for i in 0..knots.len() {
        dfs.reset(WithCost(knots[i], 0u32));
        let neighbors: ArrayVec<(usize, u32), 4> = dfs.gather(|s, WithCost((x, y), cost)| {
            let cell = grid[(x, y)];
            if cell == GOAL && cost > 0 {
                return Some((1, cost));
            }

            let positions = [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)];
            let cells = positions.map(|pos| grid[pos]);
            let walls = cells.iter().filter(|c| **c == WALL).count();

            if walls < 2 && cost > 0 {
                let index = knots.iter().position(|p| p.eq(&(x, y))).unwrap();
                if index != i {
                    Some((index, cost))
                } else {
                    None
                }
            } else {
                match cell {
                    SLIP_UP => s.add_state(WithCost((x, y - 1), cost + 1)),
                    SLIP_LEFT => s.add_state(WithCost((x - 1, y), cost + 1)),
                    SLIP_RIGHT => s.add_state(WithCost((x + 1, y), cost + 1)),
                    SLIP_DOWN => s.add_state(WithCost((x, y + 1), cost + 1)),
                    FLOOR => {
                        for (i, pos) in positions.iter().enumerate() {
                            if cells[i] != WALL {
                                s.add_state(WithCost(*pos, cost + 1))
                            }
                        }
                    }
                    _ => {}
                }

                None
            }
        });

        for (neighbor, cost) in neighbors {
            #[cfg(test)]
            println!("{} -> {} = {}", i, neighbor, cost);
            graph.replace_connection(i, neighbor, (cost, true));
            graph.fallback_connection(neighbor, i, (cost, false));
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;
}
