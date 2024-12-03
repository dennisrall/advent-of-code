use regex::Regex;

pub fn parse_mul(s: &str) -> usize {
    let re = Regex::new(r"mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)").unwrap();

    re.captures_iter(s)
        .map(|cap| {
            let left = cap.name("left").unwrap().as_str().parse::<usize>().unwrap();
            let right = cap.name("right").unwrap().as_str().parse::<usize>().unwrap();
            left * right
        })
        .sum::<usize>()
}

pub fn parse_mul_disable(s: &str) -> usize {
    let mut enabled = true;
    let mut score = 0;
    let mut parts = s.split("do");

    if let Some(p) = parts.next() {
        score += parse_mul(p);

        for part in parts {
            if part.starts_with("n't()") {
                enabled = false;
            }
            if part.starts_with("()") {
                enabled = true;
            }
            if enabled {
                score += parse_mul(part);
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let result =
            parse_mul("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, 161);
    }

    #[test]
    fn example_1() {
        let result = parse_mul("mul(44,46)");
        assert_eq!(result, 2024);
    }

    #[test]
    fn example_2() {
        let result = parse_mul("mul(123,4)");
        assert_eq!(result, 123 * 4);
    }

    #[test]
    fn example_3() {
        let result = parse_mul("mul( 1, 4 )");
        assert_eq!(result, 0);
    }

    #[test]
    fn example_4() {
        let result = parse_mul("mul(4*");
        assert_eq!(result, 0);
    }

    #[test]
    fn disabled_example_1() {
        let result = parse_mul_disable(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(result, 48);
    }
    #[test]
    fn disabled_example_2() {
        let result = parse_mul_disable("don't()mul(2,3)");
        assert_eq!(result, 0);
    }

    #[test]
    fn disabled_example_3() {
        let result = parse_mul_disable("mul(123,4)");
        assert_eq!(result, 123 * 4);
    }

    #[test]
    fn disabled_example_4() {
        let result = parse_mul_disable("don't()mul(1,4)do()mul(2,3)");
        assert_eq!(result, 6);
    }

    #[test]
    fn disabled_example_5() {
        let result = parse_mul_disable("don't()do()mul(1,4)do()mul(2,3)");
        assert_eq!(result, 10);
    }

    #[test]
    fn disabled_example_6() {
        let result = parse_mul_disable("n't()mul(1,4)do()mul(2,3)");
        assert_eq!(result, 10);
    }

    #[test]
    fn disabled_example_7() {
        let result = parse_mul_disable("do()mul(1,1)");
        assert_eq!(result, 1);
    }
    #[test]
    fn disabled_example_8() {
        let result = parse_mul_disable("don't()mul(1,1)");
        assert_eq!(result, 0);
    }

    #[test]
    fn disabled_example_9() {
        let result = parse_mul_disable("dodon't()do()domul(1,1)");
        assert_eq!(result, 1);
    }

    #[test]
    fn disabled_example_10() {
        let result = parse_mul_disable("don't()mul(do1,1)");
        assert_eq!(result, 0);
    }
}
