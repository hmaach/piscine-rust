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