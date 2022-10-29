use crate::net::node::{ConnectableNode, Place, Transition};
use crate::net::node_ref::{PlaceRef, TransitionRef};
use std::collections::HashMap;

mod node;
mod node_ref;

#[cfg(test)]
mod net_tests;

#[derive(Default)]
pub struct PetriNet {
    places: HashMap<PlaceRef, Place>,
    transitions: HashMap<TransitionRef, Transition>,
}

impl PetriNet {
    pub fn get_cardinality_places(&self) -> usize {
        self.places.len()
    }

    pub fn get_cardinality_transitions(&self) -> usize {
        self.transitions.len()
    }

    /// Add a place to the net.
    /// If the label already exists, it silently overwrites it.
    pub fn add_place(&mut self, place_label: &String) -> PlaceRef {
        let place_ref = PlaceRef(place_label.clone());
        self.places
            .insert(place_ref.clone(), Place::new(place_label.clone()));
        place_ref
    }

    /// Add a transition to the net.
    /// If the label already exists, it silently overwrites it.
    pub fn add_transition(&mut self, transition_label: &String) -> TransitionRef {
        let transition_ref = TransitionRef(transition_label.clone());
        self.transitions.insert(
            transition_ref.clone(),
            Transition::new(transition_label.clone()),
        );
        transition_ref
    }

    pub fn add_arc_place_transition(
        &mut self,
        place_ref: &PlaceRef,
        transition_ref: &TransitionRef,
    ) -> Result<(), &str> {
        let (place, transition) = self.get_place_transition_pair(place_ref, transition_ref)?;
        // We prefer to clone the references here, since the add operations technically do not need it,
        // but we just want to borrow the references from the user for this operation.
        let inserted_outgoing = place.add_outgoing(transition_ref.clone());
        let inserted_incoming = transition.add_incoming(place_ref.clone());
        Self::check_arc_insertion(inserted_incoming, inserted_outgoing)?;
        Ok(())
    }

    pub fn add_arc_transition_place(
        &mut self,
        transition_ref: &TransitionRef,
        place_ref: &PlaceRef,
    ) -> Result<(), &str> {
        let (place, transition) = self.get_place_transition_pair(place_ref, transition_ref)?;
        // We prefer to clone the references here, since the add operations technically do not need it,
        // but we just want to borrow the references from the user for this operation.
        let inserted_outgoing = transition.add_outgoing(place_ref.clone());
        let inserted_incoming = place.add_incoming(transition_ref.clone());
        Self::check_arc_insertion(inserted_incoming, inserted_outgoing)?;
        Ok(())
    }

    pub fn check_place_ref(&self, place_ref: &PlaceRef) -> bool {
        self.places.contains_key(place_ref)
    }

    pub fn check_transition_ref(&self, transition_ref: &TransitionRef) -> bool {
        self.transitions.contains_key(transition_ref)
    }

    fn get_place_transition_pair(
        &mut self,
        place_ref: &PlaceRef,
        transition_ref: &TransitionRef,
    ) -> Result<(&mut Place, &mut Transition), &str> {
        let place = match self.places.get_mut(place_ref) {
            Some(place) => place,
            None => return Err("Place reference is invalid. It is not present in the net."),
        };

        let transition = match self.transitions.get_mut(transition_ref) {
            Some(transition) => transition,
            None => return Err("Transition reference is invalid. It is not present in the net."),
        };

        Ok((place, transition))
    }

    fn check_arc_insertion(
        inserted_incoming: bool,
        inserted_outgoing: bool,
    ) -> Result<(), &'static str> {
        if !inserted_outgoing && !inserted_incoming {
            return Err("Cannot add the arc. The arc already exists.");
        }
        if !inserted_outgoing || !inserted_incoming {
            return Err("The arc existed only in one side. The net was in an inconsistent state.");
        }
        Ok(())
    }
}
