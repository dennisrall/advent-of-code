use counter::Counter;


pub fn occurence_similarity(vec_a: & mut Vec<usize>, vec_b: & mut Vec<usize>) -> usize {
    let counter_a = vec_a.iter().collect::<Counter<_>>();
    let counter_b = vec_b.iter().collect::<Counter<_>>();
    let mut score = 0;
    for (num, count_a) in counter_a.iter() {
        let count_b = counter_b.get(num).unwrap_or(&0);
        score += *num * count_a * count_b;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    fn occurence_similarity_move(mut vec_a: Vec<usize>, mut vec_b: Vec<usize>) -> usize {
        occurence_similarity(&mut vec_a, &mut vec_b)
    }

    #[test]
    fn test_example_1() {
        let result = occurence_similarity_move(vec![1, 2], vec![2, 4]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example_2() {
        let result = occurence_similarity_move(vec![1, 2, 4, 3], vec![2, 4, 2, 3, 4, 4, 5]);
        assert_eq!(result, 19);
    }

    #[test]
    fn test_empty_1() {
        let result = occurence_similarity_move(vec![], vec![2, 4]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_empty_2() {
        let result = occurence_similarity_move(vec![1, 2], vec![]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_example_advent() {
        let result = occurence_similarity_move(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(result, 31);
    }
}
