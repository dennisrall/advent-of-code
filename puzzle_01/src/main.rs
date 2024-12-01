use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use distance::ordered_distance;
use similarity::occurence_similarity;

pub mod distance;
pub mod similarity;

type Id = usize;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut vec_a: Vec<Id> = vec![];
    let mut vec_b: Vec<Id> = vec![];
    for line in reader.lines() {
        let vec_line: Vec<Id> = line.unwrap().split_whitespace().map(|v| v.parse().unwrap()).collect();
        vec_a.push(*vec_line.first().unwrap());
        vec_b.push(*vec_line.get(1).unwrap());
    }
    let distance = ordered_distance(& mut vec_a, & mut vec_b);
    println!("The total ordered distance is: {}", distance);

    let similarity_score = occurence_similarity(&mut vec_a, &mut vec_b);
    println!("The similarity score is: {}", similarity_score);
}
