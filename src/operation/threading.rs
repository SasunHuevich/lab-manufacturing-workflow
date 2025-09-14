use super::operation::*;
use crate::strategy::strategy::Strategy;

pub struct Threading;

impl Operation for Threading {
    fn name(&self) -> &'static str {
        "Threading"
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
    fn threading_name() {
        let threading = Threading;
        assert_eq!(threading.name(), "Threading")
    }
}