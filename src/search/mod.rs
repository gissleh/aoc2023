use crate::utils::gather_target::GatherTarget;
pub use bfs::bfs;
pub use dfs::dfs;
pub use dijkstra::{dijkstra, DijkstraState};
pub use utils::WithCost;

mod utils;
mod bfs;
mod dfs;
mod dijkstra;

pub trait Search<S>: Sized {
    fn reset(&mut self, initial_state: S);
    fn next_state(&mut self) -> Option<S>;
    fn add_state(&mut self, state: S);

    /// Reset the search with this new state.
    fn with_initial_state(mut self, initial_state: S) -> Self {
        self.reset(initial_state);
        self
    }

    /// Find the next state that returns Some(R) from the callback. This function will not
    /// reset the state between runs, and you could continue where you left off by calling it
    /// again if the search is for multiple objects.
    fn find<F, R>(&mut self, f: F) -> Option<R> where F: Fn(&mut Self, S) -> Option<R> {
        while let Some(state) = self.next_state() {
            if let Some(v) = f(self, state) {
                return Some(v);
            }
        }

        None
    }

    /// maximize exhausts the search and returns the greatest value from it.
    fn maximize<F, R>(&mut self, f: F) -> Option<R> where F: Fn(&mut Self, S) -> Option<R>, R: Ord {
        let mut king = None;

        while let Some(v) = self.find(&f) {
            if let Some(k) = &mut king {
                if v > *k {
                    *k = v;
                }
            } else {
                king = Some(v);
            }
        }

        king
    }

    /// Gather exhausts find_next, but will stop early if the collection referred to in G is full.
    fn gather<G, F, R>(&mut self, f: F) -> G where F: Fn(&mut Self, S) -> Option<R>, G: GatherTarget<R> {
        let mut gather_target = G::start_gathering(0);
        let mut index = 0;

        while let Some(v) = self.find(&f) {
            let full = gather_target.gather_into(index, v);
            if full {
                break;
            }
            index += 1;
        }

        gather_target
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::geo::Point;

    pub(crate) const MAZE_01: &[u8] = include_bytes!("./test_fixtures/maze01.txt");
    pub(crate) const MAZE_02: &[u8] = include_bytes!("./test_fixtures/maze02.txt");

    pub(crate) fn search_maze<S: Search<Point<usize>>>(maze: &'static [u8]) -> impl Fn(&mut S, Point<usize>) -> Option<(char, Point<usize>)> {
        let maze: Vec<&[u8]> = maze.split(|v| *v == b'\n').collect();

        move |search, p| {
            let [x, y] = *p.coords();
            let ch = maze[y][x];

            if ch == b'#' {
                return None;
            }

            search.add_state(Point::new(x, y - 1));
            search.add_state(Point::new(x - 1, y));
            search.add_state(Point::new(x + 1, y));
            search.add_state(Point::new(x, y + 1));

            if ch != b'.' {
                Some((ch as char, p))
            } else {
                None
            }
        }
    }
}