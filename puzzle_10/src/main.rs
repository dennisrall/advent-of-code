use std::collections::HashSet;
use std::fs::read_to_string;

use puzzle_04::char_grid::CharGrid;
use puzzle_04::vector::BoundVector2D;

fn sum_trailhead_end_scores(grid: &CharGrid) -> usize {
    grid.iter_indices()
        .map(|idx| get_trailhead_ends(grid, Some(idx), 0))
        .map(|s| s.len())
        .sum()
}

fn get_trailhead_ends(
    grid: &CharGrid,
    index: Option<BoundVector2D>,
    next_val: usize,
) -> HashSet<BoundVector2D> {
    if !index.is_some_and(|idx| {
        grid.get(idx)
            .and_then(|c| c.to_string().parse::<usize>().ok())
            .is_some_and(|cur_val| cur_val == next_val)
    }) {
        HashSet::new()
    } else if next_val == 9 {
        HashSet::from([index.unwrap()])
    } else {
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .iter()
            .flat_map(|&dir| get_trailhead_ends(grid, index.unwrap() + dir, next_val + 1))
            .collect()
    }
}

fn count_unique_trailheads(grid: &CharGrid) -> usize {
    grid.iter_indices()
        .map(|idx| get_unique_trailhead_paths(grid, Some(idx), 0))
        .sum()
}

fn get_unique_trailhead_paths(
    grid: &CharGrid,
    index: Option<BoundVector2D>,
    next_val: usize,
) -> usize {
    if !index.is_some_and(|idx| {
        grid.get(idx)
            .and_then(|c| c.to_string().parse::<usize>().ok())
            .is_some_and(|cur_val| cur_val == next_val)
    }) {
        0
    } else if next_val == 9 {
        1
    } else {
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .iter()
            .map(|&dir| get_unique_trailhead_paths(grid, index.unwrap() + dir, next_val + 1))
            .sum()
    }
}

fn main() {
    let content = read_to_string("input.txt").unwrap();
    let grid = CharGrid::from_string(&content).unwrap();
    let sum_scores = sum_trailhead_end_scores(&grid);
    println!("Sum of all trainhead scores: {}", sum_scores);
    let count_trailheads = count_unique_trailheads(&grid);
    println!("Count unique trailheads: {}", count_trailheads);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let grid = CharGrid::from_string(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
",
        )
        .unwrap();
        let result = sum_trailhead_end_scores(&grid);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_example_1() {
        let grid = CharGrid::from_string("0123456789").unwrap();
        let result = sum_trailhead_end_scores(&grid);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_example_2() {
        let grid = CharGrid::from_string(
            "0123456789
0000000090",
        )
        .unwrap();
        let result = sum_trailhead_end_scores(&grid);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_example_3() {
        let grid = CharGrid::from_string(
            "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01",
        )
        .unwrap();
        let result = sum_trailhead_end_scores(&grid);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_example_4() {
        let grid = CharGrid::from_string(
            "0123
1234
8765
9876",
        )
        .unwrap();
        let result = sum_trailhead_end_scores(&grid);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_main() {
        let content = read_to_string("input.txt").unwrap();
        let grid = CharGrid::from_string(&content).unwrap();
        let sum_scores = sum_trailhead_end_scores(&grid);
        assert_eq!(sum_scores, 652);
        let count_trailheads = count_unique_trailheads(&grid);
        assert_eq!(count_trailheads, 1432);
    }
}
