pub type LevelItem = isize;

pub fn check_level(level: &[LevelItem]) -> bool {
    let mut ordering: Option<bool> = Option::None;
    let mut last_item: Option<&LevelItem> = Option::None;

    for item in level.iter() {
        if ordering.is_none() {
            if last_item.is_none() {
                last_item = Some(item);
                continue;
            } else {
                ordering = Some(item < last_item.expect("Checked above"));
            }
        }

        let ordered_correctly: bool;

        if ordering.expect("Checked above") {
            ordered_correctly = item < last_item.unwrap();
        } else {
            ordered_correctly = item > last_item.unwrap();
        }

        if !ordered_correctly {
            return false;
        }

        if item.abs_diff(*last_item.unwrap()) > 3 {
            return false;
        }

        last_item = Some(item);
    }
    true
}

pub fn check_level_new(level: &[LevelItem]) -> bool {
    if level.len() <= 1 {
        return true;
    }

    let mut ordering = None;

    for window in level.windows(2) {
        let (prev, curr) = (&window[0], &window[1]);

        if let Some(is_ascending) = ordering {
            if is_ascending && prev >= curr || !is_ascending && prev <= curr {
                return false;
            }
        } else {
            ordering = Some(prev < curr);
        }

        if curr.abs_diff(*prev) > 3 {
            return false;
        }
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
        assert_eq!(result, true);
    }

    #[test]
    fn test_example_2() {
        let result = check_level_move(vec![1, 2, 7, 8, 9]);
        assert_eq!(result, false);
    }

    #[test]
    fn test_example_3() {
        let result = check_level_move(vec![9, 7, 6, 2, 1]);
        assert_eq!(result, false);
    }

    #[test]
    fn test_example_4() {
        let result = check_level_move(vec![1, 3, 2, 4, 5]);
        assert_eq!(result, false);
    }

    #[test]
    fn test_example_5() {
        let result = check_level_move(vec![8, 6, 4, 4, 1]);
        assert_eq!(result, false);
    }

    #[test]
    fn test_example_6() {
        let result = check_level_move(vec![1, 3, 6, 7, 9]);
        assert_eq!(result, true);
    }

    #[test]
    fn test_example_1_skip() {
        let result = check_level_move_skip(vec![7, 6, 4, 2, 1]);
        assert_eq!(result, true);
    }

    #[test]
    fn test_example_2_skip() {
        let result = check_level_move_skip(vec![1, 2, 7, 8, 9]);
        assert_eq!(result, false);
    }

    #[test]
    fn test_example_3_skip() {
        let result = check_level_move_skip(vec![9, 7, 6, 2, 1]);
        assert_eq!(result, false);
    }

    #[test]
    fn test_example_4_skip() {
        let result = check_level_move_skip(vec![1, 3, 2, 4, 5]);
        assert_eq!(result, true);
    }

    #[test]
    fn test_example_5_skip() {
        let result = check_level_move_skip(vec![8, 6, 4, 4, 1]);
        assert_eq!(result, true);
    }

    #[test]
    fn test_example_6_skip() {
        let result = check_level_move_skip(vec![1, 3, 6, 7, 9]);
        assert_eq!(result, true);
    }

    #[test]
    fn test_example_7_skip() {
        let result = check_level_move_skip(vec![4, 6, 3, 2, 1]);
        assert_eq!(result, true);
    }

    #[test]
    fn test_example_8_skip() {
        let result = check_level_move_skip(vec![4, 6, 5, 3, 1]);
        assert_eq!(result, true);
    }

    #[test]
    fn test_example_9_skip() {
        let result = check_level_move_skip(vec![4, 11, 7]);
        assert_eq!(result, true);
    }
}
