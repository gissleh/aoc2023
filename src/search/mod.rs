use crate::utils::gather_target::GatherTarget;
pub use bfs::bfs;
pub use dfs::dfs;
pub use dijkstra::{dijkstra, DijkstraState};
use num::{Integer, One};
use std::cmp::Ordering;
use std::ops::{AddAssign, SubAssign};
pub use utils::{WithCost, WithHint};

mod bfs;
mod dfs;
mod dijkstra;
mod utils;

pub trait Search<S>: Sized {
    fn reset(&mut self, initial_state: S);
    fn next_state(&mut self) -> Option<S>;
    fn add_state_unchecked(&mut self, state: S);
    fn has_seen_state(&mut self, state: &S) -> bool;

    fn add_state(&mut self, state: S) {
        if !self.has_seen_state(&state) {
            self.add_state_unchecked(state)
        }
    }

    /// Reset the search with this new state.
    fn with_initial_state(mut self, initial_state: S) -> Self {
        self.reset(initial_state);
        self
    }

    fn and_additional_state(mut self, state: S) -> Self {
        self.add_state(state);
        self
    }

    /// Find the next state that returns Some(R) from the callback. This function will not
    /// reset the state between runs, and you could continue where you left off by calling it
    /// again if the search is for multiple objects.
    fn find<F, R>(&mut self, f: F) -> Option<R>
    where
        F: Fn(&mut Self, S) -> Option<R>,
    {
        while let Some(state) = self.next_state() {
            if let Some(v) = f(self, state) {
                return Some(v);
            }
        }

        None
    }

    fn find_mut<F, R>(&mut self, mut f: F) -> Option<R>
    where
        F: FnMut(&mut Self, S) -> Option<R>,
    {
        while let Some(state) = self.next_state() {
            if let Some(v) = f(self, state) {
                return Some(v);
            }
        }

        None
    }

    /// maximize exhausts the search and returns the greatest value from it.
    fn maximize<F, R>(&mut self, f: F) -> Option<R>
    where
        F: Fn(&mut Self, S) -> Option<R>,
        R: Ord,
    {
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

    /// Fill does the same as `maximize`, but lets you also mutate an object within the
    /// loop that you get back after its completion.
    fn fill<F, M, R>(&mut self, mut m: M, mut f: F) -> (M, Option<R>)
    where
        F: FnMut(&mut Self, &mut M, S) -> Option<R>,
        R: Ord,
    {
        let mut king = None;

        while let Some(v) = self.find_mut(|search, state| f(search, &mut m, state)) {
            if let Some(k) = &mut king {
                if v > *k {
                    *k = v;
                }
            } else {
                king = Some(v);
            }
        }

        (m, king)
    }

    /// Gather exhausts find_next, but will stop early if the collection referred to in G is full.
    fn gather<G, F, R>(&mut self, f: F) -> G
    where
        F: Fn(&mut Self, S) -> Option<R>,
        G: GatherTarget<R>,
    {
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

pub fn binary_search<I, F>(start: I, initial_step: I, cb: F) -> Option<I>
where
    I: Integer + Copy + One,
    F: Fn(I) -> Ordering,
{
    let two = I::one() + I::one();
    let mut current = start;
    let mut step = initial_step;
    let mut ones_left = 32;

    while ones_left > 0 {
        match cb(current) {
            Ordering::Equal => {
                return Some(current);
            }
            Ordering::Less => {
                current = current.add(step);
            }
            Ordering::Greater => {
                current = current.sub(step);
            }
        }

        if step > I::one() {
            step = step.div(two);
        } else {
            ones_left -= 1;
        }
    }

    None
}

pub fn find_first_number<I, F>(
    start: I,
    end: I,
    initial_step: I,
    step_divide: I,
    cb: F,
) -> Option<I>
where
    I: Integer + Copy + One + AddAssign + SubAssign,
    F: Fn(I) -> bool,
{
    let one = I::one();
    let zero = I::zero();
    let mut current = start;
    let mut step = initial_step;
    let mut ones_left = step_divide + step_divide;
    let mut never_found = true;

    while ones_left > zero {
        if current > end {
            if never_found {
                current = initial_step;
            } else {
                current -= step;
            }

            step = step / step_divide;
            if step == zero {
                step = one;
            }
        }

        if cb(current) {
            never_found = false;

            if step == one {
                return Some(current);
            } else {
                current -= step;
                step = step / step_divide;
                if step == zero {
                    step = one;
                }
            }
        } else {
            if step == one {
                ones_left -= one;
            }

            current += step;
        }
    }

    None
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::geo::Point;

    pub(crate) const MAZE_01: &[u8] = include_bytes!("./test_fixtures/maze01.txt");
    pub(crate) const MAZE_02: &[u8] = include_bytes!("./test_fixtures/maze02.txt");

    pub(crate) fn search_maze<S: Search<Point<usize>>>(
        maze: &'static [u8],
    ) -> impl Fn(&mut S, Point<usize>) -> Option<(char, Point<usize>)> {
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
