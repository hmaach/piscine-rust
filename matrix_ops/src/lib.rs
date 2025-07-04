use std::ops::{Add, Sub};

use lalgebra_scalar::Scalar;

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(Vec::new())
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = Matrix::zero(n, n);
        for i in 0..n {
            matrix.0[i][i] = T::one();
        }
        matrix
    }
}

impl<T: Scalar<Item = T> + Add<Output = T> + Sub<Output = T>> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        if self.0.is_empty() {
            return Some(Matrix(Vec::new()));
        }

        let rows = self.0.len();
        let cols = self.0[0].len();

        let mut result = vec![vec![T::zero(); cols]; rows];

        for i in 0..rows {
            if self.0[i].len() != rhs.0[i].len() {
                return None;
            }
            for j in 0..cols {
                result[i][j] = self.0[i][j].clone() + rhs.0[i][j].clone();
            }
        }

        Some(Matrix(result))
    }
}

impl<T: Scalar<Item = T> + Add<Output = T> + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        if self.0.is_empty() {
            return Some(Matrix(Vec::new()));
        }

        let rows = self.0.len();
        let cols = self.0[0].len();

        let mut result = vec![vec![T::zero(); cols]; rows];

        for i in 0..rows {
            if self.0[i].len() != rhs.0[i].len() {
                return None;
            }
            for j in 0..cols {
                result[i][j] = self.0[i][j].clone() - rhs.0[i][j].clone();
            }
        }

        Some(Matrix(result))
    }
}