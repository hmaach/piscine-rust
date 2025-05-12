pub fn edit_distance(source: &str, target: &str) -> usize {
    let mut ops: usize = 0;
    let source_len: usize = source.to_string().chars().count();
    let target_len: usize = target.to_string().chars().count();
    if source_len > target_len {
        ops += source_len - target_len;
    } else {
        ops += target_len - source_len;
    }
    for (i, ch) in target.to_string().chars().enumerate() {
        if i + 1 > source_len {
            break;
        }
        if ch != source.chars().nth(i).unwrap() {
            ops += 1;
        }
    }
    ops
}
