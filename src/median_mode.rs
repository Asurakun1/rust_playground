use rand::Rng;
use std::vec;
use std::collections::HashMap;

fn main() {
    let vector = generate_array(11);
    let median = find_median(&vector);

    println!("Vector: {:?}", &vector);
    println!("Median: {}", median);
    println!("Mode: {}", find_mode(&vector));
}

fn find_mode(vector: &Vec<i32>) -> i32 {

    let mut map = HashMap::new();

    for item in vector{
        let count = map.entry(item).or_insert(0);
        *count += 1;
    }

    let mut mode = vector[0];
    let mut max_count = 0;

    for(&num, &count) in map.iter() {
        if count > max_count {
            mode = *num;
            max_count = count;
        }
    }
    mode
}

fn find_median(vector: &Vec<i32>) -> i32 {
    let half = vector.len() / 2;

    if vector.len() % 2 != 0 {
        return vector[half];
    }

    vector[half - 1]
}

fn generate_array(length: i32) -> Vec<i32> {
    let mut vector: Vec<i32> = vec![0; length.try_into().unwrap()];

    for element in vector.iter_mut() {
        let random = rand::thread_rng().gen_range(0..20);
        *element = random;
    }

    vector.sort();
    vector
}
