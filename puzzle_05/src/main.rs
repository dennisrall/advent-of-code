use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

use ordering::{OrderItem, OrderingRule};

mod ordering;

fn main() {
    let rules: Vec<_> = BufReader::new(File::open("rules.txt").unwrap())
        .lines()
        .map_while(Result::ok)
        .filter_map(|l| OrderingRule::from_str(&l))
        .collect();

    let sum_middle_items_correct: OrderItem = BufReader::new(File::open("sequences.txt").unwrap())
        .lines()
        .map_while(Result::ok)
        .map(|s| {
            s.split(',')
                .filter_map(|v| v.parse::<OrderItem>().ok())
                .collect::<Vec<_>>()
        })
        .filter(|s| rules.iter().all(|r| r.is_fullfilled(s)))
        .map(|s| *s.get(s.len() / 2).unwrap())
        .sum();
    println!("sum: {}", sum_middle_items_correct);

    let sum_middle_items_reordered: OrderItem =
        BufReader::new(File::open("sequences.txt").unwrap())
            .lines()
            .map_while(Result::ok)
            .map(create_sequence)
            .filter(|s| rules.iter().any(|r| !r.is_fullfilled(s)))
            .map(|s| order_correctly(&s, &rules))
            .map(|s| *s.get(s.len() / 2).unwrap())
            .sum();

    println!("sum: {}", sum_middle_items_reordered);
}

fn create_sequence(s: String) -> Vec<OrderItem> {
    s.split(',')
        .filter_map(|v| v.parse::<OrderItem>().ok())
        .collect::<Vec<_>>()
}

fn order_correctly(seq: &[OrderItem], rules: &[OrderingRule]) -> Vec<OrderItem> {
    let compare_by_rules = |a: &OrderItem, b: &OrderItem| {
        if rules
            .iter()
            .any(|r| r.before == *a && r.after == *b)
        {
            Ordering::Less
        } else if rules
            .iter()
            .any(|r| r.before == *b && r.after == *a)
        {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    let mut s = seq.to_vec();

    s.sort_by(compare_by_rules);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
            let rules: Vec<_> = BufReader::new(File::open("rules.txt").unwrap())
        .lines()
        .map_while(Result::ok)
        .filter_map(|l| OrderingRule::from_str(&l))
        .collect();

    let sum_middle_items_correct: OrderItem = BufReader::new(File::open("sequences.txt").unwrap())
        .lines()
        .map_while(Result::ok)
        .map(|s| {
            s.split(',')
                .filter_map(|v| v.parse::<OrderItem>().ok())
                .collect::<Vec<_>>()
        })
        .filter(|s| rules.iter().all(|r| r.is_fullfilled(s)))
        .map(|s| *s.get(s.len() / 2).unwrap())
        .sum();
        assert_eq!(sum_middle_items_correct, 5991);

    let sum_middle_items_reordered: OrderItem =
        BufReader::new(File::open("sequences.txt").unwrap())
            .lines()
            .map_while(Result::ok)
            .map(create_sequence)
            .filter(|s| rules.iter().any(|r| !r.is_fullfilled(s)))
            .map(|s| order_correctly(&s, &rules))
            .map(|s| *s.get(s.len() / 2).unwrap())
            .sum();

        assert_eq!(sum_middle_items_reordered, 5479);

    }
}
