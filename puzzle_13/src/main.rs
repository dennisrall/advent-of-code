use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

type Vector = (isize, isize);

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut it = reader.lines().map_while(Result::ok);
    let mut sum_tokens = 0;
    let mut sum_tokens_2 = 0;
    while let Some(l) = it.next() {
        let move_a = get_vector_from_input(&l);
        let move_b = get_vector_from_input(&it.next().unwrap());
        let goal_vec = get_vector_from_input(&it.next().unwrap());
        let goal_vec_2 = (goal_vec.0 + 10000000000000, goal_vec.1 + 10000000000000);
        it.next(); // skip empty line
        sum_tokens += fewest_tokens_to_win_lin(move_a, move_b, goal_vec).unwrap_or(0);
        sum_tokens_2 += fewest_tokens_to_win_lin(move_a, move_b, goal_vec_2).unwrap_or(0);
    }
    println!("Sum tokens needed: {}", sum_tokens);
    println!("Sum tokens needed 2: {}", sum_tokens_2);
}

fn get_vector_from_input(s: &str) -> Vector {
    let re = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();
    let v = re
        .captures_iter(s)
        .map(|caps| {
            let x = caps[1].parse::<isize>().unwrap();
            let y = caps[2].parse::<isize>().unwrap();
            (x, y)
        })
        .next()
        .unwrap();
    v
}

fn fewest_tokens_to_win_lin(a: Vector, b: Vector, v: Vector) -> Option<isize> {
    // v = na + mb
    let m = (v.1 * a.0 - v.0 * a.1) / (b.1 * a.0 - b.0 * a.1);
    let n = (v.0 - m * b.0) / a.0;

    if (a.0 * n + b.0 * m, a.1 * n + b.1 * m) == v {
        Some(3 * n + m)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let result = fewest_tokens_to_win_lin((94, 34), (22, 67), (8400, 5400)).unwrap();
        assert_eq!(result, 280);
    }

    #[test]
    fn test_main() {
        let reader = BufReader::new(File::open("input.txt").unwrap());
        let mut it = reader.lines().map_while(Result::ok);
        let mut sum_tokens = 0;
        let mut sum_tokens_2 = 0;
        while let Some(l) = it.next() {
            let move_a = get_vector_from_input(&l);
            let move_b = get_vector_from_input(&it.next().unwrap());
            let goal_vec = get_vector_from_input(&it.next().unwrap());
            let goal_vec_2 = (goal_vec.0 + 10000000000000, goal_vec.1 + 10000000000000);
            it.next(); // skip empty line
            sum_tokens += fewest_tokens_to_win_lin(move_a, move_b, goal_vec).unwrap_or(0);
            sum_tokens_2 += fewest_tokens_to_win_lin(move_a, move_b, goal_vec_2).unwrap_or(0);
        }
        assert_eq!(sum_tokens, 29877);
        assert_eq!(sum_tokens_2, 99423413811305);
    }
}
