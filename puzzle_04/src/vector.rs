use std::ops::{Add, Sub};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Vector2D {
    pub x: usize,
    pub y: usize,
}

impl Add<(isize, isize)> for Vector2D {
    type Output = Option<Vector2D>;

    fn add(self, other: (isize, isize)) -> Self::Output {
        Some(Vector2D {
            x: self.x.checked_add_signed(other.0)?,
            y: self.y.checked_add_signed(other.1)?,
        })
    }
}

impl Add for Vector2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2D {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        Some(Vector2D{
            x: self.x.checked_sub(rhs.x)?,
            y: self.y.checked_sub(rhs.y)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector2d_addition() {
        let v1 = Vector2D { x: 2, y: 3 };
        let v2 = Vector2D { x: 4, y: 5 };
        let result = v1 + v2;
        let expected = Vector2D { x: 6, y: 8 };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector2d_subtraction_valid() {
        let v1 = Vector2D { x: 6, y: 8 };
        let v2 = Vector2D { x: 4, y: 5 };
        let result = v1 - v2;
        let expected = Some(Vector2D { x: 2, y: 3 });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector2d_subtraction_negative_result() {
        let v1 = Vector2D { x: 2, y: 3 };
        let v2 = Vector2D { x: 4, y: 5 };
        let result = v1 - v2;
        assert_eq!(result, None);
    }

    #[test]
    fn test_vector2d_addition_identity() {
        let v = Vector2D { x: 3, y: 4 };
        let zero = Vector2D { x: 0, y: 0 };
        let result = v + zero;
        assert_eq!(result, v);
    }

    #[test]
    fn test_vector2d_subtraction_identity() {
        let v = Vector2D { x: 3, y: 4 };
        let zero = Vector2D { x: 0, y: 0 };
        let result = v - zero;
        assert_eq!(result, Some(v));
    }

    #[test]
    fn test_vector2d_self_subtraction() {
        let v = Vector2D { x: 3, y: 4 };
        let result = v - v;
        let expected = Some(Vector2D { x: 0, y: 0 });
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector2d_add_valid() {
        let vec = Vector2D { x: 5, y: 10 };
        let result = vec + (2, 3);
        assert_eq!(result, Some(Vector2D { x: 7, y: 13 }));
    }

    #[test]
    fn test_vector2d_add_valid_2() {
        let vec = Vector2D { x: 5, y: 10 };
        let result = vec + (-1, -2);
        assert_eq!(result, Some(Vector2D { x: 4, y: 8 }));
    }

    #[test]
    fn test_vector2d_add_valid_3() {
        let vec = Vector2D { x: 0, y: 0 };
        let result = vec + (1, 1);
        assert_eq!(result, Some(Vector2D {x: 1, y: 1}));

    }

    #[test]
    fn test_vector2d_add_invalid() {
        let vec = Vector2D { x: 0, y: 0 };
        let result = vec + (-1, -1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_default_vector() {
        let default_vec = Vector2D::default();
        assert_eq!(default_vec, Vector2D { x: 0, y: 0 });
    }

    #[test]
    fn test_clone_and_copy() {
        let vec = Vector2D { x: 5, y: 10 };
        let vec_clone = vec.clone();

        assert_eq!(vec, vec_clone);
        assert!(std::ptr::eq(&vec, &vec_clone) == false);
    }

    #[test]
    fn test_add_zero() {
        let vec = Vector2D { x: 5, y: 10 };
        let result = vec + (0, 0);
        assert_eq!(result, Some(Vector2D { x: 5, y: 10 }));
    }
}

