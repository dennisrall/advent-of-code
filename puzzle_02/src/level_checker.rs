pub type LevelItem = isize;

pub fn check_level(level: &Vec<LevelItem>, skip_one: bool) -> bool {
    let mut ordering: Option<bool> = Option::None;
    let mut last_item: Option<&LevelItem> = Option::None;
    let mut skipped_one = false;

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
            if !skip_one || skipped_one {
                return false;
            } else {
                skipped_one = true;
                continue;
            }
        }

        if item.abs_diff(*last_item.unwrap()) > 3 {
            return false;
        }

        last_item = Some(item);
    }
    true
}

pub fn check_level_skip(level: &Vec<LevelItem>) -> bool {
    if check_level(level, true) {
        return true;
    }
    let mut level_copy = level.clone();
    level_copy.remove(0);
    if check_level(&level_copy, true) {
        return true;
    }
    level_copy = level.clone();
    level_copy.remove(1);
    if check_level(&level_copy, true) {
        return true;
    }
    level_copy = level.clone();
    level_copy.remove(level_copy.len() - 2);
    if check_level(&level_copy, true) {
        return true;
    }

    level_copy = level.clone();
    level_copy.pop();
    if check_level(&level_copy, true) {
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_level_move(level: Vec<LevelItem>) -> bool {
        check_level(&level, false)
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
