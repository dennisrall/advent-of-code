use crate::direction::Direction;
use char_grid::CharGrid;
use std::fs::read_to_string;
use vector::Vector2D;

mod char_grid;
mod direction;
mod vector;

pub fn count_str(s: &str, grid: &CharGrid) -> usize {
    let length = s.len();

    grid.iter_indices()
        .flat_map(|idx| {
            Direction::variants()
                .iter()
                .filter_map(|dir| grid.get_vector_from_direction(&idx, &dir, &length))
                .collect::<Vec<Vec<char>>>()
        })
        .filter(|c| c.iter().collect::<String>() == s)
        .count()
}

fn check_diag(
    idx: &Vector2D,
    grid: &CharGrid,
    direction: &Direction,
    opposite: &Direction,
    s: &str,
) -> bool {
    let len_s = s.chars().count();

    let start = *idx + opposite.get_vector();
    if start.is_some() {
        let chars = grid.get_vector_from_direction(&start.unwrap(), direction, &len_s);
        if chars.is_some() {
            if chars.unwrap().iter().collect::<String>() == s {
                return true;
            }
        }
    }
    false
}

pub fn count_x_shape(s: &str, grid: &CharGrid) -> usize {
    // for strings of size 3
    let middle = s.chars().nth(1).unwrap();

    let mut count = 0;
    for idx in grid.iter_indices() {
        let c = grid.get(idx);
        if c.is_none() || *c.unwrap() != middle {
            continue;
        }

        let diag1 = check_diag(
            &idx,
            grid,
            &Direction::UPFORWARD,
            &Direction::DOWNBACKWARD,
            s,
        ) || check_diag(
            &idx,
            grid,
            &Direction::DOWNBACKWARD,
            &Direction::UPFORWARD,
            s,
        );
        if diag1 {
            let diag2 = check_diag(
                &idx,
                grid,
                &Direction::UPBACKWARD,
                &Direction::DOWNFORWARD,
                s,
            ) || check_diag(
                &idx,
                grid,
                &Direction::DOWNFORWARD,
                &Direction::UPBACKWARD,
                s,
            );
            if diag2 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let content = read_to_string("input.txt").unwrap();
    let grid = CharGrid::from_string(&content).unwrap();
    let count_xmas = count_str("XMAS", &grid);
    println!("Count XMAS: {}", count_xmas);

    let count_x_mas = count_x_shape("MAS", &grid);
    println!("Count X-MAS: {}", count_x_mas);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let grid = CharGrid::from_string(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        )
        .unwrap();
        let result = count_str("XMAS", &grid);
        assert_eq!(result, 18);
    }

    #[test]
    fn example_2() {
        let grid = CharGrid::from_string(
            "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX",
        )
        .unwrap();
        let result = count_str("XMAS", &grid);
        assert_eq!(result, 18);
    }
    #[test]
    fn example_3() {
        let grid = CharGrid::from_string("XMAS\nXMAS\nXMAS").unwrap();
        let result = count_str("XMAS", &grid);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_4() {
        let grid = CharGrid::from_string(
            "..X...
.SAMX.
.A..A.
XMAS.S
.X....",
        )
        .unwrap();
        let result = count_str("XMAS", &grid);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_upward() {
        let grid = CharGrid::from_string("S\nA\nM\nX").unwrap();
        let result = count_str("XMAS", &grid);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_forward() {
        let grid = CharGrid::from_string("XMAS").unwrap();
        let result = count_str("XMAS", &grid);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_downward() {
        let grid = CharGrid::from_string("X\nM\nA\nS").unwrap();
        let result = count_str("XMAS", &grid);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_x_mas() {
        let grid = CharGrid::from_string(
    ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
").unwrap();
        let result = count_x_shape("MAS", &grid);
        assert_eq!(result, 9);
    }

}
