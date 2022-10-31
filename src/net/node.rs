pub use crate::net::node::connectable::ConnectableNode;
use crate::net::node_ref::{PlaceRef, TransitionRef};
use std::collections::HashSet;

mod connectable;

#[derive(Default)]
pub struct Place {
    marking: usize,
    preset: HashSet<TransitionRef>,
    postset: HashSet<TransitionRef>,
}

#[derive(Default)]
pub struct Transition {
    preset: HashSet<PlaceRef>,
    postset: HashSet<PlaceRef>,
}

impl Place {
    pub fn new() -> Place {
        Place::default()
    }

    pub fn is_empty(&self) -> bool {
        self.marking == 0
    }

    pub fn marking(&self) -> usize {
        self.marking
    }

    pub fn add_token(&mut self, tokens_to_add: usize) -> Result<(), &str> {
        self.marking = match self.marking.checked_add(tokens_to_add) {
            Some(value) => value,
            None => return Err("Overflow when adding tokens to this place"),
        };
        Ok(())
    }

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
    pub fn new() -> Transition {
        Transition::default()
    }
}

impl ConnectableNode for Place {
    type RefType = TransitionRef;

    fn get_preset(&self) -> &HashSet<Self::RefType> {
        &self.preset
    }

    fn get_postset(&self) -> &HashSet<Self::RefType> {
        &self.postset
    }

    fn get_preset_mut(&mut self) -> &mut HashSet<Self::RefType> {
        &mut self.preset
    }

    fn get_postset_mut(&mut self) -> &mut HashSet<Self::RefType> {
        &mut self.postset
    }
}

impl ConnectableNode for Transition {
    type RefType = PlaceRef;

    fn get_preset(&self) -> &HashSet<Self::RefType> {
        &self.preset
    }

    fn get_postset(&self) -> &HashSet<Self::RefType> {
        &self.postset
    }

    fn get_preset_mut(&mut self) -> &mut HashSet<Self::RefType> {
        &mut self.preset
    }

    fn get_postset_mut(&mut self) -> &mut HashSet<Self::RefType> {
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
        let reference = TransitionRef("Example transition".to_string());

        assert!(place.add_incoming(reference));
    }

    #[test]
    fn place_add_incoming_transition_returns_false_when_already_exists() {
        let mut place = Place::new();
        let reference = TransitionRef("Example transition".to_string());

        assert!(place.add_incoming(reference));
        let reference = TransitionRef("Example transition".to_string());
        assert!(!place.add_incoming(reference));
    }

    #[test]
    fn place_remove_incoming_transition_returns_true_when_success() {
        let mut place = Place::new();
        let reference = TransitionRef("Example transition".to_string());

        assert!(place.add_incoming(reference));
        let reference = TransitionRef("Example transition".to_string());
        assert!(place.remove_incoming(&reference));
    }

    #[test]
    fn place_remove_incoming_transition_returns_false_when_not_found() {
        let mut place = Place::new();
        let reference = TransitionRef("Example transition".to_string());

        assert!(place.add_incoming(reference));
        let reference = TransitionRef("Example not found".to_string());
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
        let reference = PlaceRef("Example place".to_string());

        assert!(transition.add_incoming(reference));
    }

    #[test]
    fn transition_add_incoming_place_returns_false_when_already_exists() {
        let mut transition = Transition::new();
        let reference = PlaceRef("Example place".to_string());

        assert!(transition.add_incoming(reference));
        let reference = PlaceRef("Example place".to_string());
        assert!(!transition.add_incoming(reference));
    }

    #[test]
    fn transition_remove_incoming_place_returns_true_when_success() {
        let mut transition = Transition::new();
        let reference = PlaceRef("Example place".to_string());

        assert!(transition.add_incoming(reference));
        let reference = PlaceRef("Example place".to_string());
        assert!(transition.remove_incoming(&reference));
    }

    #[test]
    fn transition_remove_incoming_place_returns_false_when_not_found() {
        let mut transition = Transition::new();
        let reference = PlaceRef("Example place".to_string());

        assert!(transition.add_incoming(reference));
        let reference = PlaceRef("Example not found".to_string());
        assert!(!transition.remove_incoming(&reference));
    }
}
