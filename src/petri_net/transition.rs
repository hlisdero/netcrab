use crate::petri_net::node_ref::PlaceRef;
use std::collections::BTreeSet;

#[derive(Default)]
pub struct Transition {
    preset: BTreeSet<PlaceRef>,
    postset: BTreeSet<PlaceRef>,
}

impl Transition {
    /// Creates an empty transition without connections.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets an immutable reference to the set of places
    /// whose edges point to this transition.
    pub fn get_preset(&self) -> &BTreeSet<PlaceRef> {
        &self.preset
    }

    /// Gets a mutable reference to the set of places
    /// whose edges point to this transition.
    pub fn get_preset_mut(&mut self) -> &mut BTreeSet<PlaceRef> {
        &mut self.preset
    }

    /// Gets an immutable reference to the set of places
    /// to which edges from this transition point to.
    pub fn get_postset(&self) -> &BTreeSet<PlaceRef> {
        &self.postset
    }

    /// Gets a mutable reference to the set of places
    /// to which edges from this transition point to.
    pub fn get_postset_mut(&mut self) -> &mut BTreeSet<PlaceRef> {
        &mut self.postset
    }

    /// Adds an incoming `Place`, update the preset accordingly.
    pub fn add_incoming(&mut self, reference: PlaceRef) -> bool {
        self.preset.insert(reference)
    }

    /// Removes an incoming `Place`, update the preset accordingly.
    pub fn remove_incoming(&mut self, reference: &PlaceRef) -> bool {
        self.preset.remove(reference)
    }

    /// Adds an outgoing `Place`, update the postset accordingly.
    pub fn add_outgoing(&mut self, reference: PlaceRef) -> bool {
        self.postset.insert(reference)
    }

    /// Removes an outgoing `Place`, update the postset accordingly.
    pub fn remove_outgoing(&mut self, reference: &PlaceRef) -> bool {
        self.postset.remove(reference)
    }
}

#[cfg(test)]
mod transition_tests {
    use super::*;

    #[test]
    fn transition_new_has_empty_preset() {
        let transition = Transition::new();

        assert!(transition.get_preset().is_empty());
    }

    #[test]
    fn transition_new_has_empty_postset() {
        let transition = Transition::new();

        assert!(transition.get_postset().is_empty());
    }

    #[test]
    fn transition_add_incoming_place_returns_true_when_success() {
        let mut transition = Transition::new();
        let reference = PlaceRef::new("Example place");

        assert!(transition.add_incoming(reference));
    }

    #[test]
    fn transition_add_incoming_place_returns_false_when_already_exists() {
        let mut transition = Transition::new();
        let reference = PlaceRef::new("Example place");

        assert!(transition.add_incoming(reference.clone()));
        assert!(!transition.add_incoming(reference));
    }

    #[test]
    fn transition_remove_incoming_place_returns_true_when_success() {
        let mut transition = Transition::new();
        let reference = PlaceRef::new("Example place");

        assert!(transition.add_incoming(reference.clone()));
        assert!(transition.remove_incoming(&reference));
    }

    #[test]
    fn transition_remove_incoming_place_returns_false_when_not_found() {
        let mut transition = Transition::new();
        let reference = PlaceRef::new("Example place");

        assert!(transition.add_incoming(reference));
        let reference = PlaceRef::new("Example not found");
        assert!(!transition.remove_incoming(&reference));
    }
}
