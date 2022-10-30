pub use crate::net::node::connectable::ConnectableNode;
use crate::net::node_ref::{PlaceRef, TransitionRef};
use std::collections::HashSet;

mod connectable;

#[derive(Default)]
pub struct Place {
    pub label: Option<String>,
    marking: usize,
    preset: HashSet<TransitionRef>,
    postset: HashSet<TransitionRef>,
}

#[derive(Default)]
pub struct Transition {
    pub label: Option<String>,
    preset: HashSet<PlaceRef>,
    postset: HashSet<PlaceRef>,
}

impl Place {
    pub fn new(label: String) -> Place {
        Place {
            label: Some(label),
            marking: 0,
            preset: HashSet::new(),
            postset: HashSet::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.marking == 0
    }

    pub fn marking(&self) -> usize {
        self.marking
    }

    pub fn add_token(&mut self) -> Result<(), &str> {
        self.marking = match self.marking.checked_add(1) {
            Some(value) => value,
            None => return Err("Overflow when adding token"),
        };
        Ok(())
    }

    pub fn remove_token(&mut self) -> Result<(), &str> {
        if self.is_empty() {
            return Err("Cannot remove token from empty place");
        }
        self.marking -= 1;
        Ok(())
    }
}

impl Transition {
    pub fn new(label: String) -> Transition {
        Transition {
            label: Some(label),
            preset: HashSet::new(),
            postset: HashSet::new(),
        }
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
    fn place_new_assigns_label() {
        let label = "label";
        let place = Place::new(label.to_string());

        assert_eq!(place.label, Some(label.to_string()));
    }

    #[test]
    fn place_default_creates_place_with_no_label() {
        let place = Place::default();

        assert_eq!(place.label, None);
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

        assert!(place.add_token().is_ok());
        assert!(!place.is_empty());
    }

    #[test]
    fn place_add_lots_of_tokens() {
        let mut place = Place::default();
        assert_eq!(place.marking(), 0);

        for _ in 0..10 {
            assert!(place.add_token().is_ok());
        }

        assert!(!place.is_empty());
        assert_eq!(place.marking(), 10);
    }

    #[test]
    fn place_remove_token_updates_length() {
        let mut place = Place::default();

        assert!(place.add_token().is_ok());
        let result = place.remove_token();

        assert!(result.is_ok());
        assert!(place.is_empty());
    }

    #[test]
    fn place_remove_token_returns_err_if_empty() {
        let mut place = Place::default();
        let result = place.remove_token();

        assert!(result.is_err());
        assert!(place.is_empty());
    }

    #[test]
    fn place_remove_lots_of_tokens() {
        let mut place = Place::default();

        for _ in 0..10 {
            assert!(place.add_token().is_ok());
        }

        for _ in 0..7 {
            assert!(place.remove_token().is_ok());
        }

        assert!(!place.is_empty());
        assert_eq!(place.marking(), 3);
    }

    #[test]
    fn place_add_incoming_transition_returns_true_when_success() {
        let mut place = Place::new("Example place".to_string());
        let reference = TransitionRef("Example transition".to_string());

        assert!(place.add_incoming(reference));
    }

    #[test]
    fn place_add_incoming_transition_returns_false_when_already_exists() {
        let mut place = Place::new("Example place".to_string());
        let reference = TransitionRef("Example transition".to_string());

        assert!(place.add_incoming(reference));
        let reference = TransitionRef("Example transition".to_string());
        assert!(!place.add_incoming(reference));
    }

    #[test]
    fn place_remove_incoming_transition_returns_true_when_success() {
        let mut place = Place::new("Example place".to_string());
        let reference = TransitionRef("Example transition".to_string());

        assert!(place.add_incoming(reference));
        let reference = TransitionRef("Example transition".to_string());
        assert!(place.remove_incoming(&reference));
    }

    #[test]
    fn place_remove_incoming_transition_returns_false_when_not_found() {
        let mut place = Place::new("Example place".to_string());
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
    fn transition_new_assigns_label() {
        let label = "label";
        let transition = Transition::new(label.to_string());

        assert_eq!(transition.label, Some(label.to_string()));
    }

    #[test]
    fn transition_default_creates_transition_with_no_label() {
        let transition = Transition::default();
        assert_eq!(transition.label, None);
    }

    #[test]
    fn transition_add_incoming_place_returns_true_when_success() {
        let mut transition = Transition::new("Example transition".to_string());
        let reference = PlaceRef("Example place".to_string());

        assert!(transition.add_incoming(reference));
    }

    #[test]
    fn transition_add_incoming_place_returns_false_when_already_exists() {
        let mut transition = Transition::new("Example transition".to_string());
        let reference = PlaceRef("Example place".to_string());

        assert!(transition.add_incoming(reference));
        let reference = PlaceRef("Example place".to_string());
        assert!(!transition.add_incoming(reference));
    }

    #[test]
    fn transition_remove_incoming_place_returns_true_when_success() {
        let mut transition = Transition::new("Example transition".to_string());
        let reference = PlaceRef("Example place".to_string());

        assert!(transition.add_incoming(reference));
        let reference = PlaceRef("Example place".to_string());
        assert!(transition.remove_incoming(&reference));
    }

    #[test]
    fn transition_remove_incoming_place_returns_false_when_not_found() {
        let mut transition = Transition::new("Example transition".to_string());
        let reference = PlaceRef("Example place".to_string());

        assert!(transition.add_incoming(reference));
        let reference = PlaceRef("Example not found".to_string());
        assert!(!transition.remove_incoming(&reference));
    }
}
