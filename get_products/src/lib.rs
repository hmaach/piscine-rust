pub fn get_products(nums: Vec<usize>) -> Vec<usize> {
    let mut result = vec![1; nums.len()];

    let mut prod = 1;
    for (i, &num) in nums.iter().enumerate() {
        result[i] = prod;
        prod *= num;
    }

    prod = 1;
    for (i, &num) in nums.iter().enumerate().rev() {
        result[i] *= prod;
        prod *= num;
    }

    result
}
