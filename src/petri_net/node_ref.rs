#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct PlaceRef(pub String);

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct TransitionRef(pub String);

impl PlaceRef {
    /// Convert the reference to the underlying `String`.
    #[inline]
    #[must_use]
    pub const fn as_string(&self) -> &String {
        &self.0
    }
}

impl TransitionRef {
    /// Convert the reference to the underlying `String`.
    #[inline]
    #[must_use]
    pub const fn as_string(&self) -> &String {
        &self.0
    }
}

impl std::fmt::Display for PlaceRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for TransitionRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

    #[test]
    fn place_ref_display_trait_prints_the_inner_string() {
        let place_ref = PlaceRef("Example reference".to_string());

        assert_eq!(format!("{place_ref}"), "Example reference");
    }

    #[test]
    fn transition_ref_display_trait_prints_the_inner_string() {
        let transition_ref = TransitionRef("Example reference".to_string());

        assert_eq!(format!("{transition_ref}"), "Example reference");
    }
}
