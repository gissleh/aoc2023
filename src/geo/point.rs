use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::simd::{Simd, SimdElement};
use num::One;

pub struct Point<T> (Simd<T, 2>) where T: SimdElement;

impl<T> Point<T> where T: SimdElement {
    #[inline(always)]
    pub fn new(x: T, y: T) -> Self {
        Self(Simd::from_array([x, y]))
    }

    #[inline(always)]
    pub fn coords(&self) -> &[T; 2] {
        self.0.as_array()
    }

    #[inline(always)]
    pub fn coords_mut(&mut self) -> &mut [T; 2] {
        self.0.as_mut_array()
    }
}

impl<T> Point<T> where T: SimdElement, T: Copy + Add<Output=T> + Sub<Output=T> {
    #[inline]
    pub fn cardinals_offset(&self, offset: T) -> [Point<T>; 4] {
        let [x, y] = *self.coords();

        [
            Point::new(x, y - offset),
            Point::new(x - offset, y),
            Point::new(x + offset, y),
            Point::new(x, y + offset),
        ]
    }
}

impl<T> Point<T> where T: SimdElement, T: Copy + Add<Output=T> + Sub<Output=T> + One {
    #[inline]
    pub fn cardinals(&self) -> [Point<T>; 4] {
        self.cardinals_offset(T::one())
    }
}

impl<T> Hash for Point<T> where T: SimdElement, Simd<T, 2>: Hash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T> From<(T, T)> for Point<T> where T: SimdElement {
    fn from((x, y): (T, T)) -> Self {
        Self(Simd::from_array([x, y]))
    }
}

impl<T> Copy for Point<T> where T: SimdElement, Simd<T, 2>: Copy {}

impl<T> Clone for Point<T> where T: SimdElement, Simd<T, 2>: Clone {
    #[inline(always)]
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<T> const Default for Point<T> where T: SimdElement, Simd<T, 2>: ~const Default {
    #[inline(always)]
    fn default() -> Self { Self(Default::default()) }
}

impl<T> Eq for Point<T> where T: SimdElement, Simd<T, 2>: Eq {}

impl<T> PartialEq for Point<T> where T: SimdElement, Simd<T, 2>: PartialEq {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}

impl<T> Add for Point<T> where T: SimdElement, Simd<T, 2>: Add<Output=Simd<T, 2>> {
    type Output = Point<T>;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0) }
}

impl<T> AddAssign for Point<T> where T: SimdElement, Simd<T, 2>: AddAssign {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0 }
}

impl<T> Sub for Point<T> where T: SimdElement, Simd<T, 2>: Sub<Output=Simd<T, 2>> {
    type Output = Point<T>;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0) }
}

impl<T> SubAssign for Point<T> where T: SimdElement, Simd<T, 2>: SubAssign {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0 }
}

impl<T> Mul for Point<T> where T: SimdElement, Simd<T, 2>: Mul<Output=Simd<T, 2>> {
    type Output = Point<T>;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output { Self(self.0 * rhs.0) }
}

impl<T> MulAssign for Point<T> where T: SimdElement, Simd<T, 2>: MulAssign {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) { self.0 *= rhs.0 }
}

impl<T> Div for Point<T> where T: SimdElement, Simd<T, 2>: Div<Output=Simd<T, 2>> {
    type Output = Point<T>;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output { Self(self.0 / rhs.0) }
}

impl<T> DivAssign for Point<T> where T: SimdElement, Simd<T, 2>: DivAssign {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) { self.0 /= rhs.0 }
}

impl<T> Debug for Point<T> where T: SimdElement + Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let coords = self.coords();
        f.debug_tuple("Point")
            .field(&coords[0])
            .field(&coords[1])
            .finish()
    }
}

impl<T> Display for Point<T> where T: SimdElement + Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let coords = self.coords();
        write!(f, "<{}, {}>", coords[0], coords[1])
    }
}
