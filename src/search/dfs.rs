use std::hash::Hash;
use hashbrown::HashSet;
use crate::search::Search;

struct DFS<S> {
    seen: HashSet<S>,
    stack: Vec<S>,
}

impl<S> Search<S> for DFS<S> where S: Clone + Hash + Eq {
    fn reset(&mut self, initial_state: S) {
        self.seen.clear();
        self.stack.clear();
        self.seen.insert(initial_state.clone());
        self.stack.push(initial_state);
    }

    fn next_state(&mut self) -> Option<S> {
        self.stack.pop()
    }

    fn add_state(&mut self, state: S) {
        if self.seen.insert(state.clone()) {
            self.stack.push(state);
        }
    }
}

pub fn dfs<S>() -> impl Search<S> where S: Clone + Hash + Eq {
    DFS {
        stack: Vec::with_capacity(512),
        seen: HashSet::with_capacity(512),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geo::Point;
    use crate::search::tests::{MAZE_02, search_maze};

    #[test]
    fn dfs_can_gather_too() {
        let mut dfs = dfs();

        dfs.reset(Point::new(4usize, 4usize));
        let findings: Vec<(char, Point<usize>)> = dfs.gather(search_maze(MAZE_02));
        assert_eq!(findings.as_slice(), &[
            ('b', Point::new(4, 5)),
            ('r', Point::new(6, 4)),
            ('l', Point::new(1, 4)),
            ('u', Point::new(4, 1)),
        ], "The last in reading order goes first with DFS.");
    }
}