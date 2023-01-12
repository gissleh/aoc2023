use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::Hash;
use std::marker::PhantomData;
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use crate::search::Search;

struct Dijkstra<C, K, S> where C: Ord + Eq, K: Hash + Eq, S: DijkstraState<C, K> {
    seen: HashMap<K, C>,
    open: BinaryHeap<DijkstraStep<C, K, S>>,
}

impl<C, H, S> Search<S> for Dijkstra<C, H, S> where C: Ord + Eq, H: Hash + Eq + PartialEq, S: DijkstraState<C, H> {
    fn reset(&mut self, initial_state: S) {
        self.open.clear();
        self.seen.clear();

        self.open.push(DijkstraStep(
            initial_state.cost(),
            initial_state.clone(),
            Default::default(),
        ));
        self.seen.insert(initial_state.key(), initial_state.cost());
    }

    fn next_state(&mut self) -> Option<S> {
        self.open.pop().map(|DijkstraStep(_, s, _)| s)
    }

    fn push_state(&mut self, state: S) {
        let seen_key = state.key();
        let step_cost = state.cost();

        match self.seen.entry(seen_key) {
            Entry::Occupied(mut entry) => {
                let seen_cost = entry.get_mut();
                if step_cost >= *seen_cost {
                    return;
                }
                *seen_cost = step_cost;
            }
            Entry::Vacant(entry) => {
                entry.insert(step_cost);
            }
        }

        self.open.push(DijkstraStep(
            state.cost(), state,
            PhantomData::default(),
        ));
    }
}

struct DijkstraStep<C, K, S> (C, S, PhantomData<K>);

impl<C, K, S> Eq for DijkstraStep<C, K, S> where C: Eq + Ord, K: Eq + Hash, S: DijkstraState<C, K> {}

impl<C, K, S> PartialEq<Self> for DijkstraStep<C, K, S> where C: Eq + Ord, K: Eq + Hash, S: DijkstraState<C, K> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<C, K, S> PartialOrd<Self> for DijkstraStep<C, K, S> where C: Eq + Ord, K: Eq + Hash, S: DijkstraState<C, K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<C, K, S> Ord for DijkstraStep<C, K, S> where S: DijkstraState<C, K>, C: Ord + Eq, K: Hash + Eq {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

pub trait DijkstraState<C: Ord + Eq, K: Hash + Eq>: Clone {
    fn cost(&self) -> C;
    fn key(&self) -> K;
}

/// Run dijkstra search. The state needs to implement DijkstraState<T> and is not interchangeable
/// with bfs and dfs without changing up how costs and such are handled.
pub fn dijkstra<C, K, S>() -> impl Search<S> where C: Ord + Eq, K: Hash + Eq, S: DijkstraState<C, K> {
    let mut dijkstra = Dijkstra {
        seen: HashMap::default(),
        open: BinaryHeap::with_capacity(128),
    };

    dijkstra
}
