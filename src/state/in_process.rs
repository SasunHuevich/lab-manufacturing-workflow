use super::detail_state::DetailState;
use super::finished::Finished;
use crate::state::defective::Defective;

pub struct InProcess;

impl DetailState for InProcess {
    fn name(&self) -> &'static str {
        "InProcess"
    }

    fn next(self: Box<Self>) -> Box<dyn DetailState> {
        Box::new(Finished)
    }
}

impl InProcess {
    fn mark_defective(self: Box<Self>) -> Box<dyn DetailState> {
        Box::new(Defective)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_process_name() {
        let in_process = InProcess;
        assert_eq!(in_process.name(), "InProcess");
    }

    #[test]
    fn in_process_next() {
        let in_process: Box<dyn DetailState> = Box::new(InProcess);
        let next_state = in_process.next();
        assert_eq!(next_state.name(), "Finished")
    }

    #[test]
    fn in_process_mark_defective() {
        let in_process: Box<InProcess> = Box::new(InProcess);
        let defective = in_process.mark_defective();
        assert_eq!(defective.name(), "Defective")
    }
}