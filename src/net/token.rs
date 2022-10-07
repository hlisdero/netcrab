#[cfg(test)]
mod token_tests;

#[derive(Clone)]
pub struct Token {
    pub label: Option<String>,
}

impl Token {
    pub fn new(label: String) -> Token {
        Token { label: Some(label) }
    }
}

impl Default for Token {
    fn default() -> Self {
        Self { label: None }
    }
}
