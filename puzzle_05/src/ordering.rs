pub type OrderItem = usize;

#[derive(Debug)]
pub struct OrderingRule {
    pub before: OrderItem,
    pub after: OrderItem,
}

impl OrderingRule {
    pub fn from_str(s: &str) -> Option<OrderingRule> {
        let mut parts = s.split('|');

        let before = parts.next()?.parse().ok()?;
        let after = parts.next()?.parse().ok()?;

        Some(OrderingRule { before, after })
    }

    pub fn is_fullfilled(&self, sequence: &[OrderItem]) -> bool {
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
        let result = rule.is_fullfilled(&[]);
        assert!(result);
    }

    #[test]
    fn test_is_fullfilled_both_not_present() {
        let rule = OrderingRule::from_str("42|27").unwrap();
        let result = rule.is_fullfilled(&[1, 2, 3, 4]);
        assert!(result);
    }

    #[test]
    fn test_is_fullfilled_before_not_present() {
        let rule = OrderingRule::from_str("42|27").unwrap();
        let result = rule.is_fullfilled(&[100, 27, 13]);
        assert!(result);
    }

    #[test]
    fn test_is_fullfilled_after_not_present() {
        let rule = OrderingRule::from_str("42|27").unwrap();
        let result = rule.is_fullfilled(&[10, 42, 13]);
        assert!(result);
    }

    #[test]
    fn test_is_fullfilled_correct() {
        let rule = OrderingRule::from_str("42|27").unwrap();
        let result = rule.is_fullfilled(&[10, 42, 13, 27]);
        assert!(result);
    }

    #[test]
    fn test_is_fullfilled_false() {
        let rule = OrderingRule::from_str("42|27").unwrap();
        let result = rule.is_fullfilled(&[27, 10, 42, 13]);
        assert!(!result);
    }
}
