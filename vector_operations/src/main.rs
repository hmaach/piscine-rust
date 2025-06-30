use vector_operations::ThreeDVector;

fn main() {
	let a = ThreeDVector { i: 3, j: 5, k: 2 };
	let b = ThreeDVector { i: 2, j: 7, k: 4 };
	println!("{:?}", a + b);
}

