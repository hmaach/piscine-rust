use matrix::*;

fn main() {
    let m: Matrix<u32> = Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
    println!("{:?}", m);
    println!("{:?}", Matrix::<i32>::identity(4));
    println!("{:?}", Matrix::<f64>::zero(3, 4));
}
