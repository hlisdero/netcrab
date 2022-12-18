#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct PlaceRef(String);

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct TransitionRef(String);

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

impl From<String> for PlaceRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<String> for TransitionRef {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for PlaceRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<&str> for TransitionRef {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[cfg(test)]
mod net_tests {
    use super::*;

    #[test]
    fn place_ref_as_string_returns_the_inner_string() {
        let place_ref = PlaceRef::from("Example reference");

        assert_eq!(place_ref.as_string(), "Example reference");
    }

    #[test]
    fn transition_ref_as_string_returns_the_inner_string() {
        let transition_ref = TransitionRef::from("Example reference");

        assert_eq!(transition_ref.as_string(), "Example reference");
    }

    #[test]
    fn place_ref_display_trait_prints_the_inner_string() {
        let place_ref = PlaceRef::from("Example reference");

        assert_eq!(format!("{place_ref}"), "Example reference");
    }

    #[test]
    fn transition_ref_display_trait_prints_the_inner_string() {
        let transition_ref = TransitionRef::from("Example reference");

        assert_eq!(format!("{transition_ref}"), "Example reference");
    }

    #[test]
    fn place_ref_creates_ref_from_str() {
        let place_ref = PlaceRef::from("Example reference");

        assert_eq!(place_ref.0, "Example reference");
    }

    #[test]
    fn transition_ref_creates_ref_from_str() {
        let transition_ref = PlaceRef::from("Example reference");

        assert_eq!(transition_ref.0, "Example reference");
    }

    #[test]
    fn place_ref_creates_ref_from_string() {
        let string = String::from("Example reference");
        let place_ref = PlaceRef::from(string);

        assert_eq!(place_ref.0, "Example reference");
    }

    #[test]
    fn transition_ref_creates_ref_from_string() {
        let string = String::from("Example reference");
        let transition_ref = PlaceRef::from(string);

        assert_eq!(transition_ref.0, "Example reference");
    }
}
