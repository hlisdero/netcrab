#[cfg(test)]
mod place_tests;

use crate::net::token::Token;

#[derive(Default)]
pub struct Place {
    tokens: Vec<Token>,
}

impl Place {
    pub fn new() -> Place {
        Place::default()
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
