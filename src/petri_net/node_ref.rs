use uuid::Uuid;

/// A reference to a `Place` in the Petri net.
///
/// Contains a label and a UUID.
/// The ordering is done first by the label and then by the UUID.
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct PlaceRef {
    label: String,
    uuid: Uuid,
}

/// A reference to a `Transition` in the Petri net.
///
/// Contains a label and a UUID.
/// The ordering is done first by the label and then by the UUID.
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct TransitionRef {
    label: String,
    uuid: Uuid,
}

impl PlaceRef {
    /// Convert the reference to the underlying `String`.
    #[inline]
    #[must_use]
    pub const fn label(&self) -> &String {
        &self.label
    }
}

impl TransitionRef {
    /// Convert the reference to the underlying `String`.
    #[inline]
    #[must_use]
    pub const fn label(&self) -> &String {
        &self.label
    }
}

impl std::fmt::Display for PlaceRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl std::fmt::Display for TransitionRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

impl From<String> for PlaceRef {
    fn from(value: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            label: value,
        }
    }
}

impl From<String> for TransitionRef {
    fn from(value: String) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            label: value,
        }
    }
}

impl From<&str> for PlaceRef {
    fn from(value: &str) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            label: value.to_string(),
        }
    }
}

impl From<&str> for TransitionRef {
    fn from(value: &str) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            label: value.to_string(),
        }
    }
}

#[cfg(test)]
mod net_tests {
    use super::*;

    #[test]
    fn place_ref_label_returns_the_label() {
        let place_ref = PlaceRef::from("Example reference");

        assert_eq!(place_ref.label(), "Example reference");
    }

    #[test]
    fn transition_ref_label_returns_the_label() {
        let transition_ref = TransitionRef::from("Example reference");

        assert_eq!(transition_ref.label(), "Example reference");
    }

    #[test]
    fn place_ref_display_trait_prints_the_label() {
        let place_ref = PlaceRef::from("Example reference");

        assert_eq!(format!("{place_ref}"), "Example reference");
    }

    #[test]
    fn transition_ref_display_trait_prints_the_label() {
        let transition_ref = TransitionRef::from("Example reference");

        assert_eq!(format!("{transition_ref}"), "Example reference");
    }

    #[test]
    fn place_ref_creates_ref_from_str() {
        let place_ref = PlaceRef::from("Example reference");

        assert_eq!(place_ref.label, "Example reference");
    }

    #[test]
    fn transition_ref_creates_ref_from_str() {
        let transition_ref = PlaceRef::from("Example reference");

        assert_eq!(transition_ref.label, "Example reference");
    }

    #[test]
    fn place_ref_creates_ref_from_string() {
        let string = String::from("Example reference");
        let place_ref = PlaceRef::from(string);

        assert_eq!(place_ref.label, "Example reference");
    }

    #[test]
    fn transition_ref_creates_ref_from_string() {
        let string = String::from("Example reference");
        let transition_ref = PlaceRef::from(string);

        assert_eq!(transition_ref.label, "Example reference");
    }
}
