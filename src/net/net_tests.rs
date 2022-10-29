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
fn net_marking_returns_number_of_tokens_in_place() {
    let mut net = PetriNet::default();
    let place_ref = net.add_place(&"Example place".to_string());

    let result = net.marking(&place_ref);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn net_marking_add_token_increases_number_of_tokens_in_place() {
    let mut net = PetriNet::default();
    let place_ref = net.add_place(&"Example place".to_string());

    assert!(net.add_token(&place_ref).is_ok());

    let result = net.marking(&place_ref);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn net_marking_add_token_multiple_times() {
    let mut net = PetriNet::default();
    let place_ref = net.add_place(&"Example place".to_string());

    assert!(net.add_token(&place_ref).is_ok());
    assert!(net.add_token(&place_ref).is_ok());
    assert!(net.add_token(&place_ref).is_ok());
    assert!(net.add_token(&place_ref).is_ok());
    assert!(net.add_token(&place_ref).is_ok());

    let result = net.marking(&place_ref);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5);
}

#[test]
fn net_marking_remove_token_decreases_number_of_tokens_in_place() {
    let mut net = PetriNet::default();
    let place_ref = net.add_place(&"Example place".to_string());

    assert!(net.add_token(&place_ref).is_ok());
    assert!(net.add_token(&place_ref).is_ok());
    assert!(net.remove_token(&place_ref).is_ok());

    let result = net.marking(&place_ref);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

#[test]
fn net_marking_remove_token_returns_err_if_place_empty() {
    let mut net = PetriNet::default();
    let place_ref = net.add_place(&"Example place".to_string());

    assert!(net.remove_token(&place_ref).is_err());
}

#[test]
fn net_marking_vector_three_empty_places() {
    let mut net = PetriNet::default();
    let place_1 = net.add_place(&"P1".to_string());
    let place_2 = net.add_place(&"P2".to_string());
    let place_3 = net.add_place(&"P3".to_string());

    let result = net.marking_vector();
    assert_eq!(*result.get(&place_1).unwrap(), 0);
    assert_eq!(*result.get(&place_2).unwrap(), 0);
    assert_eq!(*result.get(&place_3).unwrap(), 0);
}

#[test]
fn net_marking_vector_two_places() {
    let mut net = PetriNet::default();
    let place_1 = net.add_place(&"P1".to_string());
    let place_2 = net.add_place(&"P2".to_string());

    assert!(net.add_token(&place_1).is_ok());
    assert!(net.add_token(&place_1).is_ok());
    assert!(net.add_token(&place_1).is_ok());
    assert!(net.add_token(&place_1).is_ok());
    assert!(net.add_token(&place_1).is_ok());

    assert!(net.add_token(&place_2).is_ok());
    assert!(net.add_token(&place_2).is_ok());
    assert!(net.add_token(&place_2).is_ok());

    let result = net.marking_vector();
    assert_eq!(*result.get(&place_1).unwrap(), 5);
    assert_eq!(*result.get(&place_2).unwrap(), 3);
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

#[test]
fn net_find_unconnected_places_returns_empty_set_for_empty_net() {
    let net = PetriNet::default();
    let unconnected_set = net.find_unconnected_places();

    assert!(unconnected_set.is_empty());
}

#[test]
fn net_find_unconnected_places_some_empty_places() {
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

    let unconnected_set = net.find_unconnected_places();
    assert_eq!(unconnected_set.len(), 1);
    assert!(unconnected_set.contains(&place_3));
}

#[test]
fn net_find_unconnected_places_all_empty_places() {
    let mut net = PetriNet::default();
    let place_1 = net.add_place(&"P1".to_string());
    let place_2 = net.add_place(&"P2".to_string());
    let place_3 = net.add_place(&"P3".to_string());

    let unconnected_set = net.find_unconnected_places();
    assert_eq!(unconnected_set.len(), 3);
    assert!(unconnected_set.contains(&place_1));
    assert!(unconnected_set.contains(&place_2));
    assert!(unconnected_set.contains(&place_3));
}
