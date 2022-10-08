use super::*;
use crate::net::place::Place;

#[test]
fn new_default_has_no_places() {
    let net = Net::default();
    assert!(net.get_places().is_empty());
}

#[test]
fn new_default_has_no_transitions() {
    let net = Net::default();
    assert!(net.get_transitions().is_empty());
}

#[test]
fn new_default_has_no_edges() {
    let net = Net::default();
    assert!(net.get_transitions().is_empty());
}

#[test]
fn net_add_place_updates_size() {
    let mut net = Net::default();
    let place = Place::default();
    net.add_place(place);

    assert_eq!(net.get_size(), 1);
}
