#[cfg(test)]
mod place_tests;

use crate::net::token::Token;

pub struct Place {
    pub label: Option<String>,
    tokens: Vec<Token>,
}

impl Place {
    pub fn new(label: String) -> Place {
        Place {
            label: Some(label),
            tokens: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn remove_token(&mut self) -> Option<Token> {
        self.tokens.pop()
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}

impl Default for Place {
    fn default() -> Self {
        Self {
            label: None,
            tokens: Vec::new(),
        }
    }
}
