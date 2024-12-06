use core::panic;
use std::collections::HashSet;
use std::fs::read_to_string;

use puzzle_04::char_grid::CharGrid;
use puzzle_04::direction::Direction;
use puzzle_04::vector::Vector2D;

fn direction_right(direction: &Direction) -> Direction {
    match direction {
        Direction::UP => Direction::FORWARD,
        Direction::FORWARD => Direction::DOWN,
        Direction::DOWN => Direction::BACKWARD,
        Direction::BACKWARD => Direction::UP,
        _ => panic!("No valid direction"),
    }
}

fn get_direction(c: &char) -> Option<Direction> {
    match c {
        'v' => Some(Direction::DOWN),
        '>' => Some(Direction::FORWARD),
        '<' => Some(Direction::BACKWARD),
        '^' => Some(Direction::UP),
        _ => None,
    }
}

fn find_start(grid: &CharGrid) -> Option<Vector2D> {
    grid.iter_indices()
        .filter(|idx| match grid.get(*idx) {
            Some('v') => true,
            Some('>') => true,
            Some('<') => true,
            Some('^') => true,
            _ => false,
        })
        .next()
}

fn count_positions(grid: &CharGrid) -> Option<usize> {
    let mut cur_pos = find_start(grid)?;
    let mut direction = get_direction(grid.get(cur_pos)?)?;
    let mut positions: HashSet<Vector2D> = HashSet::new();

    loop {
        positions.insert(cur_pos);
        let mut next_pos = cur_pos + direction.get_vector();

        if next_pos.is_none()
            || next_pos.unwrap().x >= grid.rows
            || next_pos.unwrap().y >= grid.cols
        {
            break;
        }

        let next_item = grid.get(next_pos.unwrap())?;
        if *next_item == '#' {
            direction = direction_right(&direction);
            next_pos = cur_pos + direction.get_vector();
            if next_pos.is_none()
                || next_pos.unwrap().x >= grid.rows
                || next_pos.unwrap().y >= grid.cols
            {
                break;
            }
        }
        cur_pos = next_pos.unwrap();
    }
    Some(positions.len())
}

fn find_barriers(grid: &CharGrid) -> HashSet<Vector2D> {
    grid.iter_indices()
        .filter(|&idx| matches!(grid.get(idx), Some('#')))
        .collect()
}

fn get_to_check(grid: &CharGrid) -> Option<HashSet<Vector2D>> {
    let cur_pos = find_start(grid)?;
    let barriers = find_barriers(grid);
    let x_vals: HashSet<usize> = barriers.iter().map(|v| v.x).collect();
    let y_vals: HashSet<usize> = barriers.iter().map(|v| v.y).collect();

    grid.iter_indices()
        .filter(|idx| x_vals.contains(&idx.x) || y_vals.contains(&idx.y))
        .filter(|idx| !barriers.contains(&idx))
        .filter(|idx| idx != &cur_pos)
        .map(|idx| Some(idx))
        .collect()
}

fn is_loop(grid: &CharGrid, &barrier: &Vector2D) -> bool {
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

fn count_loops(grid: &CharGrid) -> usize {
    get_to_check(grid)
        .unwrap()
        .iter()
        .filter(|idx| is_loop(&grid, *idx))
        .count()
}

fn main() {
    let content = read_to_string("input.txt").unwrap();
    let grid = CharGrid::from_string(&content).unwrap();
    let count_pos = count_positions(&grid);
    println!("Count positions: {}", count_pos.unwrap());
    let count_loops = count_loops(&grid);
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

        let result = count_positions(&grid);

        assert_eq!(result, Some(41));
    }
    #[test]
    fn test_example_1() {
        let grid = CharGrid::from_string(">..#").unwrap();
        let result = count_positions(&grid);
        assert_eq!(result, Some(3));
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

        let result = count_loops(&grid);

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

        let result = count_loops(&grid);

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

        let result = count_loops(&grid);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_main() {
        let content = read_to_string("input.txt").unwrap();
        let grid = CharGrid::from_string(&content).unwrap();
        let count_pos = count_positions(&grid);
        assert_eq!(count_pos, Some(5564));
        let count_loops = count_loops(&grid);
        assert_eq!(count_loops, 1976);
    }
}
