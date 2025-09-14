use super::detail_state::DetailState;
use super::in_process::InProcess;
use super::defective::Defective;

pub struct Raw;

impl DetailState for Raw {
    fn name(&self) -> &'static str {
        "Raw"
    }

    fn next(self: Box<Self>) -> Box<dyn DetailState> {
        Box::new(InProcess)
    }

    fn mark_defective(self: Box<Self>) -> Box<dyn DetailState> {
        Box::new(Defective)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_name() {
        let raw = Raw;
        assert_eq!(raw.name(), "Raw");
    }

    #[test]
    fn raw_next() {
        let raw: Box<dyn DetailState> = Box::new(Raw);
        let next = raw.next();
        assert_eq!(next.name(), "InProcess");
    }

    #[test]
    fn raw_mark_defective() {
        let raw: Box<Raw> = Box::new(Raw);
        let defective = raw.mark_defective();
        assert_eq!(defective.name(), "Defective")
    }
}