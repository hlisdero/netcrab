use crate::net::node::connectable::Connectable;
use crate::net::node_ref::{PlaceRef, TransitionRef};
use std::collections::HashSet;

mod connectable;
#[cfg(test)]
mod place_tests;
#[cfg(test)]
mod transition_tests;

#[derive(Default)]
pub struct Place {
    pub label: Option<String>,
    pub marking: usize,
    pub preset: HashSet<TransitionRef>,
    pub postset: HashSet<TransitionRef>,
}

#[derive(Default)]
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
        self.marking += 1;
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

impl Connectable for Place {
    type RefType = TransitionRef;

    fn get_preset(&mut self) -> &mut HashSet<Self::RefType> {
        &mut self.preset
    }

    fn get_postset(&mut self) -> &mut HashSet<Self::RefType> {
        &mut self.postset
    }
}

impl Connectable for Transition {
    type RefType = PlaceRef;

    fn get_preset(&mut self) -> &mut HashSet<Self::RefType> {
        &mut self.preset
    }

    fn get_postset(&mut self) -> &mut HashSet<Self::RefType> {
        &mut self.postset
    }
}
