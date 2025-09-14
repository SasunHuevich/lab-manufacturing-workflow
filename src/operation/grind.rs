use super::operation::*;
use crate::strategy::strategy::Strategy;

pub struct Grind;

impl Operation for Grind {
    fn name(&self) -> &'static str {
        "Grind"
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
    fn grind_name() {
        let grind = Grind;
        assert_eq!(grind.name(), "Grind")
    }
}