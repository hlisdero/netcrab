use crate::net::node::{ConnectableNode, Place, Transition};
use crate::net::node_ref::{PlaceRef, TransitionRef};
use std::collections::{HashMap, HashSet};

mod net_iter;
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
    /// Get the number of places in the net.
    #[must_use]
    pub fn get_cardinality_places(&self) -> usize {
        self.places.len()
    }

    /// Get the number of transitions in the net.
    #[must_use]
    pub fn get_cardinality_transitions(&self) -> usize {
        self.transitions.len()
    }

    /// Check if the place reference is valid for this net,
    /// i.e. if the referenced place still exists in the net.
    #[must_use]
    pub fn check_place_ref(&self, place_ref: &PlaceRef) -> bool {
        self.places.contains_key(place_ref)
    }

    /// Check if the transition reference is valid for this net,
    /// i.e. if the referenced transition still exists in the net.
    #[must_use]
    pub fn check_transition_ref(&self, transition_ref: &TransitionRef) -> bool {
        self.transitions.contains_key(transition_ref)
    }

    /// Find unconnected places in the net.
    /// Return a `HashSet` with the place references as keys.
    #[must_use]
    pub fn find_unconnected_places(&self) -> HashSet<PlaceRef> {
        let mut unconnected_set: HashSet<PlaceRef> = HashSet::new();
        for (place_ref, place) in &self.places {
            if place.get_preset().is_empty() && place.get_postset().is_empty() {
                unconnected_set.insert(place_ref.clone());
            }
        }
        unconnected_set
    }

    /// Add a place to the net.
    /// If the label already exists, it silently overwrites it.
    pub fn add_place(&mut self, place_label: &str) -> PlaceRef {
        let place_ref = PlaceRef(place_label.to_string());
        self.places
            .insert(place_ref.clone(), Place::new(place_label.to_string()));
        place_ref
    }

    /// Add a transition to the net.
    /// If the label already exists, it silently overwrites it.
    pub fn add_transition(&mut self, transition_label: &str) -> TransitionRef {
        let transition_ref = TransitionRef(transition_label.to_string());
        self.transitions.insert(
            transition_ref.clone(),
            Transition::new(transition_label.to_string()),
        );
        transition_ref
    }

    /// Add an arc from a place to a transition with multiplicity one.
    ///
    /// # Errors
    ///
    /// If the `PlaceRef` or the `TransitionRef` is invalid, then an error is returned.
    /// If the arc already exists, then an error is returned.
    /// If the arc was added already on one side but not on the other, then an error is returned.
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

    /// Add an arc from a transition to a place with multiplicity one.
    ///
    /// # Errors
    ///
    /// If the `PlaceRef` or the `TransitionRef` is invalid, then an error is returned.
    /// If the arc already exists, then an error is returned.
    /// If the arc was added already on one side but not on the other, then an error is returned.
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

    /// Get number of tokens in a place in the net.
    ///
    /// # Errors
    ///
    /// If the `PlaceRef` is invalid, then an error is returned.
    pub fn marking(&mut self, place_ref: &PlaceRef) -> Result<usize, &str> {
        let place = self.get_place(place_ref)?;
        Ok(place.marking())
    }

    /// Get the marking vector for the net.
    /// Returns a `HashMap` with the place references as the keys and the number of tokens as values.
    pub fn marking_vector(&mut self) -> HashMap<PlaceRef, usize> {
        let mut marking_vector: HashMap<PlaceRef, usize> = HashMap::new();
        for (key, value) in &self.places {
            marking_vector.insert(key.clone(), value.marking());
        }
        marking_vector
    }

    /// Add one token to a place in the net.
    ///
    /// # Errors
    ///
    /// If the `PlaceRef` is invalid, then an error is returned.
    pub fn add_token(&mut self, place_ref: &PlaceRef) -> Result<(), &str> {
        let place = self.get_place(place_ref)?;
        place.add_token()
    }

    /// Remove one token from a place in the net.
    ///
    /// # Errors
    ///
    /// If the `PlaceRef` is invalid, then an error is returned.
    pub fn remove_token(&mut self, place_ref: &PlaceRef) -> Result<(), &str> {
        let place = self.get_place(place_ref)?;
        place.remove_token()
    }

    fn get_place(&mut self, place_ref: &PlaceRef) -> Result<&mut Place, &str> {
        match self.places.get_mut(place_ref) {
            Some(place) => Ok(place),
            None => Err("Place reference is invalid. It is not present in the net."),
        }
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
