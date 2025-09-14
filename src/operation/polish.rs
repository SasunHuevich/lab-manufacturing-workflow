use super::operation::*;
use crate::strategy::strategy::Strategy;

pub struct Polish;

impl Operation for Polish {
    fn name(&self) -> &'static str {
        "Polish"
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
    fn polish_name() {
        let polish = Polish;
        assert_eq!(polish.name(), "Polish")
    }
}