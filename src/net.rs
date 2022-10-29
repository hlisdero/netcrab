use crate::net::node::{Place, Transition};
use crate::net::node_ref::{PlaceRef, TransitionRef};
use std::collections::HashMap;

mod node;
mod node_ref;

#[cfg(test)]
mod net_tests;

#[derive(Default)]
pub struct PetriNet {
    places: HashMap<PlaceRef, Place>,
    transitions: HashMap<TransitionRef, Transition>,
}

impl PetriNet {
    pub fn get_cardinality_places(&self) -> usize {
        self.places.len()
    }

    pub fn get_cardinality_transitions(&self) -> usize {
        self.transitions.len()
    }

    /// Add a place to the net.
    /// If the label already exists, it silently overwrites it.
    pub fn add_place(&mut self, place_label: &String) {
        self.places.insert(
            PlaceRef(place_label.clone()),
            Place::new(place_label.clone()),
        );
    }

    /// Add a transition to the net.
    /// If the label already exists, it silently overwrites it.
    pub fn add_transition(&mut self, transition_label: &String) {
        self.transitions.insert(
            TransitionRef(transition_label.clone()),
            Transition::new(transition_label.clone()),
        );
    }

    pub fn check_place_ref(&self, place_ref: &PlaceRef) -> bool {
        self.places.contains_key(place_ref)
    }

    pub fn check_transition_ref(&self, transition_ref: &TransitionRef) -> bool {
        self.transitions.contains_key(transition_ref)
    }
}
