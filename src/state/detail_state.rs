pub trait DetailState {
    fn name(&self) -> &'static str;
    fn next(self: Box<Self>) -> Box<dyn DetailState>;
    fn mark_defective(self: Box<Self>) -> Box<dyn DetailState>;
}