use crate::search::DijkstraState;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
pub struct WithCost<S, C>(pub S, pub C);

impl<S, C> DijkstraState<C, S> for WithCost<S, C>
where
    C: Ord + Eq + Copy,
    S: Hash + Eq + Clone,
{
    fn cost(&self) -> C {
        self.1
    }

    fn key(&self) -> S {
        self.0.clone()
    }
}

impl<S, C> Eq for WithCost<S, C> where S: Eq {}

impl<S, C> PartialEq<Self> for WithCost<S, C>
where
    S: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<S, C> PartialOrd<Self> for WithCost<S, C>
where
    S: Eq + PartialOrd<S> + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl<S, C> Ord for WithCost<S, C>
where
    S: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<S, C> Hash for WithCost<S, C>
where
    S: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

/// WithHint allows you to pass along extra details in the state that
/// will not be considered by the hasher.
struct WithHint<S, H>(S, H);

impl<S, H> Eq for WithHint<S, H> where S: Eq {}

impl<S, H> PartialEq<Self> for WithHint<S, H>
where
    S: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<S, H> PartialOrd<Self> for WithHint<S, H>
where
    S: Eq + PartialOrd<S> + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl<S, H> Ord for WithHint<S, H>
where
    S: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<S, HI> Hash for WithHint<S, HI>
where
    S: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
