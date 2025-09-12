pub trait DetailState {
    fn name(&self) -> &'static str;
    fn next(self: Box<Self>) -> Box<dyn DetailState>;
}