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

#[test]
fn net_add_arc_place_transition_simple() {
    let mut net = PetriNet::default();
    let place_ref = net.add_place(&"Example place".to_string());
    let transition_ref = net.add_transition(&"Example transition".to_string());

    let result = net.add_arc_place_transition(&place_ref, &transition_ref);
    assert!(result.is_ok());
}

#[test]
fn net_add_arc_transition_place_simple() {
    let mut net = PetriNet::default();
    let place_ref = net.add_place(&"Example place".to_string());
    let transition_ref = net.add_transition(&"Example transition".to_string());

    let result = net.add_arc_transition_place(&transition_ref, &place_ref);
    assert!(result.is_ok());
}

#[test]
fn net_add_place_transition_form_self_loop() {
    let mut net = PetriNet::default();
    let place_ref = net.add_place(&"Example place".to_string());
    let transition_ref = net.add_transition(&"Example transition".to_string());

    let result = net.add_arc_place_transition(&place_ref, &transition_ref);
    assert!(result.is_ok());
    let result = net.add_arc_transition_place(&transition_ref, &place_ref);
    assert!(result.is_ok());
}

#[test]
fn net_add_place_transition_form_chain() {
    let mut net = PetriNet::default();
    let place_1 = net.add_place(&"P1".to_string());
    let place_2 = net.add_place(&"P2".to_string());
    let place_3 = net.add_place(&"P3".to_string());

    let transition_1 = net.add_transition(&"T1".to_string());
    let transition_2 = net.add_transition(&"T2".to_string());

    let result = net.add_arc_place_transition(&place_1, &transition_1);
    assert!(result.is_ok());
    let result = net.add_arc_transition_place(&transition_1, &place_2);
    assert!(result.is_ok());
    let result = net.add_arc_place_transition(&place_2, &transition_2);
    assert!(result.is_ok());
    let result = net.add_arc_transition_place(&transition_2, &place_3);
    assert!(result.is_ok());
}
