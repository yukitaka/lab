use std::collections::HashMap;

pub fn measures_of_central_tendency() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    let mean = match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    };

    println!("Mean of the data is {:?}", mean);
}

pub fn using_hashmap() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];
    let frequencies = data.iter().fold(HashMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });
    println!("HashMaps is {:?}", frequencies);
    let mode = frequencies
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| *value);

    println!("Mode of the data is {:?}", mode);
}
