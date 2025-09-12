use crate::detail::Detail;

pub enum OperationResult {
    Success,
    Failure,
}

pub trait Operation {
    fn name(&self) -> &'static str;
    fn execute(&self, detail: &mut Detail) -> OperationResult;
}