use super::detail_state::DetailState;
use crate::state::Defective;

pub struct Finished;

impl DetailState for Finished {
    fn name(&self) -> &'static str {
        "Finished"
    }

    fn next(self: Box<Self>) -> Box<dyn DetailState> {
        self
    }

    fn mark_defective(self: Box<Self>) -> Box<dyn DetailState> {
        Box::new(Defective)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finished_name() {
        let finished = Finished;
        assert_eq!(finished.name(), "Finished")
    }

    #[test]
    fn finished_next() {
        let finished: Box<dyn DetailState> = Box::new(Finished);
        let next_state = finished.next();
        assert_eq!(next_state.name(), "Finished");
    }
}