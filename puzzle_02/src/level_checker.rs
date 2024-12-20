pub type LevelItem = isize;

pub fn check_level(level: &[LevelItem]) -> bool {
    if level.len() <= 1 {
        return true;
    }

    let mut ordering = None;
    let mut last_item = level[0];

    for item in level[1..].iter() {
        if ordering.is_none() {
            ordering = Some(*item < last_item);
        }

        let ordered_correctly = if ordering.expect("Checked above") {
            *item < last_item
        } else {
            *item > last_item
        };

        if !ordered_correctly || item.abs_diff(last_item) > 3 {
            return false;
        }

        last_item = *item;
    }
    true
}

pub fn check_level_skip(level: &[LevelItem]) -> bool {
    if level.len() <= 2 {
        return true;
    }

    if check_level(level) {
        return true;
    }

    for i in 0..level.len() {
        let mut modified_level = level.to_vec();
        modified_level.remove(i);

        if check_level(&modified_level) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_level_move(level: Vec<LevelItem>) -> bool {
        check_level(&level)
    }
    fn check_level_move_skip(level: Vec<LevelItem>) -> bool {
        check_level_skip(&level)
    }

    #[test]
    fn test_example_1() {
        let result = check_level_move(vec![7, 6, 4, 2, 1]);
        assert!(result);
    }

    #[test]
    fn test_example_2() {
        let result = check_level_move(vec![1, 2, 7, 8, 9]);
        assert!(!result);
    }

    #[test]
    fn test_example_3() {
        let result = check_level_move(vec![9, 7, 6, 2, 1]);
        assert!(!result);
    }

    #[test]
    fn test_example_4() {
        let result = check_level_move(vec![1, 3, 2, 4, 5]);
        assert!(!result);
    }

    #[test]
    fn test_example_5() {
        let result = check_level_move(vec![8, 6, 4, 4, 1]);
        assert!(!result);
    }

    #[test]
    fn test_example_6() {
        let result = check_level_move(vec![1, 3, 6, 7, 9]);
        assert!(result);
    }

    #[test]
    fn test_example_1_skip() {
        let result = check_level_move_skip(vec![7, 6, 4, 2, 1]);
        assert!(result);
    }

    #[test]
    fn test_example_2_skip() {
        let result = check_level_move_skip(vec![1, 2, 7, 8, 9]);
        assert!(!result);
    }

    #[test]
    fn test_example_3_skip() {
        let result = check_level_move_skip(vec![9, 7, 6, 2, 1]);
        assert!(!result);
    }

    #[test]
    fn test_example_4_skip() {
        let result = check_level_move_skip(vec![1, 3, 2, 4, 5]);
        assert!(result);
    }

    #[test]
    fn test_example_5_skip() {
        let result = check_level_move_skip(vec![8, 6, 4, 4, 1]);
        assert!(result);
    }

    #[test]
    fn test_example_6_skip() {
        let result = check_level_move_skip(vec![1, 3, 6, 7, 9]);
        assert!(result);
    }

    #[test]
    fn test_example_7_skip() {
        let result = check_level_move_skip(vec![4, 6, 3, 2, 1]);
        assert!(result);
    }

    #[test]
    fn test_example_8_skip() {
        let result = check_level_move_skip(vec![4, 6, 5, 3, 1]);
        assert!(result);
    }

    #[test]
    fn test_example_9_skip() {
        let result = check_level_move_skip(vec![4, 11, 7]);
        assert!(result);
    }
}
