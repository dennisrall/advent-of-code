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
    let reader = BufReader::new(File::open("input.txt").unwrap());

    let (mut vec_a, mut vec_b): (Vec<Id>, Vec<Id>) = reader
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut nums = l.split_whitespace().map(|s| s.parse::<Id>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    let distance = ordered_distance(&mut vec_a, &mut vec_b);
    println!("The total ordered distance is: {}", distance);

    let similarity_score = occurence_similarity(&mut vec_a, &mut vec_b);
    println!("The similarity score is: {}", similarity_score);
}
