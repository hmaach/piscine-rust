use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
    pub i: T,
    pub j: T,
    pub k: T,
}

impl<T> Add for ThreeDVector<T>
where
    T: Add<Output = T>,
{
    type Output = ThreeDVector<T>;

    fn add(self, other: ThreeDVector<T>) -> ThreeDVector<T> {
        ThreeDVector {
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}

impl<T> Sub for ThreeDVector<T>
where
    T: Sub<Output = T>,
{
    type Output = ThreeDVector<T>;

    fn sub(self, other: ThreeDVector<T>) -> ThreeDVector<T> {
        ThreeDVector {
            i: self.i - other.i,
            j: self.j - other.j,
            k: self.k - other.k,
        }
    }
}
