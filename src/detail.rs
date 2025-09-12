use uuid::Uuid;
use crate::state::detail_state::DetailState;
use std::collections::HashMap;
use crate::strategy::strategy::Strategy;
use crate::state::raw::Raw;
use crate::operation::operation::Operation;

#[derive(Debug, PartialEq, Eq)]
pub enum DetailType {
    Geat,
    Shaft,
    Plate,
    Bolt,
    Pin,
}

pub struct Detail {
    id: Uuid,
    detail_type: DetailType,
    state: Box<dyn DetailState>,
    history: Vec<String>,
    params: HashMap<String, String>,
    strategy: Box<dyn Strategy>,
}

impl Detail {
    fn new(detail_type: DetailType, strategy: Box<dyn Strategy>) -> Self {
        Self {
            id: Uuid::new_v4(),
            detail_type,
            state: Box::new(Raw),
            history: Vec::new(),
            params: HashMap::new(),
            strategy,
        }
    }

    fn record_operation(&mut self, operation_name: &str) {
        self.history.push(operation_name.to_string());
    }

    fn get_history(&self) -> &Vec<String>{
       &self.history 
    }

    fn set_state(&mut self, state: Box<dyn DetailState>) {
        self.state = state;
    }

    fn get_state(&self) -> &dyn DetailState {
        &*self.state
    }

    fn set_params(&mut self, key: &str, value: &str) {
        self.params.insert(key.to_string(), value.to_string());
    }

    fn get_params(&self, key: &str) -> Option<&String> {
        self.params.get(key)
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
    fn new_detail() {
        let strategy = Box::new(DummyStrategy);
        let detail = Detail::new(DetailType::Bolt, strategy);
        assert_eq!(detail.get_state().name(), "Raw");
        assert!(detail.get_history().is_empty());
        assert_eq!(detail.detail_type, DetailType::Bolt);
    }
}