use matrix_mult::*;

fn main() {
    let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
    println!("{:?}", matrix.col(0));
    println!("{:?}", matrix.row(1));

    let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
    let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0], vec![1, 0]]);
    let mult = matrix_1.clone() * matrix_2.clone();
    println!("{:?}", mult);
    println!("{:?}", matrix_1.number_of_cols());
    println!("{:?}", matrix_2.number_of_rows());
}
