use super::*;

#[test]
fn token_new_assigns_label() {
    let label = "label";
    let token = Token::new(label.to_string());

    assert_eq!(token.label, Some(label.to_string()));
}

#[test]
fn token_default_creates_token_with_no_label() {
    let token = Token::default();
    assert_eq!(token.label, None);
}
