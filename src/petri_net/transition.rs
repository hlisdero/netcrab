use crate::petri_net::node_ref::PlaceRef;
use crate::petri_net::postset_connectable::PostsetConnectable;
use crate::petri_net::preset_connectable::PresetConnectable;
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
}

impl PresetConnectable for Transition {
    type RefType = PlaceRef;

    /// Gets an immutable reference to the set of places
    /// whose edges point to this transition.
    fn get_preset(&self) -> &BTreeSet<Self::RefType> {
        &self.preset
    }

    /// Gets a mutable reference to the set of places
    /// whose edges point to this transition.
    fn get_preset_mut(&mut self) -> &mut BTreeSet<Self::RefType> {
        &mut self.preset
    }
}

impl PostsetConnectable for Transition {
    type RefType = PlaceRef;

    /// Gets an immutable reference to the set of places
    /// to which edges from this transition point to.
    fn get_postset(&self) -> &BTreeSet<Self::RefType> {
        &self.postset
    }

    /// Gets a mutable reference to the set of places
    /// to which edges from this transition point to.
    fn get_postset_mut(&mut self) -> &mut BTreeSet<Self::RefType> {
        &mut self.postset
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
