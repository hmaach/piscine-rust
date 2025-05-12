// pub fn edit_distance(source: &str, target: &str) -> usize {
//     let mut ops: usize = 0;
//     let source_len: usize = source.to_string().chars().count();
//     let target_len: usize = target.to_string().chars().count();
//     if source_len > target_len {
//         ops += source_len - target_len;
//     } else {
//         ops += target_len - source_len;
//     }
//     for (i, ch) in target.to_string().chars().enumerate() {
//         if i + 1 > source_len {
//             break;
//         }
//         if ch != source.chars().nth(i).unwrap() {
//             ops += 1;
//         }
//     }
//     ops
// }

pub fn edit_distance(source: &str, target: &str) -> usize {
    let mut ops: usize = 0;

    if source == target {
        return ops;
    }

    let source_len = source.chars().count();
    let target_len = target.chars().count();

    if source_len == 0 {
        return target_len;
    }

    if target_len == 0 {
        return source_len;
    }

    // this make a vector with the source length filled with range from 1 to len(source)
    let mut cache: Vec<usize> = (1..).take(source_len).collect();

    for (target_i, target_ch) in target.chars().enumerate() {
        ops = target_i;
        let mut source_distance = target_i;

        for (source_i, source_ch) in source.chars().enumerate() {
            let target_distance = if source_ch == target_ch {
                source_distance
            } else {
                source_distance + 1
            };

            source_distance = cache[source_i];

            ops = if source_distance > ops {
                if target_distance > ops {
                    ops + 1
                } else {
                    target_distance
                }
            } else if target_distance > source_distance {
                source_distance + 1
            } else {
                target_distance
            };

            cache[source_i] = ops;
        }
    }
    ops
}
