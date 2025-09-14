use super::strategy::Strategy;
use crate::operation::*;

pub struct GearStrategy;

impl Strategy for GearStrategy {
    fn get_operations(&self) -> Vec<Box<dyn Operation>> {
        vec![
            Box::new(Cutting),
            Box::new(Drill),
            Box::new(Grind),
            Box::new(HeatTreatment),
            Box::new(Polish),
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gear_strategy_operations() {
        let s = GearStrategy;
        let ops = s.get_operations();
        let names: Vec<_> = ops.iter().map(|o| o.name()).collect();
        assert_eq!(names, vec!["Cutting", "Drill", "Grind", "HeatTreatment", "Polish"]);
    }
}