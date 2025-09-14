use super::operation::*;
use crate::strategy::strategy::Strategy;

pub struct Drill;

impl Operation for Drill {
    fn name(&self) -> &'static str {
        "Drill"
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
    fn drill_name() {
        let drill = Drill;
        assert_eq!(drill.name(), "Drill")
    }
}