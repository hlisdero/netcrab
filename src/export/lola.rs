use crate::petri_net::{PetriNet, PlaceRef};
use std::collections::BTreeSet;

impl PetriNet {
    /// Converts the net to a string in the format accepted by the `LoLA` model checker and returns it.
    ///
    /// # Errors
    ///
    /// If the writer fails to write the contents of the net, then an error is returned.
    pub fn to_lola_string(&self) -> Result<String, std::io::Error> {
        let mut writer = Vec::new();
        self.to_lola(&mut writer)?;
        String::from_utf8(writer).map_err(|_|
            // This error could only be due to a bug, map it to a more standard error type.
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not convert the string to UTF-8",
            ))
    }

    /// Converts the net to the format accepted by the `LoLA` model checker.
    /// Writes the output to a trait object which implements `std::io::Write`.
    ///
    /// # Errors
    ///
    /// If the writer fails to write the contents of the net, then an error is returned.
    pub fn to_lola<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        self.write_lola_places(writer)?;
        self.write_lola_initial_marking(writer)?;
        self.write_lola_transitions(writer)?;
        Ok(())
    }

    /// Writes the lines that define the places
    /// to a trait object which implements `std::io::Write`.
    fn write_lola_places<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        if self.get_cardinality_places() == 0 {
            return Ok(());
        }
        writer.write_all(b"PLACE\n")?;

        let last_index = self.get_cardinality_places() - 1;
        for (i, (place_ref, _)) in self.places_iter().enumerate() {
            let line = if i == last_index {
                // Last place line has a semicolon and an empty line.
                format!("    {place_ref};\n\n")
            } else {
                format!("    {place_ref},\n")
            };
            writer.write_all(line.as_bytes())?;
        }
        Ok(())
    }

    /// Writes the lines that define the initial marking
    /// to a trait object which implements `std::io::Write`.
    fn write_lola_initial_marking<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        if self.get_cardinality_places() == 0 {
            return Ok(());
        }
        writer.write_all(b"MARKING\n")?;

        let last_index = self.get_cardinality_places() - 1;
        for (i, (place_ref, place)) in self.places_iter().enumerate() {
            let marking = place.marking();

            let line = if i == last_index {
                // Last marking line has a semicolon and an empty line.
                format!("    {place_ref} : {marking};\n\n")
            } else {
                format!("    {place_ref} : {marking},\n")
            };
            writer.write_all(line.as_bytes())?;
        }
        Ok(())
    }

    /// Writes the lines that define the transitions
    /// to a trait object which implements `std::io::Write`.
    fn write_lola_transitions<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        for (transition_ref, transition) in self.transitions_iter() {
            let header_line = format!("TRANSITION {transition_ref}\n");
            writer.write_all(header_line.as_bytes())?;

            Self::write_transition_arcs(transition.get_preset(), "CONSUME", writer)?;
            Self::write_transition_arcs(transition.get_postset(), "PRODUCE", writer)?;
        }
        Ok(())
    }

    // Writes the lines corresponding to either the preset or the postset of a given transition
    // that define tokens from which places are consumed and produced when the transition is fired.
    // The result is written to a trait object which implements `std::io::Write`.
    fn write_transition_arcs<T>(
        set: &BTreeSet<PlaceRef>,
        header: &str,
        writer: &mut T,
    ) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        // The format requires us to close the header with a semicolon if empty
        if set.is_empty() {
            let header_line = format!("  {header};\n");
            writer.write_all(header_line.as_bytes())?;
            return Ok(());
        }
        let header_line = format!("  {header}\n");
        writer.write_all(header_line.as_bytes())?;

        let last_index = set.len() - 1;
        for (i, place_ref) in set.iter().enumerate() {
            // Edge multiplicity is always 1 for now.
            let line = if i == last_index {
                // Last line has a semicolon and an empty line.
                format!("    {place_ref} : 1;\n")
            } else {
                format!("    {place_ref} : 1,\n")
            };
            writer.write_all(line.as_bytes())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod lola_tests {
    use super::*;
    use crate::export::test_export_examples::*;
    use crate::net_creator::*;

    #[test]
    fn lola_string_empty_net() {
        let net = PetriNet::new();
        let result = net.to_lola_string();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), String::new());
    }

    #[test]
    fn lola_string_only_empty_places_net() {
        let (net, _, _) = create_basic_unconnected_net(5, 0);
        let result = net.to_lola_string();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LOLA_STRING_ONLY_EMPTY_PLACES_NET);
    }

    #[test]
    fn lola_string_marked_places_net() {
        let mut net = PetriNet::new();
        let p1 = net.add_place("P1");
        let p2 = net.add_place("P2");
        let p3 = net.add_place("P3");
        let p4 = net.add_place("P4");
        let p5 = net.add_place("P5");

        assert!(net.add_token(&p1, 5).is_ok());
        assert!(net.add_token(&p2, 6).is_ok());
        assert!(net.add_token(&p3, 3).is_ok());
        assert!(net.add_token(&p4, 2).is_ok());
        assert!(net.add_token(&p5, 1).is_ok());
        let result = net.to_lola_string();

        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), LOLA_STRING_MARKED_PLACES_NET);
    }

    #[test]
    fn lola_string_only_empty_transitions_net() {
        let (net, _, _) = create_basic_unconnected_net(0, 5);
        let result = net.to_lola_string();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LOLA_STRING_ONLY_EMPTY_TRANSITIONS_NET,);
    }

    #[test]
    fn lola_string_net_with_chain_topology() {
        let (net, _, _) = create_net_chain_topology(3);
        let result = net.to_lola_string();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LOLA_STRING_NET_WITH_CHAIN_TOPOLOPY);
    }

    #[test]
    fn lola_string_net_with_loop_topology() {
        let (net, _, _) = create_net_loop_topology();
        let result = net.to_lola_string();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), LOLA_STRING_NET_WITH_LOOP_TOPOLOGY);
    }
}
