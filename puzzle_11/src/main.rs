use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use cached::proc_macro::cached;

fn blink_stone(v: usize) -> Vec<usize> {
    if v == 0 {
        vec![1]
    } else if v.ilog10() as usize % 2 == 1 {
        let len = v.ilog10() as usize + 1;
        let divisor = 10_usize.pow((len / 2) as u32);
        let left = v / divisor;
        let right = v % divisor;
        vec![left, right]
    } else {
        vec![v * 2024]
    }
}

#[cached]
fn count_stones_after_blinks(vec: Vec<usize>, n: usize) -> usize {
    if n == 0 {
        vec.len()
    } else {
        vec.iter()
            .map(|v| count_stones_after_blinks(blink_stone(*v), n - 1))
            .sum()
    }
}

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let vec: Vec<usize> = reader
        .lines()
        .next()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect()
        })
        .unwrap();
    let num_stones_25 = count_stones_after_blinks(vec.clone(), 25);
    println!("Number of stones after 25 blinks: {}", num_stones_25);
    let num_stones_75 = count_stones_after_blinks(vec, 75);
    println!("Number of stones after 75 blinks: {}", num_stones_75);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_blink() {
        let result = count_stones_after_blinks(vec![125, 17], 1);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_blink_stone() {
        let stone = blink_stone(125);
        assert_eq!(stone, vec![253000]);
    }

    #[test]
    fn test_blink_stone_2() {
        let stone = blink_stone(17);
        assert_eq!(stone, vec![1, 7]);
    }

    #[test]
    fn test_main() {
        let reader = BufReader::new(File::open("input.txt").unwrap());
        let vec: Vec<usize> = reader
            .lines()
            .next()
            .map(|l| {
                l.unwrap()
                    .split_whitespace()
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect()
            })
            .unwrap();
        let num_stones_25 = count_stones_after_blinks(vec.clone(), 25);
        assert_eq!(num_stones_25, 216996);
        let num_stones_75 = count_stones_after_blinks(vec, 75);
        assert_eq!(num_stones_75, 257335372288947);
    }
}
