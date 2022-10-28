mod node;

#[cfg(test)]
mod net_tests;

use crate::net::node::{Place, Transition};
#[derive(Default)]
pub struct Net {
    places: Vec<Place>,
    transition: Vec<Transition>,
}

impl Net {
    pub fn get_size(&self) -> usize {
        self.places.len()
    }

    pub fn add_place(&mut self, place: Place) {
        self.places.push(place);
    }

    pub fn add_transition(&mut self, transition: Transition) {
        self.transition.push(transition);
    }

    pub fn get_places(&self) -> &Vec<Place> {
        &self.places
    }

    pub fn get_transitions(&self) -> &Vec<Transition> {
        &self.transition
    }
}
