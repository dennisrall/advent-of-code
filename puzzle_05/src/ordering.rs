pub type OrderItem = usize;

#[derive(Debug)]
pub struct OrderingRule {
    pub before: OrderItem,
    pub after: OrderItem,
}

impl OrderingRule {
    pub fn from_str(s: &str) -> Option<OrderingRule> {
        let mut parts = s.split("|");

        let before = parts.next()?.parse().ok()?;
        let after = parts.next()?.parse().ok()?;

        Some(OrderingRule { before, after })
    }

    pub fn is_fullfilled(&self, sequence: &Vec<OrderItem>) -> bool {
        match (
            sequence.iter().position(|&item| item == self.before),
            sequence.iter().position(|&item| item == self.after),
        ) {
            (Some(before), Some(after)) => before < after,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let result = OrderingRule::from_str("425|27").unwrap();

        assert_eq!(result.before, 425);
        assert_eq!(result.after, 27);
    }

    #[test]
    fn test_is_fullfilled_empty() {
        let rule = OrderingRule::from_str("42|27").unwrap();
        let result = rule.is_fullfilled(&vec![]);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_fullfilled_both_not_present() {
        let rule = OrderingRule::from_str("42|27").unwrap();
        let result = rule.is_fullfilled(&vec![1, 2, 3, 4]);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_fullfilled_before_not_present() {
        let rule = OrderingRule::from_str("42|27").unwrap();
        let result = rule.is_fullfilled(&vec![100, 27, 13]);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_fullfilled_after_not_present() {
        let rule = OrderingRule::from_str("42|27").unwrap();
        let result = rule.is_fullfilled(&vec![10, 42, 13]);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_fullfilled_correct() {
        let rule = OrderingRule::from_str("42|27").unwrap();
        let result = rule.is_fullfilled(&vec![10, 42, 13, 27]);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_fullfilled_false() {
        let rule = OrderingRule::from_str("42|27").unwrap();
        let result = rule.is_fullfilled(&vec![27, 10, 42, 13]);
        assert_eq!(result, false);
    }
}
