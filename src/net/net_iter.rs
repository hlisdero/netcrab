use crate::net::node::{Place, Transition};
use crate::net::node_ref::{PlaceRef, TransitionRef};
use crate::net::PetriNet;
use std::collections::hash_map::Iter;

#[cfg(test)]
mod net_iter_tests;

impl PetriNet {
    /// Return an iterator over the place references and their corresponding places.
    #[must_use]
    pub fn places_iter(&self) -> Iter<PlaceRef, Place> {
        self.places.iter()
    }

    /// Return an iterator over the transition references and their corresponding transitions.
    #[must_use]
    pub fn transitions_iter(&self) -> Iter<TransitionRef, Transition> {
        self.transitions.iter()
    }
}
