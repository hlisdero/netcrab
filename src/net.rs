use crate::net::node_ref::{PlaceRef, TransitionRef};
use std::collections::HashMap;

mod node;
mod node_ref;

#[cfg(test)]
mod net_tests;

use crate::net::node::{Place, Transition};
#[derive(Default)]
pub struct PetriNet {
    places: HashMap<PlaceRef, Place>,
    transitions: HashMap<TransitionRef, Transition>,
}

impl PetriNet {
    pub fn get_size(&self) -> usize {
        self.places.len()
    }

    pub fn add_place(&mut self, place_label: &String) {
        self.places.insert(
            PlaceRef(place_label.clone()),
            Place::new(place_label.clone()),
        );
    }

    pub fn add_transition(&mut self, transition_label: &String) {
        self.transitions.insert(
            TransitionRef(transition_label.clone()),
            Transition::new(transition_label.clone()),
        );
    }
}
