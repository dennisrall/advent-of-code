use std::ops::{Index, IndexMut};

use crate::{direction::Direction, vector::BoundVector2D};

#[derive(Debug, Default, Clone)]
pub struct CharGrid {
    pub rows: usize,
    pub cols: usize,
    grid: Vec<char>,
}

impl CharGrid {
    pub fn from_string(input: &str) -> Result<CharGrid, &'static str> {
        let rows_vec: Vec<_> = input.lines().collect();
        if rows_vec.is_empty() {
            return Ok(CharGrid::default());
        }

        let cols = rows_vec[0].len();
        if rows_vec.iter().any(|row| row.len() != cols) {
            return Err("All rows must have the same size.");
        }

        let grid = rows_vec.iter().flat_map(|row| row.chars()).collect();

        let rows = rows_vec.iter().len();

        Ok(CharGrid { rows, cols, grid })
    }

    fn get_internal_index(&self, index: BoundVector2D) -> usize {
        self.cols * index.x + index.y
    }

    pub fn get(&self, index: BoundVector2D) -> Option<&char> {
        let idx = index.bound(self.rows, self.cols)?;
        self.grid.get(self.get_internal_index(idx))
    }

    pub fn set(&mut self, index: BoundVector2D, c: char) -> Option<()> {
        let idx = index.bound(self.rows, self.cols)?;
        let index = self.get_internal_index(idx);
        self.grid[index] = c;
        Some(())
    }

    pub fn iter_indices(&self) -> impl Iterator<Item = BoundVector2D> + '_ {
        (0..self.rows).flat_map(move |x| {
            (0..self.cols).map(move |y| BoundVector2D::new(x, y, self.rows, self.cols).unwrap())
        })
    }

    pub fn get_vector_from_direction(
        &self,
        start: &BoundVector2D,
        direction: &Direction,
        length: &usize,
    ) -> Option<Vec<char>> {
        let mut result = vec![];
        let dir = Direction::get_vector(&direction);
        let mut idx = Some(*start);
        for _ in 0..*length {
            result.push(*self.get(idx?)?);
            idx = idx.unwrap() + dir;
        }
        Some(result)
    }
}

impl Index<BoundVector2D> for CharGrid {
    type Output = char;

    fn index(&self, index: BoundVector2D) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl IndexMut<BoundVector2D> for CharGrid {
    fn index_mut(&mut self, index: BoundVector2D) -> &mut Self::Output {
        let idx = self.get_internal_index(index);
        &mut self.grid[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        let input = "abc\ndef\nghi";
        let grid = CharGrid::from_string(input).expect("Failed to create valid CharGrid");

        assert_eq!(grid.rows, 3);
        assert_eq!(grid.cols, 3);
        assert_eq!(grid.grid, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']);

        // Test specific grid values
        assert_eq!(
            grid.get(BoundVector2D::new(0, 0, grid.rows, grid.cols).unwrap()),
            Some(&'a')
        );
        assert_eq!(
            grid.get(BoundVector2D::new(1, 1, grid.rows, grid.cols).unwrap()),
            Some(&'e')
        );
        assert_eq!(
            grid.get(BoundVector2D::new(2, 2, grid.rows, grid.cols).unwrap()),
            Some(&'i')
        );

        assert_eq!(
            grid[BoundVector2D::new(0, 0, grid.rows, grid.cols).unwrap()],
            'a'
        );
        assert_eq!(
            grid[BoundVector2D::new(1, 1, grid.rows, grid.cols).unwrap()],
            'e'
        );
        assert_eq!(
            grid[BoundVector2D::new(2, 2, grid.rows, grid.cols).unwrap()],
            'i'
        );
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let grid = CharGrid::from_string(input).expect("Failed to create empty CharGrid");

        assert_eq!(grid.rows, 0);
        assert_eq!(grid.cols, 0);
        assert_eq!(grid.grid, vec![]);
    }

    #[test]
    fn test_inconsistent_row_lengths() {
        let input = "abc\ndefg\nhi";
        let result = CharGrid::from_string(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "All rows must have the same size.");
    }

    #[test]
    fn test_single_row() {
        let input = "abcdef";
        let grid = CharGrid::from_string(input).expect("Failed to create valid CharGrid");

        assert_eq!(grid.rows, 1);
        assert_eq!(grid.cols, 6);
        assert_eq!(grid.grid, vec!['a', 'b', 'c', 'd', 'e', 'f']);

        // Test specific grid values
        assert_eq!(
            grid.get(BoundVector2D::new(0, 0, grid.rows, grid.cols).unwrap()),
            Some(&'a')
        );
        assert_eq!(
            grid.get(BoundVector2D::new(0, 5, grid.rows, grid.cols).unwrap()),
            Some(&'f')
        );
        assert_eq!(
            grid.get(BoundVector2D::new(0, 6, grid.rows, grid.cols + 1).unwrap()),
            None
        ); // Out of bounds

        assert_eq!(
            grid[BoundVector2D::new(0, 0, grid.rows, grid.cols).unwrap()],
            'a'
        );
        assert_eq!(
            grid[BoundVector2D::new(0, 5, grid.rows, grid.cols).unwrap()],
            'f'
        );
    }

    #[test]
    #[should_panic]
    fn test_single_row_out_of_bounds() {
        let input = "abcdef";
        let grid = CharGrid::from_string(input).expect("Failed to create valid CharGrid");

        grid[BoundVector2D::new(1, 0, grid.rows, grid.cols).unwrap()];
    }

    #[test]
    fn test_single_column() {
        let input = "a\nb\nc\nd";
        let grid = CharGrid::from_string(input).expect("Failed to create valid CharGrid");

        assert_eq!(grid.rows, 4);
        assert_eq!(grid.cols, 1);
        assert_eq!(grid.grid, vec!['a', 'b', 'c', 'd']);

        // Test specific grid values
        assert_eq!(
            grid.get(BoundVector2D::new(0, 0, grid.rows, grid.cols).unwrap()),
            Some(&'a')
        );
        assert_eq!(
            grid.get(BoundVector2D::new(3, 0, grid.rows, grid.cols).unwrap()),
            Some(&'d')
        );
        assert_eq!(
            grid.get(BoundVector2D::new(4, 0, grid.rows + 1, grid.cols).unwrap()),
            None
        ); // Out of bounds

        assert_eq!(
            grid[BoundVector2D::new(0, 0, grid.rows, grid.cols).unwrap()],
            'a'
        );
        assert_eq!(
            grid[BoundVector2D::new(3, 0, grid.rows, grid.cols).unwrap()],
            'd'
        );
    }

    #[test]
    #[should_panic]
    fn test_single_column_out_of_bounds() {
        let input = "a\nb\nc\nd";
        let grid = CharGrid::from_string(input).expect("Failed to create valid CharGrid");

        grid[BoundVector2D::new(4, 0, grid.rows, grid.cols).unwrap()];
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "a b c\nd e f\ng h i";
        let grid = CharGrid::from_string(input).expect("Failed to create valid CharGrid");

        assert_eq!(grid.rows, 3);
        assert_eq!(grid.cols, 5); // Spaces are treated as valid characters
        assert_eq!(
            grid.grid,
            vec!['a', ' ', 'b', ' ', 'c', 'd', ' ', 'e', ' ', 'f', 'g', ' ', 'h', ' ', 'i']
        );
    }

    #[test]
    fn test_iter_indices() {
        let input = "a b\nc d";
        let grid = CharGrid::from_string(input).expect("Failed to create valid CharGrid");

        let mut i = grid.iter_indices();

        assert_eq!(
            i.next(),
            Some(BoundVector2D::new(0, 0, grid.rows, grid.cols).unwrap())
        );
        assert_eq!(
            i.next(),
            Some(BoundVector2D::new(0, 1, grid.rows, grid.cols).unwrap())
        );
        assert_eq!(
            i.next(),
            Some(BoundVector2D::new(0, 2, grid.rows, grid.cols).unwrap())
        );
        assert_eq!(
            i.next(),
            Some(BoundVector2D::new(1, 0, grid.rows, grid.cols).unwrap())
        );
        assert_eq!(
            i.next(),
            Some(BoundVector2D::new(1, 1, grid.rows, grid.cols).unwrap())
        );
        assert_eq!(
            i.next(),
            Some(BoundVector2D::new(1, 2, grid.rows, grid.cols).unwrap())
        );

        assert_eq!(i.next(), None);
    }

    #[test]
    fn test_get_vector() {
        let input = "a b\nc d";
        let grid = CharGrid::from_string(input).expect("Failed to create valid CharGrid");
        let result = grid.get_vector_from_direction(
            &BoundVector2D::new(0, 0, grid.rows, grid.cols).unwrap(),
            &Direction::FORWARD,
            &2,
        );
        assert_eq!(result, Some(vec!['a', ' ']));
    }

    #[test]
    fn test_get_vector_2() {
        let input = "abcd";
        let grid = CharGrid::from_string(input).expect("Failed to create valid CharGrid");
        let result = grid.get_vector_from_direction(
            &BoundVector2D::new(0, 0, grid.rows, grid.cols).unwrap(),
            &Direction::FORWARD,
            &4,
        );
        assert_eq!(result, Some(vec!['a', 'b', 'c', 'd']));
    }

    #[test]
    fn test_get_vector_3() {
        let input = "a\nb\nc\nd";
        let grid = CharGrid::from_string(input).expect("Failed to create valid CharGrid");
        assert_eq!(grid.rows, 4);
        assert_eq!(grid.cols, 1);
        let result = grid.get_vector_from_direction(
            &BoundVector2D::new(0, 0, grid.rows, grid.cols).unwrap(),
            &Direction::FORWARD,
            &4,
        );
        assert_eq!(result, None);
    }

    #[test]
    fn test_set_valid() {
        let input = "abcdef";
        let mut grid = CharGrid::from_string(input).expect("Failed to create valid CharGrid");

        assert_eq!(grid.grid, vec!['a', 'b', 'c', 'd', 'e', 'f']);
        let result = grid.set(BoundVector2D::new(0, 3, grid.rows, grid.cols).unwrap(), 'e');
        assert_eq!(result, Some(()));
        assert_eq!(grid.grid, vec!['a', 'b', 'c', 'e', 'e', 'f']);
    }
    #[test]
    fn test_set_invalid() {
        let input = "abcdef";
        let mut grid = CharGrid::from_string(input).expect("Failed to create valid CharGrid");

        let result = grid.set(
            BoundVector2D::new(1, 3, grid.rows + 1, grid.cols).unwrap(),
            'e',
        );
        assert_eq!(result, None);
        assert_eq!(grid.grid, vec!['a', 'b', 'c', 'd', 'e', 'f']);
    }
}
