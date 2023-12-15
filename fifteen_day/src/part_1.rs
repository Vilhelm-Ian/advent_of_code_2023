use std::collections::HashMap;

pub fn hash_algorithm(string: &str) -> i32 {
    string.chars().fold(0, |mut current_value, letter| {
        current_value += letter as i32;
        current_value *= 17;
        current_value %= 256;
        current_value
    })
}

pub fn sum_sequence(string: &str) -> i32 {
    string
        .split(',')
        .fold((HashMap::new(), 0), |(mut cache, result), step| {
            let value = match cache.get(&step) {
                Some(value) => *value,
                None => hash_algorithm(step),
            };
            cache.insert(step, value);
            (cache, result + value)
        })
        .1
}
