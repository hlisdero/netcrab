use crate::petri_net::node::{Place, Transition};
use crate::petri_net::node_ref::{PlaceRef, TransitionRef};
use crate::petri_net::PetriNet;
use std::collections::btree_map::Iter;

impl PetriNet {
    /// Return an iterator over the place references and their corresponding places.
    /// The places are visited in arbitrary order.
    pub fn places_iter(&self) -> Iter<PlaceRef, Place> {
        self.places.iter()
    }

    /// Return an iterator over the transition references and their corresponding transitions.
    /// The transitions are visited in arbitrary order.
    pub fn transitions_iter(&self) -> Iter<TransitionRef, Transition> {
        self.transitions.iter()
    }
}

#[cfg(test)]
mod net_iter_tests {
    use super::*;

    #[test]
    fn net_places_iter_empty_net() {
        let net = PetriNet::default();
        assert_eq!(net.places_iter().count(), 0);
    }

    #[test]
    fn net_places_iter_visits_all_places() {
        let mut net = PetriNet::default();
        net.add_place(&"P1".to_string());
        net.add_place(&"P2".to_string());
        net.add_place(&"P3".to_string());
        net.add_place(&"P4".to_string());
        assert_eq!(net.places_iter().count(), 4);
    }

    #[test]
    fn net_transitions_iter_empty_net() {
        let net = PetriNet::default();
        assert_eq!(net.transitions_iter().count(), 0);
    }

    #[test]
    fn net_transitions_iter_visits_all_transitions() {
        let mut net = PetriNet::default();
        net.add_transition(&"T1".to_string());
        net.add_transition(&"T2".to_string());
        net.add_transition(&"T3".to_string());
        net.add_transition(&"T4".to_string());
        assert_eq!(net.transitions_iter().count(), 4);
    }
}
