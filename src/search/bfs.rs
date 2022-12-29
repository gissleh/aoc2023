use std::collections::VecDeque;
use std::hash::Hash;
use hashbrown::HashSet;
use super::Search;

struct BFS<S> {
    seen: HashSet<S>,
    queue: VecDeque<S>,
}

impl<S> Search<S> for BFS<S> where S: Clone + Hash + Eq {
    fn reset(&mut self, initial_state: S) {
        self.seen.clear();
        self.queue.clear();
        self.seen.insert(initial_state.clone());
        self.queue.push_back(initial_state);
    }

    fn next_state(&mut self) -> Option<S> {
        self.queue.pop_front()
    }

    fn push_state(&mut self, state: S) {
        if self.seen.insert(state.clone()) {
            self.queue.push_back(state);
        }
    }
}

pub fn bfs<S>() -> impl Search<S> where S: Clone + Hash + Eq {
    BFS {
        queue: VecDeque::with_capacity(512),
        seen: HashSet::with_capacity(512),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geo::Point;

    const MAZE_01: &[u8] = include_bytes!("./test_fixtures/maze01.txt");

    #[test]
    fn bfs_can_gather() {
        let maze: Vec<&[u8]> = MAZE_01.split(|v| *v == b'\n').collect();

        let mut bfs = bfs();
        bfs.reset(Point::new(1usize, 1usize));

        let findings: Vec<(char, Point<usize>)> = bfs.gather(|search, p| {
            let [x, y] = *p.coords();
            let ch = maze[y][x];

            if ch == b'#' {
                return None;
            }

            search.push_state(Point::new(x, y - 1));
            search.push_state(Point::new(x - 1, y));
            search.push_state(Point::new(x + 1, y));
            search.push_state(Point::new(x, y + 1));

            if ch != b'.' {
                Some((ch as char, p))
            } else {
                None
            }
        });

        assert_eq!(findings.as_slice(), &[
            ('f', Point::new(1, 5)),
            ('g', Point::new(24, 2)),
            ('a', Point::new(26, 10)),
            ('d', Point::new(24, 7)),
            ('b', Point::new(28, 10)),
            ('c', Point::new(22, 7)),
            ('e', Point::new(8, 5)),
        ])
    }
}