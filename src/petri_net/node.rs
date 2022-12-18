pub use crate::petri_net::node::postset_connectable::PostsetConnectable;
pub use crate::petri_net::node::preset_connectable::PresetConnectable;
use crate::petri_net::node_ref::{PlaceRef, TransitionRef};
use std::collections::BTreeSet;

mod postset_connectable;
mod preset_connectable;

#[derive(Default)]
pub struct Place {
    marking: usize,
    preset: BTreeSet<TransitionRef>,
    postset: BTreeSet<TransitionRef>,
}

#[derive(Default)]
pub struct Transition {
    preset: BTreeSet<PlaceRef>,
    postset: BTreeSet<PlaceRef>,
}

impl Place {
    /// Create an empty place without tokens nor connections.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Check whether the place has zero tokens.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.marking == 0
    }

    /// Get the number of tokens at the given place.
    #[inline]
    #[must_use]
    pub const fn marking(&self) -> usize {
        self.marking
    }

    /// Add `tokens_to_add` tokens to the place.
    ///
    /// # Errors
    ///
    /// If the addition causes an overflow, then an error is returned.
    pub fn add_token(&mut self, tokens_to_add: usize) -> Result<(), &str> {
        self.marking = match self.marking.checked_add(tokens_to_add) {
            Some(value) => value,
            None => return Err("Overflow when adding tokens to this place"),
        };
        Ok(())
    }

    /// Remove `tokens_to_remove` tokens from the place.
    ///
    /// # Errors
    ///
    /// If the subtraction causes an overflow, then an error is returned.
    pub fn remove_token(&mut self, tokens_to_remove: usize) -> Result<(), &str> {
        if self.marking() < tokens_to_remove {
            return Err("Cannot remove more tokens than available at this place");
        }
        self.marking = match self.marking.checked_sub(tokens_to_remove) {
            Some(value) => value,
            None => return Err("Overflow when removing tokens from this place"),
        };
        Ok(())
    }
}

impl Transition {
    /// Create an empty transition without connections.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl PresetConnectable for Place {
    type RefType = TransitionRef;

    /// Get an immutable reference to the set of transitions
    /// whose edges point to this place.
    fn get_preset(&self) -> &BTreeSet<Self::RefType> {
        &self.preset
    }

    /// Get a mutable reference to the set of transitions
    /// whose edges point to this place.
    fn get_preset_mut(&mut self) -> &mut BTreeSet<Self::RefType> {
        &mut self.preset
    }
}

impl PostsetConnectable for Place {
    type RefType = TransitionRef;

    /// Get an immutable reference to the set of transitions
    /// to which edges from this place point to.
    fn get_postset(&self) -> &BTreeSet<Self::RefType> {
        &self.postset
    }

    /// Get a mutable reference to the set of transitions
    /// to which edges from this place point to.
    fn get_postset_mut(&mut self) -> &mut BTreeSet<Self::RefType> {
        &mut self.postset
    }
}

impl PresetConnectable for Transition {
    type RefType = PlaceRef;

    /// Get an immutable reference to the set of places
    /// whose edges point to this transition.
    fn get_preset(&self) -> &BTreeSet<Self::RefType> {
        &self.preset
    }

    /// Get a mutable reference to the set of places
    /// whose edges point to this transition.
    fn get_preset_mut(&mut self) -> &mut BTreeSet<Self::RefType> {
        &mut self.preset
    }
}

impl PostsetConnectable for Transition {
    type RefType = PlaceRef;

    /// Get an immutable reference to the set of places
    /// to which edges from this transition point to.
    fn get_postset(&self) -> &BTreeSet<Self::RefType> {
        &self.postset
    }

    /// Get a mutable reference to the set of places
    /// to which edges from this transition point to.
    fn get_postset_mut(&mut self) -> &mut BTreeSet<Self::RefType> {
        &mut self.postset
    }
}

#[cfg(test)]
mod place_tests {
    use super::*;

    #[test]
    fn place_new_has_empty_preset() {
        let place = Place::new();

        assert!(place.get_preset().is_empty());
    }

    #[test]
    fn place_new_has_empty_postset() {
        let place = Place::new();

        assert!(place.get_postset().is_empty());
    }

    #[test]
    fn place_default_is_empty() {
        let place = Place::default();

        assert!(place.is_empty());
    }

    #[test]
    fn place_default_marking_is_zero() {
        let place = Place::default();

        assert_eq!(place.marking(), 0);
    }

    #[test]
    fn place_add_token_updates_marking() {
        let mut place = Place::default();

        assert!(place.add_token(1).is_ok());
        assert!(!place.is_empty());
    }

    #[test]
    fn place_add_lots_of_tokens() {
        let mut place = Place::default();
        assert_eq!(place.marking(), 0);

        assert!(place.add_token(10).is_ok());

        assert!(!place.is_empty());
        assert_eq!(place.marking(), 10);
    }

    #[test]
    fn place_remove_token_updates_length() {
        let mut place = Place::default();

        assert!(place.add_token(1).is_ok());
        let result = place.remove_token(1);

        assert!(result.is_ok());
        assert!(place.is_empty());
    }

    #[test]
    fn place_remove_token_returns_err_if_empty() {
        let mut place = Place::default();
        let result = place.remove_token(1);

        assert!(result.is_err());
        assert!(place.is_empty());
    }

    #[test]
    fn place_remove_lots_of_tokens() {
        let mut place = Place::default();

        assert!(place.add_token(10).is_ok());
        assert!(place.remove_token(7).is_ok());

        assert!(!place.is_empty());
        assert_eq!(place.marking(), 3);
    }

    #[test]
    fn place_add_incoming_transition_returns_true_when_success() {
        let mut place = Place::new();
        let reference = TransitionRef::from("Example transition");

        assert!(place.add_incoming(reference));
    }

    #[test]
    fn place_add_incoming_transition_returns_false_when_already_exists() {
        let mut place = Place::new();
        let reference = TransitionRef::from("Example transition");

        assert!(place.add_incoming(reference));
        let reference = TransitionRef::from("Example transition");
        assert!(!place.add_incoming(reference));
    }

    #[test]
    fn place_remove_incoming_transition_returns_true_when_success() {
        let mut place = Place::new();
        let reference = TransitionRef::from("Example transition");

        assert!(place.add_incoming(reference));
        let reference = TransitionRef::from("Example transition");
        assert!(place.remove_incoming(&reference));
    }

    #[test]
    fn place_remove_incoming_transition_returns_false_when_not_found() {
        let mut place = Place::new();
        let reference = TransitionRef::from("Example transition");

        assert!(place.add_incoming(reference));
        let reference = TransitionRef::from("Example not found");
        assert!(!place.remove_incoming(&reference));
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
        let reference = PlaceRef::from("Example place");

        assert!(transition.add_incoming(reference));
    }

    #[test]
    fn transition_add_incoming_place_returns_false_when_already_exists() {
        let mut transition = Transition::new();
        let reference = PlaceRef::from("Example place");

        assert!(transition.add_incoming(reference));
        let reference = PlaceRef::from("Example place");
        assert!(!transition.add_incoming(reference));
    }

    #[test]
    fn transition_remove_incoming_place_returns_true_when_success() {
        let mut transition = Transition::new();
        let reference = PlaceRef::from("Example place");

        assert!(transition.add_incoming(reference));
        let reference = PlaceRef::from("Example place");
        assert!(transition.remove_incoming(&reference));
    }

    #[test]
    fn transition_remove_incoming_place_returns_false_when_not_found() {
        let mut transition = Transition::new();
        let reference = PlaceRef::from("Example place");

        assert!(transition.add_incoming(reference));
        let reference = PlaceRef::from("Example not found");
        assert!(!transition.remove_incoming(&reference));
    }
}
