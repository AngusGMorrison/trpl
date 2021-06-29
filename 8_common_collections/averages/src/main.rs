// Given a list of integers, use a vector and return the mean (the average
// value), median (when sorted, the value in the middle position), and mode (the
// value that occurs most often; a hash map will be helpful here) of the list.

// Note: these solutions don't account for scenarios where the input vector is
// empty. This may be best handled with a Result. I'll keep reading to find out.

use std::{collections::HashMap, i32};

fn main() {
    let ints = vec![10, 20, 30, 40, 50, 50];
    println!("Mean: {}", mean(&ints));
    println!("Median: {}", median(&ints));
    println!("Mode: {}", mode(&ints));
}

fn mean(v: &Vec<i32>) -> f64 {
    let total: i32 = v.iter().sum();
    total as f64 / v.len() as f64
}

fn median(v: &Vec<i32>) -> f64 {
    let mut copy = v.to_vec();
    copy.sort_unstable();

    let mid = copy.len() / 2;
    if copy.len() % 2 == 0 {
        let first = copy[mid - 1] as f64;
        let second = copy[mid] as f64;
        (first + second) / 2f64
    } else {
        copy[mid] as f64
    }
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for i in v {
        *map.entry(i).or_insert(0) += 1;
    }

    let mut elem = i32::MAX;
    let mut count = 0;
    for (key, val) in map {
        if val > count {
            elem = *key;
            count = val;
        }
    }

    elem
}
