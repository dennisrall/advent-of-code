use core::panic;
use std::collections::HashSet;
use std::fs::read_to_string;

use puzzle_04::char_grid::CharGrid;
use puzzle_04::direction::Direction;
use puzzle_04::vector::BoundVector2D;

fn direction_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Forward,
        Direction::Forward => Direction::Down,
        Direction::Down => Direction::Backward,
        Direction::Backward => Direction::Up,
        _ => panic!("No valid direction"),
    }
}

fn get_direction(c: &char) -> Option<Direction> {
    match c {
        'v' => Some(Direction::Down),
        '>' => Some(Direction::Forward),
        '<' => Some(Direction::Backward),
        '^' => Some(Direction::Up),
        _ => None,
    }
}

fn find_start(grid: &CharGrid) -> Option<BoundVector2D> {
    grid.iter_indices().find(|idx| {
        matches!(
            grid.get(*idx),
            Some('v') | Some('>') | Some('<') | Some('^')
        )
    })
}

fn get_visited_positions(grid: &CharGrid) -> Option<HashSet<BoundVector2D>> {
    let mut cur_pos = find_start(grid)?;
    let mut direction = get_direction(grid.get(cur_pos)?)?;
    let mut positions = HashSet::new();

    loop {
        positions.insert(cur_pos);
        let next_pos = cur_pos + direction.get_vector();

        if next_pos.is_none() {
            return Some(positions);
        }

        let next_item = grid.get(next_pos.unwrap())?;
        if *next_item == '#' {
            direction = direction_right(&direction);
        } else {
            cur_pos = next_pos.unwrap();
        }
    }
}

fn find_barriers(grid: &CharGrid) -> HashSet<BoundVector2D> {
    grid.iter_indices()
        .filter(|&idx| matches!(grid.get(idx), Some('#')))
        .collect()
}

fn get_to_check(
    grid: &CharGrid,
    visited_positions: HashSet<BoundVector2D>,
) -> Option<HashSet<BoundVector2D>> {
    let cur_pos = find_start(grid)?;
    let barriers = find_barriers(grid);
    let x_vals: HashSet<usize> = barriers.iter().map(|v| v.x).collect();
    let y_vals: HashSet<usize> = barriers.iter().map(|v| v.y).collect();

    visited_positions
        .into_iter()
        .filter(|idx| x_vals.contains(&idx.x) || y_vals.contains(&idx.y))
        .filter(|idx| !barriers.contains(idx))
        .filter(|idx| idx != &cur_pos)
        .map(Some)
        .collect()
}

fn is_loop(grid: &CharGrid, &barrier: &BoundVector2D) -> bool {
    let mut cur_pos = find_start(grid).unwrap();
    let mut direction = get_direction(grid.get(cur_pos).unwrap()).unwrap();
    let mut loc_seen = HashSet::new();
    loop {
        if !loc_seen.insert((cur_pos, direction.clone())) {
            return true;
        }
        let next_pos = cur_pos + direction.get_vector();

        if next_pos.is_none()
            || next_pos.unwrap().x >= grid.rows
            || next_pos.unwrap().y >= grid.cols
        {
            return false;
        }

        let next_item = grid.get(next_pos.unwrap()).unwrap();
        if *next_item == '#' || next_pos.unwrap() == barrier {
            direction = direction_right(&direction);
        } else {
            cur_pos = next_pos.unwrap();
        }
    }
}

fn count_loops(grid: &CharGrid, visited_positions: HashSet<BoundVector2D>) -> usize {
    get_to_check(grid, visited_positions)
        .unwrap()
        .iter()
        .filter(|idx| is_loop(grid, idx))
        .count()
}

fn main() {
    let content = read_to_string("input.txt").unwrap();
    let grid = CharGrid::from_string(&content).unwrap();
    let visited_positions = get_visited_positions(&grid).unwrap();
    println!("Count positions: {}", &visited_positions.len());
    let count_loops = count_loops(&grid, visited_positions);
    println!("Count loops: {}", count_loops);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let grid = CharGrid::from_string(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        )
        .unwrap();

        let result = get_visited_positions(&grid).unwrap();

        assert_eq!(result.len(), 41);
    }
    #[test]
    fn test_example_1() {
        let grid = CharGrid::from_string(">..#").unwrap();
        let result = get_visited_positions(&grid).unwrap();
        assert_eq!(result.len(), 3);
    }
    #[test]
    fn test_example_loop() {
        let grid = CharGrid::from_string(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        )
        .unwrap();

        let result = count_loops(&grid, grid.iter_indices().collect());

        assert_eq!(result, 6);
    }
    #[test]
    fn test_example_loop_1() {
        let grid = CharGrid::from_string(
            ".#..
...#
.^#.",
        )
        .unwrap();

        let result = count_loops(&grid, grid.iter_indices().collect());

        assert_eq!(result, 1);
    }
    #[test]
    fn test_example_loop_2() {
        let grid = CharGrid::from_string(
            ".#.
.^#
.#.",
        )
        .unwrap();

        let result = count_loops(&grid, grid.iter_indices().collect());

        assert_eq!(result, 1);
    }

    #[test]
    fn test_main() {
        let content = read_to_string("input.txt").unwrap();
        let grid = CharGrid::from_string(&content).unwrap();
        let visited_positions = get_visited_positions(&grid).unwrap();
        assert_eq!(visited_positions.len(), 5564);
        let count_loops = count_loops(&grid, visited_positions);
        assert_eq!(count_loops, 1976);
    }
}
