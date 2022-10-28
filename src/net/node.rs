use crate::net::node_ref::{PlaceRef, TransitionRef};
use std::collections::HashSet;
#[cfg(test)]
mod place_tests;
#[cfg(test)]
mod transition_tests;

pub struct Place {
    pub label: Option<String>,
    pub marking: usize,
    pub preset: HashSet<TransitionRef>,
    pub postset: HashSet<TransitionRef>,
}

pub struct Transition {
    pub label: Option<String>,
    pub preset: HashSet<PlaceRef>,
    pub postset: HashSet<PlaceRef>,
}

impl Place {
    pub fn new(label: String) -> Place {
        Place {
            label: Some(label),
            marking: 0,
            preset: HashSet::new(),
            postset: HashSet::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.marking == 0
    }

    pub fn add_token(&mut self) {
        self.marking += 1
    }

    pub fn remove_token(&mut self) -> Result<(), &str> {
        if self.is_empty() {
            return Err("Cannot remove token from empty place");
        }
        self.marking -= 1;
        Ok(())
    }
}

impl Transition {
    pub fn new(label: String) -> Transition {
        Transition {
            label: Some(label),
            preset: HashSet::new(),
            postset: HashSet::new(),
        }
    }
}

impl Default for Place {
    fn default() -> Self {
        Self {
            label: None,
            marking: 0,
            preset: HashSet::new(),
            postset: HashSet::new(),
        }
    }
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            label: None,
            preset: HashSet::new(),
            postset: HashSet::new(),
        }
    }
}
