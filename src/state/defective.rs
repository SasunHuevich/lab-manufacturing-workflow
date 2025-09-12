use super::detail_state::DetailState;

pub struct Defective;

impl DetailState for Defective {
    fn name(&self) -> &'static str {
        "Defective"
    }

    fn next(self: Box<Self>) -> Box<dyn DetailState> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defective_name() {
        let defective = Defective;
        assert_eq!(defective.name(), "Defective")
    }

    #[test]
    fn defective_next() {
        let defective: Box<dyn DetailState> = Box::new(Defective);
        let next_state = defective.next();
        assert_eq!(next_state.name(), "Defective");
    }
}