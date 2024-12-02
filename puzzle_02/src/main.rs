use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use level_checker::{check_level, check_level_skip, LevelItem};

pub mod level_checker;

fn main() -> io::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);

    let (count_safe, count_safe_skip) = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<LevelItem>().unwrap())
                .collect::<Vec<_>>()
        })
        .fold((0, 0), |(safe, safe_skip), line_vec| {
            (
                safe + check_level(&line_vec, false) as usize,
                safe_skip + check_level_skip(&line_vec) as usize,
            )
        });

    println!("Amount of safe reports: {}", count_safe);
    println!("Amount of safe reports with skipping: {}", count_safe_skip);

    Ok(())
}
