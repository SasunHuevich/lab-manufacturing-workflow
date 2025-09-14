use super::strategy::Strategy;
use crate::operation::*;

pub struct BoltStrategy;

impl Strategy for BoltStrategy {
    fn get_operations(&self) -> Vec<Box<dyn Operation>> {
        vec![
            Box::new(Threading),
            Box::new(Polish),
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bolt_strategy_operations() {
        let s = BoltStrategy;
        let ops = s.get_operations();
        let names: Vec<_> = ops.iter().map(|o| o.name()).collect();
        assert_eq!(names, vec!["Threading", "Polish"]);
    }
}