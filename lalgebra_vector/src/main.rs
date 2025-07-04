use lalgebra_vector::*;

fn main() {
	let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
	let vector_2: Vector<i64> = Vector(vec![4, -2, -1]);
	println!("{:?}", vector_1.dot(&vector_2));
	println!("{:?}", vector_1 + vector_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_product() {
        let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
        let vector_2: Vector<i64> = Vector(vec![4, -2, -1]);
        let expected: i64 = 3;
        assert_eq!(vector_1.dot(&vector_2), Some(expected));

        let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
        let vector_2: Vector<i64> = Vector(vec![4, -2]);
        assert_eq!(vector_1.dot(&vector_2), None);
    }
}