use std::fmt::Debug;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

pub trait Scalar:
    Copy + Add<Output = Self> + Mul<Output = Self> + Debug + Clone + PartialEq + Eq
{
}

impl<T> Scalar for T where
    T: Copy + Add<Output = Self> + Mul<Output = Self> + Debug + Clone + PartialEq + Eq
{
}

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Option<Self> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let result = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(a, b)| *a + *b)
            .collect();

        Some(Vector(result))
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut sum = None;
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            let product = *a * *b;
            sum = Some(match sum {
                Some(s) => s + product,
                None => product,
            });
        }

        sum
    }
}
