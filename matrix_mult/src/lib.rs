use lalgebra_scalar::Scalar;
use std::iter::Sum;
use std::ops::Mul;

#[derive(Debug, PartialEq, Clone)]
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

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut column = Vec::new();
        for row in &self.0 {
            for (i, v) in row.iter().enumerate() {
                if i == n {
                    column.push(v.clone());
                }
            }
        }
        column
    }
}

impl<T: Scalar<Item = T> + Sum<<T as Mul>::Output>> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        let row_lenght = self.number_of_rows();
        let col_lenght = rhs.number_of_cols();
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }
        let mut result: Matrix<T> = Matrix::zero(row_lenght, col_lenght);
        for j in 0..result.number_of_rows() {
            for i in 0..result.number_of_cols() {
                result.0[j][i] = self
                    .row(j)
                    .iter()
                    .zip(rhs.col(i).iter())
                    .map(|(x, y)| x.clone() * y.clone())
                    .sum();
            }
        }
        Some(result)
    }
}
