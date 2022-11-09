//! Module with simple functions to create nets
//!
//! This could be extended to a struct with a custom format
//! for the place labels and transition labels.
use crate::net::{PetriNet, PlaceRef, TransitionRef};

fn place_label_from_index(index: usize) -> String {
    format!("P{}", index)
}

fn transition_label_from_index(index: usize) -> String {
    format!("T{}", index)
}

/// Create a new Petri net with no arcs
/// and `number_of_places` places
/// and `number_of_transitions` transitions.
#[must_use]
pub fn create_basic_unconnected_net(
    number_of_places: usize,
    number_of_transitions: usize,
) -> PetriNet {
    let mut net = PetriNet::new();

    for i in 1..=number_of_places {
        net.add_place(&place_label_from_index(i));
    }
    for i in 1..=number_of_transitions {
        net.add_transition(&transition_label_from_index(i));
    }

    net
}

/// Create a new Petri net where the places and the transition form a simple chain.
/// The net contains `length` places and `length - 1` transitions.
#[must_use]
pub fn create_net_chain_topology(length: usize) -> PetriNet {
    if length == 0 {
        return PetriNet::new();
    }
    let mut net = create_basic_unconnected_net(length, length - 1);

    for i in 1..length {
        let place_ref = PlaceRef(place_label_from_index(i));
        let transition_ref = TransitionRef(transition_label_from_index(i));
        net.add_arc_place_transition(&place_ref, &transition_ref)
            .expect("Failed while creating a net with chain topology");
    }

    for i in 1..length {
        let transition_ref = TransitionRef(transition_label_from_index(i));
        let place_ref = PlaceRef(place_label_from_index(i + 1));
        net.add_arc_transition_place(&transition_ref, &place_ref)
            .expect("Failed while creating a net with chain topology");
    }

    net
}

/// Create a new Petri net with one place and one transition forming a loop.
#[must_use]
pub fn create_net_loop_topology() -> PetriNet {
    let mut net = PetriNet::new();
    let place_ref = net.add_place("P1");
    let transition_ref = net.add_transition("T1");

    net.add_arc_place_transition(&place_ref, &transition_ref)
        .expect("Failed while trying to create a simple net with a loop topology");
    net.add_arc_transition_place(&transition_ref, &place_ref)
        .expect("Failed while trying to create a simple net with a loop topology");

    net
}

#[cfg(test)]
mod net_creator_tests {
    use super::*;

    #[test]
    fn create_basic_unconnected_net_has_correct_number_of_nodes() {
        let net = create_basic_unconnected_net(8, 14);

        assert_eq!(net.get_cardinality_places(), 8);
        assert_eq!(net.get_cardinality_transitions(), 14);
    }

    #[test]
    fn create_basic_unconnected_net_has_correct_number_of_nodes_from_0_to_10() {
        for number_of_places in 0..=10 {
            for number_of_transitions in 0..=10 {
                let net = create_basic_unconnected_net(number_of_places, number_of_transitions);
                assert_eq!(net.get_cardinality_places(), number_of_places);
                assert_eq!(net.get_cardinality_transitions(), number_of_transitions);
            }
        }
    }

    #[test]
    fn create_basic_unconnected_net_has_no_arcs() {
        let net = create_basic_unconnected_net(8, 14);
        let arcs_1 = net.find_arcs_place_transition();
        let arcs_2 = net.find_arcs_transition_place();

        assert!(arcs_1.is_empty());
        assert!(arcs_2.is_empty());
    }

    #[test]
    fn create_basic_unconnected_net_has_no_arcs_from_0_to_10() {
        for number_of_places in 0..=10 {
            for number_of_transitions in 0..=10 {
                let net = create_basic_unconnected_net(number_of_places, number_of_transitions);
                let arcs_1 = net.find_arcs_place_transition();
                let arcs_2 = net.find_arcs_transition_place();

                assert!(arcs_1.is_empty());
                assert!(arcs_2.is_empty());
            }
        }
    }

    #[test]
    fn create_net_chain_topology_has_correct_number_of_nodes() {
        let net = create_net_chain_topology(3);

        assert_eq!(net.get_cardinality_places(), 3);
        assert_eq!(net.get_cardinality_transitions(), 2);
    }

    #[test]
    fn create_net_chain_topology_length_zero_returns_empty_net() {
        let net = create_net_chain_topology(0);

        assert_eq!(net.get_cardinality_places(), 0);
        assert_eq!(net.get_cardinality_transitions(), 0);
    }

    #[test]
    fn create_net_chain_topology_has_correct_number_of_nodes_from_1_to_10() {
        for length in 1..=10 {
            let net = create_net_chain_topology(length);

            assert_eq!(net.get_cardinality_places(), length);
            assert_eq!(net.get_cardinality_transitions(), length - 1);
        }
    }

    #[test]
    fn create_net_chain_topology_has_correct_number_of_arcs() {
        let net = create_net_chain_topology(3);
        let arcs_1 = net.find_arcs_place_transition();
        let arcs_2 = net.find_arcs_transition_place();

        assert_eq!(arcs_1.len(), 2);
        assert_eq!(arcs_2.len(), 2);
    }

    #[test]
    fn create_net_chain_topology_has_correct_number_of_arcs_from_1_to_10() {
        for length in 1..=10 {
            let net = create_net_chain_topology(length);
            let arcs_1 = net.find_arcs_place_transition();
            let arcs_2 = net.find_arcs_transition_place();

            assert_eq!(arcs_1.len(), length - 1);
            assert_eq!(arcs_2.len(), length - 1);
        }
    }

    #[test]
    fn create_net_loop_topology_has_correct_number_of_places() {
        let net = create_net_loop_topology();

        assert_eq!(net.get_cardinality_places(), 1);
        assert_eq!(net.get_cardinality_transitions(), 1);
    }

    #[test]
    fn create_net_loop_topology_has_correct_number_of_arcs() {
        let net = create_net_loop_topology();
        let arcs_1 = net.find_arcs_place_transition();
        let arcs_2 = net.find_arcs_transition_place();

        assert_eq!(arcs_1.len(), 1);
        assert_eq!(arcs_2.len(), 1);
    }
}
