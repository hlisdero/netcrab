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
    /// Creates a new place reference with a randomly generated UUID.
    #[must_use]
    pub fn new(place_label: &str) -> Self {
        Self {
            label: place_label.to_string(),
            uuid: Uuid::new_v4(),
        }
    }

    /// Returns the place label for this reference.
    #[inline]
    #[must_use]
    pub const fn label(&self) -> &String {
        &self.label
    }
}

impl TransitionRef {
    /// Creates a new transition reference with a randomly generated UUID.
    #[must_use]
    pub fn new(transition_label: &str) -> Self {
        Self {
            label: transition_label.to_string(),
            uuid: Uuid::new_v4(),
        }
    }

    /// Returns the transition label for this reference.
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

#[cfg(test)]
mod net_tests {
    use super::*;

    #[test]
    fn place_ref_new_sets_label() {
        let place_ref = PlaceRef::new("Example reference");

        assert_eq!(place_ref.label, "Example reference");
    }

    #[test]
    fn transition_ref_new_sets_label() {
        let transition_ref = TransitionRef::new("Example reference");

        assert_eq!(transition_ref.label, "Example reference");
    }

    #[test]
    fn place_ref_new_sets_uuid_not_nil() {
        let place_ref = PlaceRef::new("Example reference");

        assert_ne!(
            place_ref.uuid,
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()
        );
    }

    #[test]
    fn transition_ref_new_sets_uuid_not_nil() {
        let transition_ref = TransitionRef::new("Example reference");

        assert_ne!(
            transition_ref.uuid,
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()
        );
    }

    #[test]
    fn place_ref_new_sets_a_different_uuid_every_time() {
        let ref_1 = PlaceRef::new("Example reference");
        let ref_2 = PlaceRef::new("Example reference");
        let ref_3 = PlaceRef::new("Example reference");

        assert_ne!(ref_1.uuid, ref_2.uuid);
        assert_ne!(ref_2.uuid, ref_3.uuid);
        assert_ne!(ref_3.uuid, ref_1.uuid);
    }

    #[test]
    fn transition_ref_new_sets_a_different_uuid_every_time() {
        let ref_1 = TransitionRef::new("Example reference");
        let ref_2 = TransitionRef::new("Example reference");
        let ref_3 = TransitionRef::new("Example reference");

        assert_ne!(ref_1.uuid, ref_2.uuid);
        assert_ne!(ref_2.uuid, ref_3.uuid);
        assert_ne!(ref_3.uuid, ref_1.uuid);
    }

    #[test]
    fn place_ref_label_returns_the_label() {
        let place_ref = PlaceRef::new("Example reference");

        assert_eq!(place_ref.label(), "Example reference");
    }

    #[test]
    fn transition_ref_label_returns_the_label() {
        let transition_ref = TransitionRef::new("Example reference");

        assert_eq!(transition_ref.label(), "Example reference");
    }

    #[test]
    fn place_ref_display_trait_prints_the_label() {
        let place_ref = PlaceRef::new("Example reference");

        assert_eq!(format!("{place_ref}"), "Example reference");
    }

    #[test]
    fn transition_ref_display_trait_prints_the_label() {
        let transition_ref = TransitionRef::new("Example reference");

        assert_eq!(format!("{transition_ref}"), "Example reference");
    }
}
