//! Module with simple functions to create nets
//!
//! This could be extended to a struct with a custom format
//! for the place labels and transitions label.
use crate::net::{PetriNet, PlaceRef, TransitionRef};

fn place_label_from_index(index: usize) -> String {
    format!("P{}", index)
}

fn transition_label_from_index(index: usize) -> String {
    format!("T{}", index)
}

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

pub fn create_net_chain_topology(length: usize) -> PetriNet {
    if length == 0 {
        return PetriNet::new();
    }
    let mut net = create_basic_unconnected_net(length, length - 1);

    for i in 1..length - 1 {
        let place_ref = PlaceRef(place_label_from_index(i));
        let transition_ref = TransitionRef(transition_label_from_index(i));
        net.add_arc_place_transition(&place_ref, &transition_ref)
            .expect("Failed while creating a net with chain topology");
    }

    for i in 1..length - 2 {
        let transition_ref = TransitionRef(transition_label_from_index(i));
        let place_ref = PlaceRef(place_label_from_index(i + 1));
        net.add_arc_transition_place(&transition_ref, &place_ref)
            .expect("Failed while creating a net with chain topology");
    }

    net
}

pub fn create_net_loop_topology() -> PetriNet {
    let mut net = PetriNet::new();
    let place_ref = net.add_place(&"P1".to_string());
    let transition_ref = net.add_transition(&"T1".to_string());

    net.add_arc_place_transition(&place_ref, &transition_ref)
        .expect("Failed while trying to create a simple net with a loop topology");
    net.add_arc_transition_place(&transition_ref, &place_ref)
        .expect("Failed while trying to create a simple net with a loop topology");

    net
}
