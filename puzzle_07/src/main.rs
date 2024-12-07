use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type CalculationType = isize;

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut score = 0;
    for line in reader.lines().filter_map(|l| l.ok()) {
        let mut parts = line.split(':');
        let result = parts.next().unwrap().parse::<CalculationType>().unwrap();
        let rest = parts.next().unwrap().trim();
        let operands: Vec<CalculationType> = rest
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let first_operand = operands.first().unwrap();
        if could_be_calculated(&result, &operands[1..], *first_operand) {
            score += result;
        }
    }
    println!("Sum of all possible calculations: {}", score);

    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut score = 0;
    for line in reader.lines().filter_map(|l| l.ok()) {
        let mut parts = line.split(':');
        let result = parts.next().unwrap().parse::<CalculationType>().unwrap();
        let rest = parts.next().unwrap().trim();
        let operands: Vec<CalculationType> = rest
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let first_operand = operands.first().unwrap();
        if could_be_calculated_with_concat(&result, &operands[1..], *first_operand) {
            score += result;
        }
    }
    println!("Sum of all possible calculations with concat: {}", score);
}

fn could_be_calculated(
    result: &CalculationType,
    operands: &[CalculationType],
    cur_result: CalculationType,
) -> bool {
    if operands.is_empty() {
        cur_result == *result
    } else if cur_result > *result {
        false
    } else {
        let operator = operands.first().unwrap();
        could_be_calculated(result, &operands[1..], cur_result + operator)
            || could_be_calculated(result, &operands[1..], cur_result * operator)
    }
}

fn concat_vals(op1: &CalculationType, op2: &CalculationType) -> CalculationType {
    (op1.to_string() + &op2.to_string()).parse().unwrap()
}

fn could_be_calculated_with_concat(
    result: &CalculationType,
    operands: &[CalculationType],
    cur_result: CalculationType,
) -> bool {
    if operands.is_empty() {
        cur_result == *result
    } else if cur_result > *result {
        false
    } else {
        let operator = operands.first().unwrap();
        could_be_calculated_with_concat(result, &operands[1..], cur_result + operator)
            || could_be_calculated_with_concat(result, &operands[1..], cur_result * operator)
            || could_be_calculated_with_concat(
                result,
                &operands[1..],
                concat_vals(&cur_result, operator),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let mut score = 0;

        for line in input.lines() {
            let mut parts = line.split(':');
            let result = parts.next().unwrap().parse::<CalculationType>().unwrap();
            let rest = parts.next().unwrap().trim();
            let operands: Vec<CalculationType> = rest
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let first_operand = operands.first().unwrap();
            if could_be_calculated(&result, &operands[1..], *first_operand) {
                score += result;
            }
        }
        assert_eq!(score, 3749);
    }

    #[test]
    fn test_example_1() {
        let result = could_be_calculated(&190, &vec![19], 10);
        assert!(result);
    }

    #[test]
    fn test_example_2() {
        let result = could_be_calculated(&3267, &vec![40, 27], 81);
        assert!(result);
    }

    #[test]
    fn test_example_3() {
        let result = could_be_calculated(&292, &vec![6, 16, 20], 11);
        assert!(result);
    }

    #[test]
    fn test_concat_vals() {
        let result = concat_vals(&11, &2);
        assert_eq!(result, 112);
    }

    #[test]
    fn test_concat_example_1() {
        let result = could_be_calculated_with_concat(&156, &vec![6], 15);
        assert!(result);
    }

    #[test]
    fn test_concat_example_2() {
        let result = could_be_calculated_with_concat(&7290, &vec![8, 6, 15], 6);
        assert!(result);
    }

    #[test]
    fn test_main() {
        let reader = BufReader::new(File::open("input.txt").unwrap());
        let mut score = 0;
        for line in reader.lines().filter_map(|l| l.ok()) {
            let mut parts = line.split(':');
            let result = parts.next().unwrap().parse::<CalculationType>().unwrap();
            let rest = parts.next().unwrap().trim();
            let operands: Vec<CalculationType> = rest
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let first_operand = operands.first().unwrap();
            if could_be_calculated(&result, &operands[1..], *first_operand) {
                score += result;
            }
        }
        assert_eq!(score, 42283209483350);

        let reader = BufReader::new(File::open("input.txt").unwrap());
        let mut score = 0;
        for line in reader.lines().filter_map(|l| l.ok()) {
            let mut parts = line.split(':');
            let result = parts.next().unwrap().parse::<CalculationType>().unwrap();
            let rest = parts.next().unwrap().trim();
            let operands: Vec<CalculationType> = rest
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let first_operand = operands.first().unwrap();
            if could_be_calculated_with_concat(&result, &operands[1..], *first_operand) {
                score += result;
            }
        }
        assert_eq!(score, 1026766857276279);
    }
}
