use super::Search;
use hashbrown::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;

struct BFS<S> {
    seen: HashSet<S>,
    queue: VecDeque<S>,
}

impl<S> Search<S> for BFS<S>
where
    S: Clone + Hash + Eq,
{
    fn reset(&mut self, initial_state: S) {
        self.seen.clear();
        self.queue.clear();
        self.seen.insert(initial_state.clone());
        self.queue.push_back(initial_state);
    }

    fn next_state(&mut self) -> Option<S> {
        self.queue.pop_front()
    }

    fn add_state(&mut self, state: S) {
        if self.seen.insert(state.clone()) {
            self.queue.push_back(state);
        }
    }

    fn add_state_unchecked(&mut self, state: S) {
        self.queue.push_back(state);
    }

    fn has_seen_state(&mut self, state: &S) -> bool {
        self.seen.contains(state)
    }
}

pub fn bfs<S>() -> impl Search<S>
where
    S: Clone + Hash + Eq,
{
    BFS {
        queue: VecDeque::with_capacity(512),
        seen: HashSet::with_capacity(512),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geo::Point;
    use crate::search::tests::{search_maze, MAZE_01, MAZE_02};

    #[test]
    fn bfs_can_gather() {
        let mut bfs = bfs();

        bfs.reset(Point::new(4usize, 4usize));
        let findings: Vec<(char, Point<usize>)> = bfs.gather(search_maze(MAZE_02));
        assert_eq!(
            findings.as_slice(),
            &[
                ('b', Point::new(4, 5)),
                ('r', Point::new(6, 4)),
                ('u', Point::new(4, 1)),
                ('l', Point::new(1, 4)),
            ]
        );

        bfs.reset(Point::new(1usize, 1usize));
        let findings: Vec<(char, Point<usize>)> = bfs.gather(search_maze(MAZE_01));
        assert_eq!(
            findings.as_slice(),
            &[
                ('f', Point::new(1, 5)),
                ('g', Point::new(24, 2)),
                ('a', Point::new(26, 10)),
                ('d', Point::new(24, 7)),
                ('b', Point::new(28, 10)),
                ('c', Point::new(22, 7)),
                ('e', Point::new(8, 5)),
            ]
        );
    }
}
