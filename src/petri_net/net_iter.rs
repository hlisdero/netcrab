use crate::petri_net::place::Place;
use crate::petri_net::place_ref::PlaceRef;
use crate::petri_net::transition::Transition;
use crate::petri_net::transition_ref::TransitionRef;
use crate::petri_net::PetriNet;
use std::collections::btree_map::Iter;

impl PetriNet {
    /// Returns an iterator over the place references and their corresponding places.
    /// The places are visited in alphabetical order.
    pub fn places_iter(&self) -> Iter<PlaceRef, Place> {
        self.places.iter()
    }

    /// Returns an iterator over the transition references and their corresponding transitions.
    /// The transitions are visited in alphabetical order.
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
        net.add_place("P1");
        net.add_place("P2");
        net.add_place("P3");
        net.add_place("P4");
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
        net.add_transition("T1");
        net.add_transition("T2");
        net.add_transition("T3");
        net.add_transition("T4");
        assert_eq!(net.transitions_iter().count(), 4);
    }
}
