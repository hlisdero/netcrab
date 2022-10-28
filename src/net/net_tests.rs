use super::*;

#[test]
fn new_default_has_no_places() {
    let net = PetriNet::default();
    assert_eq!(net.get_size(), 0);
}

#[test]
fn net_add_place_updates_size() {
    let mut net = PetriNet::default();
    net.add_place(&"Example place".to_string());

    assert_eq!(net.get_size(), 1);
}
