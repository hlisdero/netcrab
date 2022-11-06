#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct PlaceRef(pub String);

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct TransitionRef(pub String);

impl PlaceRef {
    pub fn as_string(&self) -> &String {
        &self.0
    }
}

impl TransitionRef {
    pub fn as_string(&self) -> &String {
        &self.0
    }
}

#[cfg(test)]
mod net_tests {
    use super::*;

    #[test]
    fn place_ref_as_string_returns_the_inner_string() {
        let place_ref = PlaceRef("Example reference".to_string());

        assert_eq!(place_ref.as_string(), "Example reference");
    }

    #[test]
    fn transition_ref_as_string_returns_the_inner_string() {
        let transition_ref = TransitionRef("Example reference".to_string());

        assert_eq!(transition_ref.as_string(), "Example reference");
    }
}
