use crate::utils::gather_target::GatherTarget;
pub use bfs::bfs;

mod bfs;

pub trait Search<S>: Sized {
    fn reset(&mut self, initial_state: S);
    fn next_state(&mut self) -> Option<S>;
    fn push_state(&mut self, state: S);

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
