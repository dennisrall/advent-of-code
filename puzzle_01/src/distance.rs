use std::iter::zip;


pub fn ordered_distance(vec_a: & mut Vec<usize>, vec_b: & mut Vec<usize>) -> usize {
    vec_a.sort();
    vec_b.sort();
    zip(vec_a, vec_b).map(|(a, b)| a.abs_diff(*b)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ordered_distance_move(mut vec_a: Vec<usize>, mut vec_b: Vec<usize>) -> usize {
        ordered_distance(&mut vec_a, &mut vec_b)
    }

    #[test]
    fn test_ordered_unequal() {
        let result = ordered_distance_move(vec![1, 2], vec![2, 4]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_unordered_unequal() {
        let result = ordered_distance_move(vec![1, 2], vec![4, 2]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_ordered_equal() {
        let result = ordered_distance_move(vec![0, 1, 2, 3], vec![0, 1, 2, 3]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_unordered_equal() {
        let result = ordered_distance_move(vec![0, 1, 2, 3], vec![0, 3, 2, 1]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_example_advent() {
        let result = ordered_distance_move(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(result, 11);
    }
}

