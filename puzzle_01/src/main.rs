use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

use distance::ordered_distance;
use similarity::occurence_similarity;

pub mod distance;
pub mod similarity;

type Id = usize;

#[derive(Debug)]
enum Error {
    CouldNotOpenFile,
    ParseNumberError(String),
    TooFewNumbersError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CouldNotOpenFile => write!(f, "Could not open the file"),
            Error::ParseNumberError(num) => write!(f, "Could not parse the number: {}", num),
            Error::TooFewNumbersError => write!(f, "Too few numbers in line."),
        }
    }
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt").map_err(|_| Error::CouldNotOpenFile)?;
    let reader = BufReader::new(file);
    let mut vec_a: Vec<Id> = vec![];
    let mut vec_b: Vec<Id> = vec![];
    for line in reader.lines() {
        let line_content = line.map_err(|_| Error::CouldNotOpenFile)?.trim().to_owned();

        if line_content.is_empty() {
            continue;
        }

        let vec_line = line_content
            .split_whitespace()
            .map(|v| {
                v.parse::<Id>()
                    .map_err(|_| Error::ParseNumberError(v.to_string()))
            })
            .collect::<Result<Vec<Id>, _>>()?;
        vec_a.push(*vec_line.get(0).ok_or(Error::TooFewNumbersError)?);
        vec_b.push(*vec_line.get(1).ok_or(Error::TooFewNumbersError)?);
    }
    let distance = ordered_distance(&mut vec_a, &mut vec_b);
    println!("The total ordered distance is: {}", distance);

    let similarity_score = occurence_similarity(&mut vec_a, &mut vec_b);
    println!("The similarity score is: {}", similarity_score);
    Ok(())
}
