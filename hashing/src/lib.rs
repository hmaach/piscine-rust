use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let mut hash_map = HashMap::new();
    let mut res: f64 = 0.0;

    for nb in list {
        *hash_map.entry(nb).or_insert(0) += 1;
    }

    for (key, value) in hash_map.into_iter() {
        res += (key * value) as f64;
    }

    res / (list.len() as f64)
}
pub fn median(list: &[i32]) -> i32 {
    let mut new_vec = list.to_vec();
    new_vec.sort();
    let n = new_vec.len();
    if n % 2 == 0 {
        return (new_vec[n / 2 - 1] + new_vec[n / 2]) / 2;
    }
    new_vec[n / 2]
}

pub fn mode(list: &[i32]) -> i32 {
    let mut hash_map = HashMap::new();
    let mut res = 0;

    for nb in list {
        *hash_map.entry(nb).or_insert(0) += 1;
    }

    let mut highest = 0;
    for (key, value) in hash_map {
        if value >= highest {
            highest = value;
            res = *key;
        }
    }
    res
}
