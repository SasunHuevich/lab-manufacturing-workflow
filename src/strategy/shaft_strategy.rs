use super::strategy::Strategy;
use crate::operation::*;

pub struct ShaftStrategy;

impl Strategy for ShaftStrategy {
    fn get_operations(&self) -> Vec<Box<dyn Operation>> {
        vec![
            Box::new(Cutting),
            Box::new(Drill),
            Box::new(Grind),
            Box::new(HeatTreatment),
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shaft_strategy_operations() {
        let s = ShaftStrategy;
        let ops = s.get_operations();
        let names: Vec<_> = ops.iter().map(|o| o.name()).collect();
        assert_eq!(names, vec!["Cutting", "Drill", "Grind", "HeatTreatment"]);
    }
}