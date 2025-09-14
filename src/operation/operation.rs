use crate::detail::Detail;
use rand::Rng;

#[derive(Debug, PartialEq, Eq)]
pub enum OperationResult {
    Success,
    Failure,
}

pub trait Operation {
    fn name(&self) -> &'static str;
    
    fn execute(&self, detail: &mut Detail) -> OperationResult {
        println!("Executing {}", self.name());
        let chance = rand::thread_rng().gen_range(1..=100);

        if chance < 5 {
            let defective_state = detail.mark_defective();
            OperationResult::Failure
        } else {
            OperationResult::Success
        }  
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::detail::{Detail, DetailType};
    use crate::strategy::strategy::Strategy;

    struct DummyStrategy;
    impl Strategy for DummyStrategy {
        fn get_operations(&self) -> Vec<Box<dyn Operation>> {
            vec![]
        }
    }

    struct DummyOperation;
    impl Operation for DummyOperation {
        fn name(&self) -> &'static str {
            "DummyOperation"
        }
    }

    #[test]
    fn execute_default() {
        let strategy = Box::new(DummyStrategy);
        let mut detail = Detail::new(DetailType::Gear, strategy);
        detail.next_state();

        let op = DummyOperation;
        let result = op.execute(&mut detail);

        match result {
            OperationResult::Success => assert_eq!(detail.state_name(), "InProcess"),
            OperationResult::Failure => assert_eq!(detail.state_name(), "Defective"),
        }
    }
}