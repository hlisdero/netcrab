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
