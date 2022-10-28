use super::*;

#[test]
fn transition_new_assigns_label() {
    let label = "label";
    let transition = Transition::new(label.to_string());

    assert_eq!(transition.label, Some(label.to_string()));
}

#[test]
fn transition_default_creates_transition_with_no_label() {
    let transition = Transition::default();
    assert_eq!(transition.label, None);
}

#[test]
fn transition_add_incoming_place_returns_true_when_success() {
    let mut transition = Transition::new("Example transition".to_string());
    let reference = PlaceRef("Example place".to_string());

    assert!(transition.add_incoming(reference));
}

#[test]
fn transition_add_incoming_place_returns_false_when_already_exists() {
    let mut transition = Transition::new("Example transition".to_string());
    let reference = PlaceRef("Example place".to_string());

    assert!(transition.add_incoming(reference));
    let reference = PlaceRef("Example place".to_string());
    assert!(!transition.add_incoming(reference));
}

#[test]
fn transition_remove_incoming_place_returns_true_when_success() {
    let mut transition = Transition::new("Example transition".to_string());
    let reference = PlaceRef("Example place".to_string());

    assert!(transition.add_incoming(reference));
    let reference = PlaceRef("Example place".to_string());
    assert!(transition.remove_incoming(&reference));
}

#[test]
fn transition_remove_incoming_place_returns_false_when_not_found() {
    let mut transition = Transition::new("Example transition".to_string());
    let reference = PlaceRef("Example place".to_string());

    assert!(transition.add_incoming(reference));
    let reference = PlaceRef("Example not found".to_string());
    assert!(!transition.remove_incoming(&reference));
}
