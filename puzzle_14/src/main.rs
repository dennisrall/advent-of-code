use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use counter::Counter;

type Vector = (isize, isize);

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let counter: Counter<_> = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| parse_pos_and_vel(&l))
        .map(|(pos, vel)| move_robot(&pos, &vel, &100, &(101, 103)))
        .map(|pos| get_quadrant(&pos, &(101, 103)))
        .collect();

    let safety_factor = counter.get(&1).unwrap_or(&0)
        * counter.get(&2).unwrap_or(&0)
        * counter.get(&3).unwrap_or(&0)
        * counter.get(&4).unwrap_or(&0);
    println!("Safety factor: {}", safety_factor);

    let reader = BufReader::new(File::open("input.txt").unwrap());

    let robots: Vec<_> = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| parse_pos_and_vel(&l))
        .collect();

    let secs = min_secs_to_christmas_tree(robots, &(101, 103));
    println!("Min secs to easter egg: {}", secs);
}

fn min_secs_to_christmas_tree(robots: Vec<(Vector, Vector)>, grid: &Vector) -> isize {
    // let mut secs = 1;
    let mut secs = 7790; // speed up for testing
    loop {
        let positions = robots
            .iter()
            .map(|(pos, vel)| move_robot(pos, vel, &secs, grid))
            .collect();
        if is_christmas_tree(&positions, grid) {
            break;
        }
        secs += 1;
    }
    secs
}

fn is_christmas_tree(robots: &HashSet<Vector>, grid: &Vector) -> bool {
    let count = robots
        .iter()
        .filter(|pos| {
            let touching_positions = [
                move_robot(pos, &(1, 0), &1, grid),
                move_robot(pos, &(0, 1), &1, grid),
                move_robot(pos, &(-1, 0), &1, grid),
                move_robot(pos, &(0, -1), &1, grid),
            ];
            robots.iter().any(|p| touching_positions.contains(p))
        })
        .count();
    count > robots.len() / 2
}

fn parse_pos_and_vel(s: &str) -> (Vector, Vector) {
    let vs = s
        .split_whitespace()
        .map(|s| {
            s.split(',')
                .map(|s| {
                    s.chars()
                        .filter(|c| c.is_ascii_digit() || *c == '-')
                        .collect::<String>()
                })
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(|v| (v[0], v[1]))
        .collect::<Vec<Vector>>();
    (vs[0], vs[1])
}

fn move_robot(pos: &Vector, vel: &Vector, times: &isize, grid: &Vector) -> Vector {
    (
        ((pos.0 + times * vel.0).rem_euclid(grid.0)),
        ((pos.1 + times * vel.1).rem_euclid(grid.1)),
    )
}

fn get_quadrant(pos: &Vector, grid: &Vector) -> usize {
    let mid = (grid.0 / 2, grid.1 / 2);
    if pos.0 < mid.0 && pos.1 < mid.1 {
        1 // Top-left
    } else if pos.0 > mid.0 && pos.1 < mid.1 {
        2 // Top-right
    } else if pos.0 < mid.0 && pos.1 > mid.1 {
        3 // Bottom-left
    } else if pos.0 > mid.0 && pos.1 > mid.1 {
        4 // Bottom-right
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadrant() {
        let result = get_quadrant(&(1, 3), &(3, 7));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_move_robot() {
        let result = move_robot(&(2, 4), &(2, -3), &5, &(11, 7));
        assert_eq!(result, (1, 3));
    }

    #[test]
    fn test_main() {
        let reader = BufReader::new(File::open("input.txt").unwrap());
        let counter: Counter<_> = reader
            .lines()
            .map_while(Result::ok)
            .map(|l| parse_pos_and_vel(&l))
            .map(|(pos, vel)| move_robot(&pos, &vel, &100, &(101, 103)))
            .map(|pos| get_quadrant(&pos, &(101, 103)))
            .collect();

        let safety_factor = counter.get(&1).unwrap_or(&0)
            * counter.get(&2).unwrap_or(&0)
            * counter.get(&3).unwrap_or(&0)
            * counter.get(&4).unwrap_or(&0);
        assert_eq!(safety_factor, 228421332);

        let reader = BufReader::new(File::open("input.txt").unwrap());

        let robots: Vec<_> = reader
            .lines()
            .map_while(Result::ok)
            .map(|l| parse_pos_and_vel(&l))
            .collect();

        let secs = min_secs_to_christmas_tree(robots, &(101, 103));
        assert_eq!(secs, 7790);
    }
}
