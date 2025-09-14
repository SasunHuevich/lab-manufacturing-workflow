use super::strategy::Strategy;
use crate::operation::*;

pub struct PinStrategy;

impl Strategy for PinStrategy {
    fn get_operations(&self) -> Vec<Box<dyn Operation>> {
        vec![
            Box::new(Cutting),
            Box::new(Drill),
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pin_strategy_operations() {
        let s = PinStrategy;
        let ops = s.get_operations();
        let names: Vec<_> = ops.iter().map(|o| o.name()).collect();
        assert_eq!(names, vec!["Cutting", "Drill"]);
    }
}