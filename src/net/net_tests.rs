use super::*;

#[test]
fn new_default_has_no_places() {
    let net = PetriNet::default();
    assert_eq!(net.get_cardinality_places(), 0);
}

#[test]
fn new_default_has_no_transitions() {
    let new = PetriNet::default();
    assert_eq!(new.get_cardinality_transitions(), 0)
}

#[test]
fn net_add_place_updates_size() {
    let mut net = PetriNet::default();
    net.add_place(&"Example place".to_string());

    assert_eq!(net.get_cardinality_places(), 1);
}

#[test]
fn net_add_transition_updates_size() {
    let mut net = PetriNet::default();
    net.add_transition(&"Example transition".to_string());

    assert_eq!(net.get_cardinality_transitions(), 1);
}
