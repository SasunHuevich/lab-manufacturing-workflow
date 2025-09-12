use crate::operation::operation::Operation;

pub trait Strategy {
    fn get_operations(&self) -> Vec<Box<dyn Operation>>;
}