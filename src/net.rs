mod edge;
mod place;
mod transition;

#[cfg(test)]
mod net_tests;

use crate::net::edge::Edge;
use crate::net::place::Place;
use crate::net::transition::Transition;

#[derive(Default)]
pub struct Net {
    places: Vec<Place>,
    transition: Vec<Transition>,
    edges: Vec<Edge>,
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

    pub fn get_edges(&self) -> &Vec<Edge> {
        &self.edges
    }
}
