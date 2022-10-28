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

#[test]
fn place_add_token_updates_length() {
    let mut place = Place::default();

    place.add_token();

    assert!(!place.is_empty());
}

#[test]
fn place_add_lots_of_tokens() {
    let mut place = Place::default();

    for _ in 0..10 {
        place.add_token();
    }

    assert!(!place.is_empty());
    assert_eq!(place.marking, 10);
}

#[test]
fn place_remove_token_updates_length() {
    let mut place = Place::default();

    place.add_token();
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
        place.add_token();
    }

    for _ in 0..7 {
        assert!(place.remove_token().is_ok());
    }

    assert!(!place.is_empty());
    assert_eq!(place.marking, 3);
}
