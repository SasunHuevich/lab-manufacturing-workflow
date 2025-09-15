use crate::detail::{Detail, DetailType};
use crate::operation::operation::{Operation, OperationResult};
use crate::strategy::strategy::Strategy;
use crate::state::*;

pub struct Executor;

impl Executor {
    pub fn execute(&self, detail: &mut Detail, operation: &dyn Operation) -> OperationResult {
        if detail.state_name() == "Raw" {
            detail.next_state();
        }

        let result = operation.execute(detail);

        if result == OperationResult::Success {
            let expected: Vec<String> = detail
            .get_strategy()
            .get_operations()
            .iter()
            .map(|op| op.name().to_string())
            .collect();

            let history = detail.get_history();

            if history.len() == expected.len() && history == &expected {
                detail.next_state();
            }
        }

        result
    }

    pub fn run_strategy(&self, detail: &mut Detail) -> OperationResult {
        if detail.state_name() == "Raw" {
            detail.next_state();
        }

        for op in detail.get_strategy().get_operations() {
            let result = self.execute(detail, op.as_ref());

            if result == OperationResult::Failure {
                return OperationResult::Failure;
            }
        }

        detail.next_state();
        OperationResult::Success
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct DummyFirstOperation;

    impl Operation for DummyFirstOperation {
        fn name(&self) -> &'static str {
            "DummyFirstOperation"
        }
    }

    struct DummySecondOperation;

    impl Operation for DummySecondOperation {
        fn name(&self) -> &'static str {
            "DummySecondOperation"
        }
    }

    struct DummyStrategy;

    impl Strategy for DummyStrategy {
        fn get_operations(&self) -> Vec<Box<dyn Operation>> {
            vec![
                Box::new(DummyFirstOperation),
                Box::new(DummySecondOperation),
            ]
        }
    }


    #[test]
    fn execute_single_operation() {
        let strategy = Box::new(DummyStrategy);
        let mut detail = Detail::new(DetailType::Bolt, strategy);
        let executor = Executor;
        let operation = DummyFirstOperation;

        let result = executor.execute(&mut detail, &operation);

        assert_eq!(detail.get_history().len(), 1);
        assert_eq!(detail.get_history()[0], "DummyFirstOperation");

        match result {
            OperationResult::Success => {
                let state = detail.state_name();
                
                assert!(state == "InProcess" || state == "Finished");
            }
            OperationResult::Failure => assert_eq!(detail.state_name(), "Defective"),
        }
    }

    #[test]
    fn execute_all_operations() {
        let strategy = Box::new(DummyStrategy);
        let mut detail = Detail::new(DetailType::Bolt, strategy);
        let executor = Executor;
        let first_operation = DummyFirstOperation;
        let second_operation = DummySecondOperation;

        let result = executor.execute(&mut detail, &first_operation);
        let result = executor.execute(&mut detail, &second_operation);

        assert_eq!(detail.get_history().len(), 2);

        assert_eq!(detail.get_history()[0], "DummyFirstOperation");
        assert_eq!(detail.get_history()[1], "DummySecondOperation");

        match result {
            OperationResult::Success => assert_eq!(detail.state_name(), "Finished"),
            OperationResult::Failure => assert_eq!(detail.state_name(), "Defective"),
        }
    }

    #[test]
    fn run_strategy() {
        let strategy = Box::new(DummyStrategy);
        let mut detail = Detail::new(DetailType::Gear, strategy);
        let executor = Executor;

        let result = executor.run_strategy(&mut detail);

        match result {
            OperationResult::Success => {
                assert_eq!(detail.get_history().len(), 2);

                assert_eq!(detail.get_history()[0], "DummyFirstOperation");
                assert_eq!(detail.get_history()[1], "DummySecondOperation");
                
                assert_eq!(detail.state_name(), "Finished");
            },
            OperationResult::Failure => assert_eq!(detail.state_name(), "Defective"),
        }
    }
}