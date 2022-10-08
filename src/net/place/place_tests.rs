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
    let token = Token::default();
    place.add_token(token);
    assert!(!place.is_empty());
}

#[test]
fn place_add_lots_of_tokens() {
    let mut place = Place::default();
    let tokens = vec![Token::default(); 10];

    for t in tokens {
        place.add_token(t);
    }

    assert!(!place.is_empty());
    assert_eq!(place.get_tokens().len(), 10);
}

#[test]
fn place_remove_token_updates_length() {
    let mut place = Place::default();
    let token = Token::default();
    place.add_token(token);
    place.remove_token();
    assert!(place.is_empty());
}

#[test]
fn place_remove_token_returns_none_if_empty() {
    let mut place = Place::default();
    let result = place.remove_token();
    assert!(place.is_empty());
    assert!(result.is_none());
}

#[test]
fn place_remove_lots_of_tokens() {
    let mut place = Place::default();
    let tokens = vec![Token::default(); 10];

    for t in tokens {
        place.add_token(t);
    }

    for _ in 0..7 {
        place.remove_token();
    }

    assert!(!place.is_empty());
    assert_eq!(place.get_tokens().len(), 3);
}
