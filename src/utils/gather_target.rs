use arrayvec::ArrayVec;
use smallvec::{Array, SmallVec};

/// GatherTarget is a common trait used to define collections for gathering results into when
/// parsing or searching.
pub trait GatherTarget<T> {
    fn start_gathering(size_hint: usize) -> Self;
    fn gather_into(&mut self, index: usize, value: T) -> bool;
}

impl<T> GatherTarget<T> for Vec<T> {
    fn start_gathering(size_hint: usize) -> Self {
        Vec::with_capacity(size_hint)
    }

    fn gather_into(&mut self, _index: usize, value: T) -> bool {
        self.push(value);
        false
    }
}

impl<const N: usize, T> GatherTarget<T> for [T; N]
where
    T: Default + Copy,
{
    fn start_gathering(_size_hint: usize) -> Self {
        [T::default(); N]
    }

    fn gather_into(&mut self, index: usize, value: T) -> bool {
        self[index] = value;
        index == self.len() - 1
    }
}

impl<const N: usize, T> GatherTarget<T> for ([T; N], usize)
where
    T: Default + Copy,
{
    fn start_gathering(_size_hint: usize) -> Self {
        ([T::default(); N], 0)
    }

    fn gather_into(&mut self, index: usize, value: T) -> bool {
        self.0[index] = value;
        self.1 = index + 1;
        index == self.0.len() - 1
    }
}

impl<const N: usize, T> GatherTarget<T> for ArrayVec<T, N> {
    fn start_gathering(_size_hint: usize) -> Self {
        ArrayVec::new()
    }

    fn gather_into(&mut self, _index: usize, value: T) -> bool {
        self.push(value);
        self.is_full()
    }
}

impl<A, T> GatherTarget<T> for SmallVec<A>
where
    A: Array<Item = T>,
{
    fn start_gathering(_size_hint: usize) -> Self {
        SmallVec::new()
    }

    fn gather_into(&mut self, _index: usize, value: T) -> bool {
        self.push(value);
        false
    }
}

impl<T> GatherTarget<T> for (T, T)
where
    T: Default,
{
    fn start_gathering(_size_hint: usize) -> Self {
        (T::default(), T::default())
    }

    fn gather_into(&mut self, index: usize, value: T) -> bool {
        match index {
            0 => self.0 = value,
            1 => self.1 = value,
            _ => {}
        }
        index == 1
    }
}

impl<T> GatherTarget<T> for (T, T, T)
where
    T: Default,
{
    fn start_gathering(_size_hint: usize) -> Self {
        (T::default(), T::default(), T::default())
    }

    fn gather_into(&mut self, index: usize, value: T) -> bool {
        match index {
            0 => self.0 = value,
            1 => self.1 = value,
            2 => self.2 = value,
            _ => {}
        }
        index == 2
    }
}

impl<T> GatherTarget<T> for (T, T, T, T)
where
    T: Default,
{
    fn start_gathering(_size_hint: usize) -> Self {
        (T::default(), T::default(), T::default(), T::default())
    }

    fn gather_into(&mut self, index: usize, value: T) -> bool {
        match index {
            0 => self.0 = value,
            1 => self.1 = value,
            2 => self.2 = value,
            3 => self.3 = value,
            _ => {}
        }
        index == 3
    }
}

impl<T> GatherTarget<T> for (T, T, T, T, T)
where
    T: Default,
{
    fn start_gathering(_size_hint: usize) -> Self {
        (
            T::default(),
            T::default(),
            T::default(),
            T::default(),
            T::default(),
        )
    }

    fn gather_into(&mut self, index: usize, value: T) -> bool {
        match index {
            0 => self.0 = value,
            1 => self.1 = value,
            2 => self.2 = value,
            3 => self.3 = value,
            4 => self.4 = value,
            _ => {}
        }
        index == 4
    }
}

impl<T> GatherTarget<T> for (T, T, T, T, T, T)
where
    T: Default,
{
    fn start_gathering(_size_hint: usize) -> Self {
        (
            T::default(),
            T::default(),
            T::default(),
            T::default(),
            T::default(),
            T::default(),
        )
    }

    fn gather_into(&mut self, index: usize, value: T) -> bool {
        match index {
            0 => self.0 = value,
            1 => self.1 = value,
            2 => self.2 = value,
            3 => self.3 = value,
            4 => self.4 = value,
            5 => self.5 = value,
            _ => {}
        }
        index == 5
    }
}

impl GatherTarget<()> for u32 {
    fn start_gathering(_size_hint: usize) -> Self {
        0
    }

    fn gather_into(&mut self, _index: usize, _value: ()) -> bool {
        *self += 1;
        false
    }
}
