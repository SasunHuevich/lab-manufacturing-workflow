use super::operation::*;
use crate::strategy::strategy::Strategy;

pub struct Cutting;

impl Operation for Cutting {
    fn name(&self) -> &'static str {
        "Cutting"
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
    fn cutting_name() {
        let cutting = Cutting;
        assert_eq!(cutting.name(), "Cutting")
    }
}