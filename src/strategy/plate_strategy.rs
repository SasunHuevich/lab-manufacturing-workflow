use super::strategy::Strategy;
use crate::operation::*;

pub struct PlateStrategy;

impl Strategy for PlateStrategy {
    fn get_operations(&self) -> Vec<Box<dyn Operation>> {
        vec![
            Box::new(Cutting),
            Box::new(HeatTreatment),
            Box::new(Polish),
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plate_strategy_operations() {
        let s = PlateStrategy;
        let ops = s.get_operations();
        let names: Vec<_> = ops.iter().map(|o| o.name()).collect();
        assert_eq!(names, vec!["Cutting", "HeatTreatment", "Polish"]);
    }
}