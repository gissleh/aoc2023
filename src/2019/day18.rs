use arrayvec::ArrayVec;
use common::aoc::Day;
use common::ds::Graph;
use common::geo::Point;
use common::grid::Grid;
use common::parse::{Parser, any_byte};
use common::search::{Search, bfs, WithCost, dijkstra};

type MazeGrid = Grid<Maze, [Maze; 8192]>;
type MazeGraph = Graph<Maze, Point<usize>, (u32, u32, u32), 32>;

pub fn main(day: &mut Day, input: &[u8]) {
    let grid_p1 = day.prep("Parse Grid", || parse_grid(input));

    day.part("Part 1 (Naive)", || p1_naive(&grid_p1));
    day.mark_dead_end();

    day.branch_from("Parse Grid");
    let graph_p1 = day.prep("Build Graph 1", || build_graph(&grid_p1));
    day.part("Part 1 (Graph)", || p1_graph(&graph_p1));
    let grid_p2 = day.prep("Update Grid", || change_grid_for_p2(&grid_p1));
    let graph_p2 = day.prep("Build Graph 2", || build_graph(&grid_p2));
    day.part("Part 2 (Graph)", || p2_graph(&graph_p2));
}

fn p1_graph(graph: &MazeGraph) -> u32 {
    let initial_pos = graph.find(&Maze::Entrance).unwrap();
    let initial_state = State { pos: initial_pos, keys: 0 };
    let target_mask = (1 << (graph.len() - 1)) - 1;

    dijkstra()
        .with_initial_state(WithCost(initial_state, 0))
        .find(|dijkstra, WithCost(state, steps)| {
            for ((distance, keys_required, keys_grabbed), next, _) in graph.edges_from(state.pos) {
                if state.keys & *keys_required == *keys_required {
                    let mut state = state;

                    state.pos = *next;
                    state.keys |= keys_grabbed;
                    let steps = steps + *distance;

                    if state.keys == target_mask {
                        return Some(steps);
                    }

                    dijkstra.add_state(WithCost(state, steps));
                }
            }

            None
        }).unwrap()
}

fn p2_graph(graph: &MazeGraph) -> u32 {
    let initial_pos_array: ArrayVec<usize, 4> = graph.nodes()
        .filter(|(_, m, ..)| **m == Maze::Entrance)
        .map(|(i, ..)| i)
        .collect();
    let mut initial_pos = [0usize; 4];
    for (i, pos) in initial_pos_array.iter().enumerate() {
        initial_pos[i] = *pos;
    }

    let initial_state = State { pos: initial_pos, keys: 0 };
    let target_mask = (1 << (graph.len() - 4)) - 1;

    dijkstra()
        .with_initial_state(WithCost(initial_state, 0))
        .find(|dijkstra, WithCost(state, steps)| {
            for (i, pos) in state.pos.iter().enumerate() {
                for ((distance, keys_required, keys_grabbed), next, _) in graph.edges_from(*pos) {
                    if state.keys & *keys_required == *keys_required {
                        let mut state = state;

                        state.pos[i] = *next;
                        state.keys |= keys_grabbed;
                        let steps = steps + *distance;

                        if state.keys == target_mask {
                            return Some(steps);
                        }

                        dijkstra.add_state(WithCost(state, steps));
                    }
                }
            }

            None
        }).unwrap()
}

fn build_graph(grid: &MazeGrid) -> MazeGraph {
    let mut graph = Graph::with_capacity(32);

    for (pos, cell) in grid.iter() {
        match *cell {
            Maze::Door(_) | Maze::Wall | Maze::Floor => {}
            _ => { graph.create_node(*cell, pos); }
        }
    }

    let mut bfs = bfs();
    for i in 0..graph.len() {
        let (_, pos) = graph.node(i).unwrap();
        bfs.reset(WithCost(*pos, (0, 0, 0)));

        let edges: ArrayVec<(Maze, u32, u32, u32), 32> = bfs.gather(|bfs, state| {
            let WithCost(pos, (steps, mut keys_required, mut keys_found)) = state;

            let mut res = None;

            match grid[pos] {
                Maze::Wall => { return None; }
                Maze::Floor => {}
                Maze::Key(mask) => {
                    keys_found |= mask;
                    res = Some((Maze::Key(mask), steps, keys_required, keys_found));
                }
                Maze::Door(mask) => {
                    keys_required |= mask;
                }
                Maze::Entrance => {}
            }

            for pos in pos.cardinals() {
                bfs.add_state(WithCost(pos, (steps + 1, keys_required, keys_found)));
            }

            res
        });

        for (cell, steps, keys_required, keys_found) in edges {
            graph.connect(i, graph.find(&cell).unwrap(), (steps, keys_required, keys_found));
        }
    }

    graph
}

fn change_grid_for_p2(grid: &MazeGrid) -> MazeGrid {
    let mut grid = grid.clone();
    let corner = grid.find(&Maze::Entrance).unwrap() - Point::new(1, 1);

    grid[corner + Point::new(0, 0)] = Maze::Entrance;
    grid[corner + Point::new(1, 0)] = Maze::Wall;
    grid[corner + Point::new(2, 0)] = Maze::Entrance;
    grid[corner + Point::new(0, 1)] = Maze::Wall;
    grid[corner + Point::new(1, 1)] = Maze::Wall;
    grid[corner + Point::new(2, 1)] = Maze::Wall;
    grid[corner + Point::new(0, 2)] = Maze::Entrance;
    grid[corner + Point::new(1, 2)] = Maze::Wall;
    grid[corner + Point::new(2, 2)] = Maze::Entrance;

    grid
}

fn p1_naive(grid: &MazeGrid) -> u32 {
    let initial_pos = grid.find(&Maze::Entrance).unwrap();
    let initial_state = State { pos: initial_pos, keys: 0 };
    let target_mask = (1 << grid.count_by(|v| v.is_key())) - 1;

    bfs().with_initial_state(WithCost(initial_state, 0))
        .find(|bfs, WithCost(mut state, steps)| {
            match grid[state.pos] {
                Maze::Wall => { return None; }
                Maze::Key(mask) => {
                    state.keys |= mask;
                    if state.keys == target_mask {
                        return Some(steps);
                    }
                }
                Maze::Door(mask) => {
                    if mask & state.keys != mask {
                        return None;
                    }
                }
                Maze::Floor | Maze::Entrance => {}
            };

            for pos in state.pos.cardinals() {
                state.pos = pos;
                bfs.add_state(WithCost(state, steps + 1));
            }

            None
        }).unwrap()
}

#[allow(dead_code)]
fn p2_naive(grid: &MazeGrid) -> u32 {
    let initial_pos = grid.find(&Maze::Entrance).unwrap();
    let initial_state = State {
        pos: [
            initial_pos, initial_pos + Point::new(2, 0),
            initial_pos + Point::new(0, 2), initial_pos + Point::new(2, 2),
        ],
        keys: 0,
    };
    let target_mask = (1 << grid.count_by(|v| v.is_key())) - 1;

    bfs().with_initial_state(WithCost(initial_state, 0))
        .find(|bfs, WithCost(mut state, steps)| {
            for pos in state.pos.iter() {
                match grid[*pos] {
                    Maze::Wall => { return None; }
                    Maze::Key(mask) => {
                        state.keys |= mask;
                        if state.keys == target_mask {
                            return Some(steps);
                        }
                    }
                    Maze::Door(mask) => {
                        if mask & state.keys != mask {
                            return None;
                        }
                    }
                    Maze::Floor | Maze::Entrance => {}
                };
            }

            for (i, pos) in state.pos.iter().enumerate() {
                for pos in pos.cardinals() {
                    let mut state = state;
                    state.pos[i] = pos;

                    bfs.add_state(WithCost(state, steps + 1));
                }
            }

            None
        }).unwrap()
}

fn parse_grid(input: &[u8]) -> MazeGrid {
    Grid::parser(any_byte().map(|b| match b {
        b'@' => Maze::Entrance,
        b'.' => Maze::Floor,
        b'#' => Maze::Wall,
        b'a'..=b'z' => Maze::Key(1 << (b - b'a')),
        b'A'..=b'Z' => Maze::Door(1 << (b - b'A')),
        _ => panic!("{} is not a valid maze cell", b as char),
    })).parse(input).unwrap()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
enum Maze {
    #[default]
    Wall,
    Floor,
    Door(u32),
    Key(u32),
    Entrance,
}

impl Maze {
    fn is_key(&self) -> bool {
        match self {
            Maze::Key(_) => true,
            _ => false,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct State<P> {
    pos: P,
    keys: u32,
}