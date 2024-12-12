use std::{
    cmp::min,
    ops::{Add, Sub},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct BoundVector2D {
    pub x: usize,
    pub y: usize,
    bound_x: usize,
    bound_y: usize,
}

impl BoundVector2D {
    pub fn new(x: usize, y: usize, bound_x: usize, bound_y: usize) -> Option<Self> {
        (x < bound_x && y < bound_y).then_some(Self {
            x,
            y,
            bound_x,
            bound_y,
        })
    }

    pub fn from_sized(x_s: isize, y_s: isize, bound_x: usize, bound_y: usize) -> Option<Self> {
        let x: usize = x_s.try_into().ok()?;
        let y: usize = y_s.try_into().ok()?;
        BoundVector2D::new(x, y, bound_x, bound_y)
    }

    pub fn to_sized(self) -> Option<(isize, isize)> {
        Some((self.x.try_into().ok()?, self.y.try_into().ok()?))
    }

    pub fn bound(&self, bound_x: usize, bound_y: usize) -> Option<Self> {
        BoundVector2D::new(self.x, self.y, bound_x, bound_y)
    }
}

impl Add for BoundVector2D {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        BoundVector2D::new(
            self.x + rhs.x,
            self.y + rhs.y,
            min(self.bound_x, rhs.bound_x),
            min(self.bound_y, rhs.bound_y),
        )
    }
}

impl Sub for BoundVector2D {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        BoundVector2D::new(
            self.x.checked_sub(rhs.x)?,
            self.y.checked_sub(rhs.y)?,
            min(self.bound_x, rhs.bound_x),
            min(self.bound_y, rhs.bound_y),
        )
    }
}

impl Add<(isize, isize)> for BoundVector2D {
    type Output = Option<Self>;

    fn add(self, other: (isize, isize)) -> Option<Self> {
        BoundVector2D::new(
            self.x.checked_add_signed(other.0)?,
            self.y.checked_add_signed(other.1)?,
            self.bound_x,
            self.bound_y,
        )
    }
}

impl Sub<(isize, isize)> for BoundVector2D {
    type Output = Option<Self>;

    fn sub(self, other: (isize, isize)) -> Option<Self> {
        BoundVector2D::new(
            self.x.checked_add_signed(-other.0)?,
            self.y.checked_add_signed(-other.1)?,
            self.bound_x,
            self.bound_y,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid_bounds() {
        let vec = BoundVector2D::new(2, 3, 5, 5);
        assert_eq!(
            vec,
            Some(BoundVector2D {
                x: 2,
                y: 3,
                bound_x: 5,
                bound_y: 5
            })
        );
    }

    #[test]
    fn test_new_out_of_bounds() {
        let vec = BoundVector2D::new(6, 3, 5, 5);
        assert_eq!(vec, None);
    }

    #[test]
    fn test_from_sized_valid_bounds() {
        let vec = BoundVector2D::from_sized(2, 3, 5, 5);
        assert_eq!(
            vec,
            Some(BoundVector2D {
                x: 2,
                y: 3,
                bound_x: 5,
                bound_y: 5
            })
        );
    }

    #[test]
    fn test_from_sized_negative_values() {
        let vec = BoundVector2D::from_sized(-1, 3, 5, 5);
        assert_eq!(vec, None);
    }

    #[test]
    fn test_add_vectors_within_bounds() {
        let vec1 = BoundVector2D::new(1, 1, 5, 5).unwrap();
        let vec2 = BoundVector2D::new(2, 2, 5, 5).unwrap();
        let result = vec1 + vec2;
        assert_eq!(
            result,
            Some(BoundVector2D {
                x: 3,
                y: 3,
                bound_x: 5,
                bound_y: 5
            })
        );
    }

    #[test]
    fn test_add_vectors_exceeding_bounds() {
        let vec1 = BoundVector2D::new(4, 4, 5, 5).unwrap();
        let vec2 = BoundVector2D::new(2, 2, 5, 5).unwrap();
        let result = vec1 + vec2;
        assert_eq!(result, None);
    }

    #[test]
    fn test_sub_vectors_within_bounds() {
        let vec1 = BoundVector2D::new(3, 3, 5, 5).unwrap();
        let vec2 = BoundVector2D::new(1, 1, 5, 5).unwrap();
        let result = vec1 - vec2;
        assert_eq!(
            result,
            Some(BoundVector2D {
                x: 2,
                y: 2,
                bound_x: 5,
                bound_y: 5
            })
        );
    }

    #[test]
    fn test_sub_vectors_below_zero() {
        let vec1 = BoundVector2D::new(1, 1, 5, 5).unwrap();
        let vec2 = BoundVector2D::new(2, 2, 5, 5).unwrap();
        let result = vec1 - vec2;
        assert_eq!(result, None);
    }

    #[test]
    fn test_add_tuple_within_bounds() {
        let vec = BoundVector2D::new(1, 1, 5, 5).unwrap();
        let result = vec + (2, 3);
        assert_eq!(
            result,
            Some(BoundVector2D {
                x: 3,
                y: 4,
                bound_x: 5,
                bound_y: 5
            })
        );
    }

    #[test]
    fn test_add_tuple_exceeding_bounds() {
        let vec = BoundVector2D::new(4, 4, 5, 5).unwrap();
        let result = vec + (2, 1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_add_tuple_negative_values() {
        let vec = BoundVector2D::new(4, 4, 5, 5).unwrap();
        let result = vec + (-2, -3);
        assert_eq!(
            result,
            Some(BoundVector2D {
                x: 2,
                y: 1,
                bound_x: 5,
                bound_y: 5
            })
        );
    }

    #[test]
    fn test_add_tuple_resulting_negative() {
        let vec = BoundVector2D::new(1, 1, 5, 5).unwrap();
        let result = vec + (-2, -1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_sub_tuple_within_bounds() {
        let vec = BoundVector2D::new(1, 1, 5, 5).unwrap();
        let result = vec - (-2, -3);
        assert_eq!(
            result,
            Some(BoundVector2D {
                x: 3,
                y: 4,
                bound_x: 5,
                bound_y: 5
            })
        );
    }

    #[test]
    fn test_sub_tuple_resulting_negative() {
        let vec = BoundVector2D::new(1, 1, 5, 5).unwrap();
        let result = vec - (2, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_bound_within_new_bounds() {
        let vec = BoundVector2D::new(2, 3, 5, 5).unwrap();
        let result = vec.bound(4, 4);
        assert_eq!(
            result,
            Some(BoundVector2D {
                x: 2,
                y: 3,
                bound_x: 4,
                bound_y: 4
            })
        );
    }

    #[test]
    fn test_bound_exceeding_new_bounds() {
        let vec = BoundVector2D::new(4, 4, 6, 6).unwrap();
        let result = vec.bound(3, 3);
        assert_eq!(result, None);
    }

    #[test]
    fn test_bound_equal_new_bounds() {
        let vec = BoundVector2D::new(3, 3, 5, 5).unwrap();
        let result = vec.bound(4, 4);
        assert_eq!(
            result,
            Some(BoundVector2D {
                x: 3,
                y: 3,
                bound_x: 4,
                bound_y: 4
            })
        );
    }

    #[test]
    fn test_bound_larger_than_current_bounds() {
        let vec = BoundVector2D::new(2, 2, 5, 5).unwrap();
        let result = vec.bound(10, 10);
        assert_eq!(
            result,
            Some(BoundVector2D {
                x: 2,
                y: 2,
                bound_x: 10,
                bound_y: 10
            })
        );
    }

    #[test]
    fn test_bound_no_change_in_bounds() {
        let vec = BoundVector2D::new(1, 1, 5, 5).unwrap();
        let result = vec.bound(5, 5);
        assert_eq!(
            result,
            Some(BoundVector2D {
                x: 1,
                y: 1,
                bound_x: 5,
                bound_y: 5
            })
        );
    }

    #[test]
    fn test_to_sized() {
        let vec = BoundVector2D::new(1, 1, 5, 5).unwrap();
        let result = vec.to_sized();
        assert_eq!(result, Some((1, 1)));
    }
}
