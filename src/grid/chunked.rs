use std::marker::PhantomData;
use crate::grid::Grid;

pub struct ChunkedGrid<T, S> {

}

struct Chunk<T, S> {
    neighs: [usize; 4],
    top: isize,
    left: isize,
    grid: Grid<T, S>,
    spooky: PhantomData<T>
}