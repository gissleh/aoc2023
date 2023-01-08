use std::ops::{Index, IndexMut};

pub trait GridStorage<T> : Index<usize, Output=T> + IndexMut<usize, Output=T> where T: Sized {
    fn create(size: usize, value: T) -> Self;
    fn cell_range(&self, i: usize, j: usize) -> &[T];
    fn cell_range_mut(&mut self, i: usize, j: usize) -> &mut [T];
}

impl<const N: usize, T> GridStorage<T> for [T; N] where T: Copy {
    #[inline]
    fn create(size: usize, value: T) -> Self {
        assert!(size < N);
        [value; N]
    }

    #[inline]
    fn cell_range(&self, i: usize, j: usize) -> &[T] { &self[i..j] }
    #[inline]
    fn cell_range_mut(&mut self, i: usize, j: usize) -> &mut [T] { &mut self[i..j] }
}

impl<T> GridStorage<T> for Vec<T> where T: Copy {
    #[inline]
    fn create(size: usize, value: T) -> Self {
        let mut vec = Vec::with_capacity(size);
        vec.resize(size, value);
        vec
    }

    #[inline]
    fn cell_range(&self, i: usize, j: usize) -> &[T] { &self[i..j] }
    #[inline]
    fn cell_range_mut(&mut self, i: usize, j: usize) -> &mut [T] { &mut self[i..j] }
}