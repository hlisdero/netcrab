pub use crate::petri_net::place::Place;
pub use crate::petri_net::place_ref::PlaceRef;
pub use crate::petri_net::transition::Transition;
pub use crate::petri_net::transition_ref::TransitionRef;
use std::collections::{BTreeMap, BTreeSet};

mod net_iter;
mod place;
mod place_ref;
mod transition;
mod transition_ref;

#[derive(Default)]
pub struct PetriNet {
    places: BTreeMap<PlaceRef, Place>,
    transitions: BTreeMap<TransitionRef, Transition>,
}

impl PetriNet {
    /// Creates an empty Petri net.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the number of places in the net.
    #[inline]
    #[must_use]
    pub fn get_cardinality_places(&self) -> usize {
        self.places.len()
    }

    /// Gets the number of transitions in the net.
    #[inline]
    #[must_use]
    pub fn get_cardinality_transitions(&self) -> usize {
        self.transitions.len()
    }

    /// Checks if the place reference is valid for this net,
    /// i.e. if the referenced place still exists in the net.
    #[inline]
    #[must_use]
    pub fn check_place_ref(&self, place_ref: &PlaceRef) -> bool {
        self.places.contains_key(place_ref)
    }

    /// Checks if the transition reference is valid for this net,
    /// i.e. if the referenced transition still exists in the net.
    #[inline]
    #[must_use]
    pub fn check_transition_ref(&self, transition_ref: &TransitionRef) -> bool {
        self.transitions.contains_key(transition_ref)
    }

    /// Finds unconnected places in the net.
    /// Returns a `BTreeSet` with the place references as keys.
    #[must_use]
    pub fn find_unconnected_places(&self) -> BTreeSet<PlaceRef> {
        let mut unconnected_set: BTreeSet<PlaceRef> = BTreeSet::new();
        for (place_ref, place) in &self.places {
            if place.get_preset().is_empty() && place.get_postset().is_empty() {
                unconnected_set.insert(place_ref.clone());
            }
        }
        unconnected_set
    }

    /// Finds all arcs from places to transitions in the net.
    /// Returns a `BTreeSet` with tuples of references (source, dest).
    #[must_use]
    pub fn find_arcs_place_transition(&self) -> BTreeSet<(PlaceRef, TransitionRef)> {
        let mut arcs: BTreeSet<(PlaceRef, TransitionRef)> = BTreeSet::new();
        for (place_ref, place) in &self.places {
            for transition_ref in place.get_postset() {
                arcs.insert((place_ref.clone(), transition_ref.clone()));
            }
        }
        arcs
    }

    /// Finds all arcs from transitions to places in the net.
    /// Returns a `BTreeSet` with tuples of references (source, dest).
    #[must_use]
    pub fn find_arcs_transition_place(&self) -> BTreeSet<(TransitionRef, PlaceRef)> {
        let mut arcs: BTreeSet<(TransitionRef, PlaceRef)> = BTreeSet::new();
        for (transition_ref, transition) in &self.transitions {
            for place_ref in transition.get_postset() {
                arcs.insert((transition_ref.clone(), place_ref.clone()));
            }
        }
        arcs
    }

    /// Adds a place to the net.
    /// The place label need not be unique for the net.
    pub fn add_place(&mut self, place_label: &str) -> PlaceRef {
        let place_ref = PlaceRef::new(place_label);
        self.places.insert(place_ref.clone(), Place::new());
        place_ref
    }

    /// Adds a transition to the net.
    /// The transition label need not be unique for the net.
    pub fn add_transition(&mut self, transition_label: &str) -> TransitionRef {
        let transition_ref = TransitionRef::new(transition_label);
        self.transitions
            .insert(transition_ref.clone(), Transition::new());
        transition_ref
    }

    /// Adds an arc from a place to a transition with multiplicity one.
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
        let (place, transition) = self.get_place_transition_pair_mut(place_ref, transition_ref)?;
        // We prefer to clone the references here, since the add operations technically do not need it,
        // but we just want to borrow the references from the user for this operation.
        let inserted_outgoing = place.add_outgoing(transition_ref.clone());
        let inserted_incoming = transition.add_incoming(place_ref.clone());
        Self::check_arc_insertion(inserted_incoming, inserted_outgoing)?;
        Ok(())
    }

    /// Adds an arc from a transition to a place with multiplicity one.
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
        let (place, transition) = self.get_place_transition_pair_mut(place_ref, transition_ref)?;
        // We prefer to clone the references here, since the add operations technically do not need it,
        // but we just want to borrow the references from the user for this operation.
        let inserted_outgoing = transition.add_outgoing(place_ref.clone());
        let inserted_incoming = place.add_incoming(transition_ref.clone());
        Self::check_arc_insertion(inserted_incoming, inserted_outgoing)?;
        Ok(())
    }

    /// Gets the number of tokens in a place in the net.
    ///
    /// # Errors
    ///
    /// If the `PlaceRef` is invalid, then an error is returned.
    pub fn marking(&self, place_ref: &PlaceRef) -> Result<usize, &str> {
        let place = self.get_place(place_ref)?;
        Ok(place.marking())
    }

    /// Gets the marking vector for the net, i.e, the number of tokens for every place in the net.
    /// Returns a `BTreeMap` with the place references as the keys and the number of tokens as values.
    pub fn marking_vector(&mut self) -> BTreeMap<PlaceRef, usize> {
        let mut marking_vector: BTreeMap<PlaceRef, usize> = BTreeMap::new();
        for (key, value) in &self.places {
            marking_vector.insert(key.clone(), value.marking());
        }
        marking_vector
    }

    /// Adds `tokens_to_add` tokens to a place in the net.
    ///
    /// # Errors
    ///
    /// If the `PlaceRef` is invalid, then an error is returned.
    /// If the addition causes an overflow, then an error is returned.
    pub fn add_token(&mut self, place_ref: &PlaceRef, tokens_to_add: usize) -> Result<(), &str> {
        let place = self.get_place_mut(place_ref)?;
        place.add_token(tokens_to_add)
    }

    /// Removes `tokens_to_remove` tokens from a place in the net.
    ///
    /// # Errors
    ///
    /// If the `PlaceRef` is invalid, then an error is returned.
    /// If the subtraction causes an overflow, then an error is returned.
    pub fn remove_token(
        &mut self,
        place_ref: &PlaceRef,
        tokens_to_remove: usize,
    ) -> Result<(), &str> {
        let place = self.get_place_mut(place_ref)?;
        place.remove_token(tokens_to_remove)
    }

    fn get_place(&self, place_ref: &PlaceRef) -> Result<&Place, &str> {
        let Some(place) = self.places.get(place_ref) else {
            return Err("Place reference is invalid. It is not present in the net.");
        };
        Ok(place)
    }

    fn get_place_mut(&mut self, place_ref: &PlaceRef) -> Result<&mut Place, &str> {
        let Some(place) = self.places.get_mut(place_ref) else {
            return Err("Place reference is invalid. It is not present in the net.");
        };
        Ok(place)
    }

    fn get_place_transition_pair_mut(
        &mut self,
        place_ref: &PlaceRef,
        transition_ref: &TransitionRef,
    ) -> Result<(&mut Place, &mut Transition), &str> {
        let Some(place) = self.places.get_mut(place_ref) else {
            return Err("Place reference is invalid. It is not present in the net.");
        };

        let Some(transition) = self.transitions.get_mut(transition_ref) else {
            return Err("Transition reference is invalid. It is not present in the net.");
        };

        Ok((place, transition))
    }

    const fn check_arc_insertion(
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

#[cfg(test)]
mod net_tests {
    use super::*;

    #[test]
    fn new_default_has_no_places() {
        let net = PetriNet::new();
        assert_eq!(net.get_cardinality_places(), 0);
    }

    #[test]
    fn new_default_has_no_transitions() {
        let new = PetriNet::new();
        assert_eq!(new.get_cardinality_transitions(), 0)
    }

    #[test]
    fn net_find_unconnected_places_returns_empty_set_for_empty_net() {
        let net = PetriNet::new();
        let unconnected_set = net.find_unconnected_places();

        assert!(unconnected_set.is_empty());
    }

    #[test]
    fn net_find_unconnected_places_some_empty_places() {
        let mut net = PetriNet::new();
        let place_1 = net.add_place("P1");
        let place_2 = net.add_place("P2");
        let place_3 = net.add_place("P3");

        let transition_1 = net.add_transition("T1");
        let transition_2 = net.add_transition("T2");

        let result = net.add_arc_place_transition(&place_1, &transition_1);
        assert!(result.is_ok());
        let result = net.add_arc_transition_place(&transition_1, &place_2);
        assert!(result.is_ok());
        let result = net.add_arc_place_transition(&place_2, &transition_2);
        assert!(result.is_ok());

        let unconnected_set = net.find_unconnected_places();
        assert_eq!(unconnected_set.len(), 1);
        assert!(unconnected_set.contains(&place_3));
    }

    #[test]
    fn net_find_unconnected_places_all_empty_places() {
        let mut net = PetriNet::new();
        let place_1 = net.add_place("P1");
        let place_2 = net.add_place("P2");
        let place_3 = net.add_place("P3");

        let unconnected_set = net.find_unconnected_places();
        assert_eq!(unconnected_set.len(), 3);
        assert!(unconnected_set.contains(&place_1));
        assert!(unconnected_set.contains(&place_2));
        assert!(unconnected_set.contains(&place_3));
    }

    #[test]
    fn net_find_arcs_place_transition_lists_all_arcs() {
        let mut net = PetriNet::new();
        let place_1 = net.add_place("P1");
        let place_2 = net.add_place("P2");
        let place_3 = net.add_place("P3");

        let transition_1 = net.add_transition("T1");
        let transition_2 = net.add_transition("T2");

        let result = net.add_arc_place_transition(&place_1, &transition_1);
        assert!(result.is_ok());
        let result = net.add_arc_transition_place(&transition_1, &place_2);
        assert!(result.is_ok());
        let result = net.add_arc_place_transition(&place_2, &transition_2);
        assert!(result.is_ok());
        let result = net.add_arc_transition_place(&transition_2, &place_3);
        assert!(result.is_ok());

        let arcs = net.find_arcs_place_transition();
        assert_eq!(arcs.len(), 2);
        assert!(arcs.contains(&(place_1, transition_1)));
        assert!(arcs.contains(&(place_2, transition_2)));
    }

    #[test]
    fn net_find_arcs_transition_place_lists_all_arcs() {
        let mut net = PetriNet::new();
        let place_1 = net.add_place("P1");
        let place_2 = net.add_place("P2");
        let place_3 = net.add_place("P3");

        let transition_1 = net.add_transition("T1");
        let transition_2 = net.add_transition("T2");

        let result = net.add_arc_place_transition(&place_1, &transition_1);
        assert!(result.is_ok());
        let result = net.add_arc_transition_place(&transition_1, &place_2);
        assert!(result.is_ok());
        let result = net.add_arc_place_transition(&place_2, &transition_2);
        assert!(result.is_ok());
        let result = net.add_arc_transition_place(&transition_2, &place_3);
        assert!(result.is_ok());

        let arcs = net.find_arcs_transition_place();
        assert_eq!(arcs.len(), 2);
        assert!(arcs.contains(&(transition_1, place_2)));
        assert!(arcs.contains(&(transition_2, place_3)));
    }

    #[test]
    fn net_add_place_updates_size() {
        let mut net = PetriNet::new();
        net.add_place("Example place");

        assert_eq!(net.get_cardinality_places(), 1);
    }

    #[test]
    fn net_add_transition_updates_size() {
        let mut net = PetriNet::new();
        net.add_transition("Example transition");

        assert_eq!(net.get_cardinality_transitions(), 1);
    }

    #[test]
    fn net_marking_returns_number_of_tokens_in_place() {
        let mut net = PetriNet::new();
        let place_ref = net.add_place("Example place");

        let result = net.marking(&place_ref);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn net_marking_add_token_increases_number_of_tokens_in_place() {
        let mut net = PetriNet::new();
        let place_ref = net.add_place("Example place");

        assert!(net.add_token(&place_ref, 1).is_ok());

        let result = net.marking(&place_ref);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn net_marking_add_token_multiple_times() {
        let mut net = PetriNet::new();
        let place_ref = net.add_place("Example place");

        assert!(net.add_token(&place_ref, 5).is_ok());

        let result = net.marking(&place_ref);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn net_marking_remove_token_decreases_number_of_tokens_in_place() {
        let mut net = PetriNet::new();
        let place_ref = net.add_place("Example place");

        assert!(net.add_token(&place_ref, 2).is_ok());
        assert!(net.remove_token(&place_ref, 1).is_ok());

        let result = net.marking(&place_ref);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn net_marking_remove_token_returns_err_if_place_empty() {
        let mut net = PetriNet::new();
        let place_ref = net.add_place("Example place");

        assert!(net.remove_token(&place_ref, 1).is_err());
    }

    #[test]
    fn net_marking_vector_three_empty_places() {
        let mut net = PetriNet::new();
        let place_1 = net.add_place("P1");
        let place_2 = net.add_place("P2");
        let place_3 = net.add_place("P3");

        let result = net.marking_vector();
        assert_eq!(*result.get(&place_1).unwrap(), 0);
        assert_eq!(*result.get(&place_2).unwrap(), 0);
        assert_eq!(*result.get(&place_3).unwrap(), 0);
    }

    #[test]
    fn net_marking_vector_two_places() {
        let mut net = PetriNet::new();
        let place_1 = net.add_place("P1");
        let place_2 = net.add_place("P2");

        assert!(net.add_token(&place_1, 5).is_ok());
        assert!(net.add_token(&place_2, 3).is_ok());

        let result = net.marking_vector();
        assert_eq!(*result.get(&place_1).unwrap(), 5);
        assert_eq!(*result.get(&place_2).unwrap(), 3);
    }

    #[test]
    fn net_add_arc_place_transition_simple() {
        let mut net = PetriNet::new();
        let place_ref = net.add_place("Example place");
        let transition_ref = net.add_transition("Example transition");

        let result = net.add_arc_place_transition(&place_ref, &transition_ref);
        assert!(result.is_ok());
    }

    #[test]
    fn net_add_arc_transition_place_simple() {
        let mut net = PetriNet::new();
        let place_ref = net.add_place("Example place");
        let transition_ref = net.add_transition("Example transition");

        let result = net.add_arc_transition_place(&transition_ref, &place_ref);
        assert!(result.is_ok());
    }

    #[test]
    fn net_add_place_transition_form_self_loop() {
        let mut net = PetriNet::new();
        let place_ref = net.add_place("Example place");
        let transition_ref = net.add_transition("Example transition");

        let result = net.add_arc_place_transition(&place_ref, &transition_ref);
        assert!(result.is_ok());
        let result = net.add_arc_transition_place(&transition_ref, &place_ref);
        assert!(result.is_ok());
    }

    #[test]
    fn net_add_place_transition_form_chain() {
        let mut net = PetriNet::new();
        let place_1 = net.add_place("P1");
        let place_2 = net.add_place("P2");
        let place_3 = net.add_place("P3");

        let transition_1 = net.add_transition("T1");
        let transition_2 = net.add_transition("T2");

        let result = net.add_arc_place_transition(&place_1, &transition_1);
        assert!(result.is_ok());
        let result = net.add_arc_transition_place(&transition_1, &place_2);
        assert!(result.is_ok());
        let result = net.add_arc_place_transition(&place_2, &transition_2);
        assert!(result.is_ok());
        let result = net.add_arc_transition_place(&transition_2, &place_3);
        assert!(result.is_ok());
    }
}
