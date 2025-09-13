use uuid::Uuid;
use crate::state::detail_state::DetailState;
use std::collections::HashMap;
use crate::strategy::strategy::Strategy;
use crate::state::*;
use crate::operation::operation::Operation;

#[derive(Debug, PartialEq, Eq)]
pub enum DetailType {
    Gear,
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

    fn get_state(&self) -> &mut dyn DetailState {
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

    #[test]
    fn record_and_get_history() {
        let strategy = Box::new(DummyStrategy);
        let mut detail = Detail::new(DetailType::Plate, strategy);
        detail.record_operation("Drill");
        detail.record_operation("Grind");
        let history = detail.get_history();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0], "Drill");
        assert_eq!(history[1], "Grind");
    }

    #[test]
    fn set_and_get_state() {
        let strategy = Box::new(DummyStrategy);
        let mut detail = Detail::new(DetailType::Pin, strategy);
        let new_state: Box<dyn DetailState> = Box::new(InProcess);
        detail.set_state(new_state);
        assert_eq!(detail.get_state().name(), "InProcess");
    }

    #[test]
    fn set_and_get_params() {
        let strategy = Box::new(DummyStrategy);
        let mut detail = Detail::new(DetailType::Shaft, strategy);
        detail.set_params("length", "100");
        detail.set_params("width", "50");
        assert_eq!(detail.get_params("length").unwrap(), "100");
        assert_eq!(detail.get_params("width").unwrap(), "50");
        assert!(detail.get_params("height").is_none());
    }
}