use crate::net::node::{ConnectableNode, Place, Transition};
use crate::net::node_ref::{PlaceRef, TransitionRef};
use std::collections::{HashMap, HashSet};

mod net_iter;
mod node;
mod node_ref;

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

    /// Find all edges from places to transitions in the net.
    /// Return a `HashSet` with tuples of references (source, dest).
    #[must_use]
    pub fn find_edges_place_transition(&self) -> HashSet<(PlaceRef, TransitionRef)> {
        let mut edges: HashSet<(PlaceRef, TransitionRef)> = HashSet::new();
        for (place_ref, place) in &self.places {
            for transition_ref in place.get_postset() {
                edges.insert((place_ref.clone(), transition_ref.clone()));
            }
        }
        edges
    }

    /// Find all edges from transitions to places in the net.
    /// Return a `HashSet` with tuples of references (source, dest).
    #[must_use]
    pub fn find_edges_transition_place(&self) -> HashSet<(TransitionRef, PlaceRef)> {
        let mut edges: HashSet<(TransitionRef, PlaceRef)> = HashSet::new();
        for (transition_ref, transition) in &self.transitions {
            for place_ref in transition.get_postset() {
                edges.insert((transition_ref.clone(), place_ref.clone()));
            }
        }
        edges
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

#[cfg(test)]
mod net_tests {
    use super::*;

    #[test]
    fn new_default_has_no_places() {
        let net = PetriNet::default();
        assert_eq!(net.get_cardinality_places(), 0);
    }

    #[test]
    fn new_default_has_no_transitions() {
        let new = PetriNet::default();
        assert_eq!(new.get_cardinality_transitions(), 0)
    }

    #[test]
    fn net_find_unconnected_places_returns_empty_set_for_empty_net() {
        let net = PetriNet::default();
        let unconnected_set = net.find_unconnected_places();

        assert!(unconnected_set.is_empty());
    }

    #[test]
    fn net_find_unconnected_places_some_empty_places() {
        let mut net = PetriNet::default();
        let place_1 = net.add_place(&"P1".to_string());
        let place_2 = net.add_place(&"P2".to_string());
        let place_3 = net.add_place(&"P3".to_string());

        let transition_1 = net.add_transition(&"T1".to_string());
        let transition_2 = net.add_transition(&"T2".to_string());

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
        let mut net = PetriNet::default();
        let place_1 = net.add_place(&"P1".to_string());
        let place_2 = net.add_place(&"P2".to_string());
        let place_3 = net.add_place(&"P3".to_string());

        let unconnected_set = net.find_unconnected_places();
        assert_eq!(unconnected_set.len(), 3);
        assert!(unconnected_set.contains(&place_1));
        assert!(unconnected_set.contains(&place_2));
        assert!(unconnected_set.contains(&place_3));
    }

    #[test]
    fn net_find_edges_place_transition_list_all_edges() {
        let mut net = PetriNet::default();
        let place_1 = net.add_place(&"P1".to_string());
        let place_2 = net.add_place(&"P2".to_string());
        let place_3 = net.add_place(&"P3".to_string());

        let transition_1 = net.add_transition(&"T1".to_string());
        let transition_2 = net.add_transition(&"T2".to_string());

        let result = net.add_arc_place_transition(&place_1, &transition_1);
        assert!(result.is_ok());
        let result = net.add_arc_transition_place(&transition_1, &place_2);
        assert!(result.is_ok());
        let result = net.add_arc_place_transition(&place_2, &transition_2);
        assert!(result.is_ok());
        let result = net.add_arc_transition_place(&transition_2, &place_3);
        assert!(result.is_ok());

        let edges = net.find_edges_place_transition();
        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&(place_1, transition_1)));
        assert!(edges.contains(&(place_2, transition_2)));
    }

    #[test]
    fn net_find_edges_transition_place_list_all_edges() {
        let mut net = PetriNet::default();
        let place_1 = net.add_place(&"P1".to_string());
        let place_2 = net.add_place(&"P2".to_string());
        let place_3 = net.add_place(&"P3".to_string());

        let transition_1 = net.add_transition(&"T1".to_string());
        let transition_2 = net.add_transition(&"T2".to_string());

        let result = net.add_arc_place_transition(&place_1, &transition_1);
        assert!(result.is_ok());
        let result = net.add_arc_transition_place(&transition_1, &place_2);
        assert!(result.is_ok());
        let result = net.add_arc_place_transition(&place_2, &transition_2);
        assert!(result.is_ok());
        let result = net.add_arc_transition_place(&transition_2, &place_3);
        assert!(result.is_ok());

        let edges = net.find_edges_transition_place();
        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&(transition_1, place_2)));
        assert!(edges.contains(&(transition_2, place_3)));
    }

    #[test]
    fn net_add_place_updates_size() {
        let mut net = PetriNet::default();
        net.add_place(&"Example place".to_string());

        assert_eq!(net.get_cardinality_places(), 1);
    }

    #[test]
    fn net_add_transition_updates_size() {
        let mut net = PetriNet::default();
        net.add_transition(&"Example transition".to_string());

        assert_eq!(net.get_cardinality_transitions(), 1);
    }

    #[test]
    fn net_marking_returns_number_of_tokens_in_place() {
        let mut net = PetriNet::default();
        let place_ref = net.add_place(&"Example place".to_string());

        let result = net.marking(&place_ref);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn net_marking_add_token_increases_number_of_tokens_in_place() {
        let mut net = PetriNet::default();
        let place_ref = net.add_place(&"Example place".to_string());

        assert!(net.add_token(&place_ref).is_ok());

        let result = net.marking(&place_ref);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn net_marking_add_token_multiple_times() {
        let mut net = PetriNet::default();
        let place_ref = net.add_place(&"Example place".to_string());

        assert!(net.add_token(&place_ref).is_ok());
        assert!(net.add_token(&place_ref).is_ok());
        assert!(net.add_token(&place_ref).is_ok());
        assert!(net.add_token(&place_ref).is_ok());
        assert!(net.add_token(&place_ref).is_ok());

        let result = net.marking(&place_ref);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn net_marking_remove_token_decreases_number_of_tokens_in_place() {
        let mut net = PetriNet::default();
        let place_ref = net.add_place(&"Example place".to_string());

        assert!(net.add_token(&place_ref).is_ok());
        assert!(net.add_token(&place_ref).is_ok());
        assert!(net.remove_token(&place_ref).is_ok());

        let result = net.marking(&place_ref);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn net_marking_remove_token_returns_err_if_place_empty() {
        let mut net = PetriNet::default();
        let place_ref = net.add_place(&"Example place".to_string());

        assert!(net.remove_token(&place_ref).is_err());
    }

    #[test]
    fn net_marking_vector_three_empty_places() {
        let mut net = PetriNet::default();
        let place_1 = net.add_place(&"P1".to_string());
        let place_2 = net.add_place(&"P2".to_string());
        let place_3 = net.add_place(&"P3".to_string());

        let result = net.marking_vector();
        assert_eq!(*result.get(&place_1).unwrap(), 0);
        assert_eq!(*result.get(&place_2).unwrap(), 0);
        assert_eq!(*result.get(&place_3).unwrap(), 0);
    }

    #[test]
    fn net_marking_vector_two_places() {
        let mut net = PetriNet::default();
        let place_1 = net.add_place(&"P1".to_string());
        let place_2 = net.add_place(&"P2".to_string());

        assert!(net.add_token(&place_1).is_ok());
        assert!(net.add_token(&place_1).is_ok());
        assert!(net.add_token(&place_1).is_ok());
        assert!(net.add_token(&place_1).is_ok());
        assert!(net.add_token(&place_1).is_ok());

        assert!(net.add_token(&place_2).is_ok());
        assert!(net.add_token(&place_2).is_ok());
        assert!(net.add_token(&place_2).is_ok());

        let result = net.marking_vector();
        assert_eq!(*result.get(&place_1).unwrap(), 5);
        assert_eq!(*result.get(&place_2).unwrap(), 3);
    }

    #[test]
    fn net_add_arc_place_transition_simple() {
        let mut net = PetriNet::default();
        let place_ref = net.add_place(&"Example place".to_string());
        let transition_ref = net.add_transition(&"Example transition".to_string());

        let result = net.add_arc_place_transition(&place_ref, &transition_ref);
        assert!(result.is_ok());
    }

    #[test]
    fn net_add_arc_transition_place_simple() {
        let mut net = PetriNet::default();
        let place_ref = net.add_place(&"Example place".to_string());
        let transition_ref = net.add_transition(&"Example transition".to_string());

        let result = net.add_arc_transition_place(&transition_ref, &place_ref);
        assert!(result.is_ok());
    }

    #[test]
    fn net_add_place_transition_form_self_loop() {
        let mut net = PetriNet::default();
        let place_ref = net.add_place(&"Example place".to_string());
        let transition_ref = net.add_transition(&"Example transition".to_string());

        let result = net.add_arc_place_transition(&place_ref, &transition_ref);
        assert!(result.is_ok());
        let result = net.add_arc_transition_place(&transition_ref, &place_ref);
        assert!(result.is_ok());
    }

    #[test]
    fn net_add_place_transition_form_chain() {
        let mut net = PetriNet::default();
        let place_1 = net.add_place(&"P1".to_string());
        let place_2 = net.add_place(&"P2".to_string());
        let place_3 = net.add_place(&"P3".to_string());

        let transition_1 = net.add_transition(&"T1".to_string());
        let transition_2 = net.add_transition(&"T2".to_string());

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
