use super::operation::*;
use crate::strategy::strategy::Strategy;

pub struct HeatTreatment;

impl Operation for HeatTreatment {
    fn name(&self) -> &'static str {
        "HeatTreatment"
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct DummyStrategy;
    impl Strategy for DummyStrategy {
        fn get_operations(&self) -> Vec<Box<dyn Operation>> {
            vec![]
        }
    }

    #[test]
    fn heat_treatment_name() {
        let heat_treatment = HeatTreatment;
        assert_eq!(heat_treatment.name(), "HeatTreatment")
    }
}