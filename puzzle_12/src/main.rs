use std::collections::HashSet;
use std::fs::read_to_string;

use puzzle_04::char_grid::CharGrid;
use puzzle_04::vector::BoundVector2D;

fn main() {
    let content = read_to_string("input.txt").unwrap();
    let grid = CharGrid::from_string(&content).unwrap();
    let fence_price = calculate_fence_price(&grid);
    println!("Price for fence around gardens: {}", fence_price);
    let fence_price_2 = calculate_fence_price_2(&grid);
    println!(
        "Price for fence around gardens with discount: {}",
        fence_price_2
    );
}

fn calculate_fence_price(grid: &CharGrid) -> usize {
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut indices_to_check: Vec<BoundVector2D> = Vec::new();
    let mut processed_indices: HashSet<BoundVector2D> = HashSet::new();
    let mut indices_same_group: Vec<BoundVector2D> = Vec::new();

    let mut score = 0;
    let mut perimeter = 0;
    let mut size = 0;

    let mut idx = BoundVector2D::new(0, 0, grid.rows, grid.cols).unwrap();
    loop {
        processed_indices.insert(idx);
        size += 1;

        let item = grid.get(idx).unwrap();
        for dir in directions {
            let neighbour_idx = idx + dir;
            let c = neighbour_idx.and_then(|i| grid.get(i));
            if c.is_none() {
                // edge
                perimeter += 1;
            } else if *c.unwrap() != *item {
                // border to other
                perimeter += 1;
                if !processed_indices.contains(&neighbour_idx.unwrap())
                    && !indices_to_check.contains(&neighbour_idx.unwrap())
                {
                    indices_to_check.push(neighbour_idx.unwrap());
                }
            } else if !processed_indices.contains(&neighbour_idx.unwrap())
                && !indices_same_group.contains(&neighbour_idx.unwrap())
            {
                // element same garden
                indices_same_group.push(neighbour_idx.unwrap());
            }
        }

        if let Some(i) = indices_same_group.pop() {
            idx = i;
        } else {
            score += size * perimeter;
            size = 0;
            perimeter = 0;

            match indices_to_check
                .iter()
                .rev()
                .find(|i| !processed_indices.contains(i))
            {
                Some(i) => idx = *i,
                None => break,
            }
        }
    }
    score
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn to_vector(&self) -> (isize, isize) {
        match self {
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Up => (0, -1),
        }
    }
}

fn calculate_fence_price_2(grid: &CharGrid) -> usize {
    let directions = [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ];

    let mut indices_to_check: Vec<BoundVector2D> = Vec::new();
    let mut processed_indices: HashSet<BoundVector2D> = HashSet::new();
    let mut indices_same_group: Vec<BoundVector2D> = Vec::new();

    let mut score = 0;
    let mut size = 0;
    let mut fences: HashSet<(BoundVector2D, Direction)> = HashSet::new();

    let mut idx = BoundVector2D::new(0, 0, grid.rows, grid.cols).unwrap();
    loop {
        processed_indices.insert(idx);
        size += 1;

        let item = grid.get(idx).unwrap();
        for dir in &directions {
            let neighbour_idx = idx + dir.to_vector();
            let c = neighbour_idx.and_then(|i| grid.get(i));
            if c.is_none() {
                // edge
                fences.insert((idx, dir.clone()));
            } else if *c.unwrap() != *item {
                // border to other
                fences.insert((idx, dir.clone()));
                if !processed_indices.contains(&neighbour_idx.unwrap())
                    && !indices_to_check.contains(&neighbour_idx.unwrap())
                {
                    indices_to_check.push(neighbour_idx.unwrap());
                }
            } else if !processed_indices.contains(&neighbour_idx.unwrap())
                && !indices_same_group.contains(&neighbour_idx.unwrap())
            {
                // element same garden
                indices_same_group.push(neighbour_idx.unwrap());
            }
        }

        if let Some(i) = indices_same_group.pop() {
            idx = i;
        } else {
            let sides = calculate_sides(&fences);
            score += size * sides;
            size = 0;
            fences.clear();

            match indices_to_check
                .iter()
                .rev()
                .find(|i| !processed_indices.contains(i))
            {
                Some(i) => idx = *i,
                None => break,
            }
        }
    }
    score
}

fn calculate_sides(fences: &HashSet<(BoundVector2D, Direction)>) -> usize {
    let mut processed_fences: HashSet<(BoundVector2D, Direction)> = HashSet::new();
    let mut fences_same_side: HashSet<(BoundVector2D, Direction)> = HashSet::new();
    let mut sides = 1;

    let mut cur_item = fences.iter().next().unwrap().clone();

    loop {
        processed_fences.insert(cur_item.clone());

        let n_dir = get_neighbour_direction(&cur_item.1).to_vector();

        // check neighbour 1
        if let Some(n) = cur_item.0 + n_dir {
            let fence = (n, cur_item.1.clone());
            if fences.contains(&fence) {
                fences_same_side.insert(fence);
            }
        }

        // check neighbour 2
        if let Some(n) = cur_item.0 - n_dir {
            let fence = (n, cur_item.1.clone());
            if fences.contains(&fence) {
                fences_same_side.insert(fence);
            }
        }

        if let Some(i) = fences_same_side
            .iter()
            .find(|i| !processed_fences.contains(i))
        {
            cur_item = i.clone();
        } else if let Some(i) = fences
            .iter()
            .find(|i| !processed_fences.contains(i))
        {
            sides += 1;
            cur_item = i.clone();
        } else {
            break;
        }
    }
    sides
}

fn get_neighbour_direction(dir: &Direction) -> Direction {
    match dir {
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_large_example() {
        let grid = CharGrid::from_string(
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
        )
        .unwrap();
        let result = calculate_fence_price(&grid);
        assert_eq!(result, 1930);
    }

    #[test]
    fn test_small_example_1() {
        let grid = CharGrid::from_string(
            "AAAA
BBCD
BBCC
EEEC",
        )
        .unwrap();
        let result = calculate_fence_price(&grid);
        assert_eq!(result, 140);
    }

    #[test]
    fn test_small_example_2() {
        let grid = CharGrid::from_string(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        )
        .unwrap();
        let result = calculate_fence_price(&grid);
        assert_eq!(result, 772);
    }

    #[test]
    fn test_small_example_3() {
        let grid = CharGrid::from_string(
            "BBCD
EEEC",
        )
        .unwrap();
        let result = calculate_fence_price(&grid);
        assert_eq!(result, 48);
    }

    #[test]
    fn test_tiny_example_1() {
        let grid = CharGrid::from_string("A").unwrap();
        let result = calculate_fence_price(&grid);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_tiny_example_2() {
        let grid = CharGrid::from_string("AB").unwrap();
        let result = calculate_fence_price(&grid);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_tiny_example_3() {
        let grid = CharGrid::from_string("AAA").unwrap();
        let result = calculate_fence_price(&grid);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_2_example() {
        let grid = CharGrid::from_string(
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        )
        .unwrap();
        let result = calculate_fence_price_2(&grid);
        assert_eq!(result, 1206);
    }

    #[test]
    fn test_2_example_1() {
        let grid = CharGrid::from_string(
            "AAAA
BBCD
BBCC
EEEC",
        )
        .unwrap();
        let result = calculate_fence_price_2(&grid);
        assert_eq!(result, 80);
    }

    #[test]
    fn test_2_example_2() {
        let grid = CharGrid::from_string(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        )
        .unwrap();
        let result = calculate_fence_price_2(&grid);
        assert_eq!(result, 436);
    }

    #[test]
    fn test_2_example_3() {
        let grid = CharGrid::from_string(
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        )
        .unwrap();
        let result = calculate_fence_price_2(&grid);
        assert_eq!(result, 236);
    }

    #[test]
    fn test_2_example_special_case() {
        let grid = CharGrid::from_string(
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
        )
        .unwrap();
        let result = calculate_fence_price_2(&grid);
        assert_eq!(result, 368);
    }

    #[test]
    fn test_2_tiny_1() {
        let grid = CharGrid::from_string("AABB").unwrap();
        let result = calculate_fence_price_2(&grid);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_2_tiny_2() {
        let grid = CharGrid::from_string("AAABBB").unwrap();
        let result = calculate_fence_price_2(&grid);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_2_tiny_3() {
        let grid = CharGrid::from_string(
            "AB
CD",
        )
        .unwrap();
        let result = calculate_fence_price_2(&grid);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_2_tiny_4() {
        let grid = CharGrid::from_string(
            "AB
AA",
        )
        .unwrap();
        let result = calculate_fence_price_2(&grid);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_calculate_sides_1() {
        let vec = BoundVector2D::new(0, 0, 1, 1).unwrap();
        let fences = HashSet::from([
            (vec, Direction::Right),
            (vec, Direction::Down),
            (vec, Direction::Left),
            (vec, Direction::Up),
        ]);

        let result = calculate_sides(&fences);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_calculate_sides_2() {
        let vec1 = BoundVector2D::new(0, 0, 2, 2).unwrap();
        let vec2 = BoundVector2D::new(1, 0, 2, 2).unwrap();
        let fences = HashSet::from([
            (vec1, Direction::Left),
            (vec1, Direction::Up),
            (vec1, Direction::Down),
            (vec2, Direction::Right),
            (vec2, Direction::Down),
            (vec2, Direction::Up),
        ]);

        let result = calculate_sides(&fences);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_calculate_sides_3() {
        let vec1 = BoundVector2D::new(0, 0, 2, 2).unwrap();
        let vec2 = BoundVector2D::new(0, 1, 2, 2).unwrap();
        let vec3 = BoundVector2D::new(1, 1, 2, 2).unwrap();
        let fences = HashSet::from([
            (vec1, Direction::Left),
            (vec1, Direction::Up),
            (vec1, Direction::Right),
            (vec2, Direction::Left),
            (vec2, Direction::Down),
            (vec3, Direction::Up),
            (vec3, Direction::Right),
            (vec3, Direction::Down),
        ]);

        let result = calculate_sides(&fences);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_main() {
        let content = read_to_string("input.txt").unwrap();
        let grid = CharGrid::from_string(&content).unwrap();
        let fence_price = calculate_fence_price(&grid);
        assert_eq!(fence_price, 1431440);
        println!("Price for fence around gardens: {}", fence_price);
        let fence_price_2 = calculate_fence_price_2(&grid);
        assert_eq!(fence_price_2, 869070);
    }
}
