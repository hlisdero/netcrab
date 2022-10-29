use super::*;

#[test]
fn place_new_assigns_label() {
    let label = "label";
    let place = Place::new(label.to_string());

    assert_eq!(place.label, Some(label.to_string()));
}

#[test]
fn place_default_creates_place_with_no_label() {
    let place = Place::default();

    assert_eq!(place.label, None);
}

#[test]
fn place_default_is_empty() {
    let place = Place::default();

    assert!(place.is_empty());
}

fn place_default_marking_is_zero() {
    let place = Place::default();

    assert_eq!(place.marking(), 0);
}

#[test]
fn place_add_token_updates_marking() {
    let mut place = Place::default();

    assert!(place.add_token().is_ok());
    assert!(!place.is_empty());
}

#[test]
fn place_add_lots_of_tokens() {
    let mut place = Place::default();
    assert_eq!(place.marking(), 0);

    for _ in 0..10 {
        assert!(place.add_token().is_ok());
    }

    assert!(!place.is_empty());
    assert_eq!(place.marking(), 10);
}

#[test]
fn place_remove_token_updates_length() {
    let mut place = Place::default();

    assert!(place.add_token().is_ok());
    let result = place.remove_token();

    assert!(result.is_ok());
    assert!(place.is_empty());
}

#[test]
fn place_remove_token_returns_err_if_empty() {
    let mut place = Place::default();
    let result = place.remove_token();

    assert!(result.is_err());
    assert!(place.is_empty());
}

#[test]
fn place_remove_lots_of_tokens() {
    let mut place = Place::default();

    for _ in 0..10 {
        assert!(place.add_token().is_ok());
    }

    for _ in 0..7 {
        assert!(place.remove_token().is_ok());
    }

    assert!(!place.is_empty());
    assert_eq!(place.marking(), 3);
}

#[test]
fn place_add_incoming_transition_returns_true_when_success() {
    let mut place = Place::new("Example place".to_string());
    let reference = TransitionRef("Example transition".to_string());

    assert!(place.add_incoming(reference));
}

#[test]
fn place_add_incoming_transition_returns_false_when_already_exists() {
    let mut place = Place::new("Example place".to_string());
    let reference = TransitionRef("Example transition".to_string());

    assert!(place.add_incoming(reference));
    let reference = TransitionRef("Example transition".to_string());
    assert!(!place.add_incoming(reference));
}

#[test]
fn place_remove_incoming_transition_returns_true_when_success() {
    let mut place = Place::new("Example place".to_string());
    let reference = TransitionRef("Example transition".to_string());

    assert!(place.add_incoming(reference));
    let reference = TransitionRef("Example transition".to_string());
    assert!(place.remove_incoming(&reference));
}

#[test]
fn place_remove_incoming_transition_returns_false_when_not_found() {
    let mut place = Place::new("Example place".to_string());
    let reference = TransitionRef("Example transition".to_string());

    assert!(place.add_incoming(reference));
    let reference = TransitionRef("Example not found".to_string());
    assert!(!place.remove_incoming(&reference));
}
