use crate::net::PetriNet;

pub fn create_net_chain_topology() -> PetriNet {
    let mut net = PetriNet::new();
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
    net
}

pub fn create_net_loop_topology() -> PetriNet {
    let mut net = PetriNet::new();
    let place_ref = net.add_place(&"P1".to_string());
    let transition_ref = net.add_transition(&"T1".to_string());

    let result = net.add_arc_place_transition(&place_ref, &transition_ref);
    assert!(result.is_ok());
    let result = net.add_arc_transition_place(&transition_ref, &place_ref);
    assert!(result.is_ok());
    net
}
