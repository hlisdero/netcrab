use crate::net::{ConnectableNode, PetriNet, PlaceRef};
use std::collections::HashSet;

impl PetriNet {
    /// Convert the net to a string in the format accepted by the LoLA model checker and return it.
    ///
    /// # Errors
    ///
    /// If the writer fails to write the contents of the net, then an error is returned.
    pub fn to_lola_string(&self) -> Result<String, std::io::Error> {
        let mut writer = Vec::new();
        self.to_lola(&mut writer)?;
        match String::from_utf8(writer) {
            Ok(string) => Ok(string),
            // This error could only be due to a bug, map it to a different error type.
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not convert the string to UTF-8",
            )),
        }
    }

    /// Convert the net to the format accepted by the LoLA model checker.
    /// Write the output to a trait object which implements `std::io::Write`.
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

    /// Write the lines that define the places
    /// to a trait object which implements `std::io::Write`.
    fn write_lola_places<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        if self.get_cardinality_places() == 0 {
            return Ok(());
        }
        writer.write("PLACE\n".as_bytes())?;

        let last_index = self.get_cardinality_places() - 1;
        for (i, (place_ref, _)) in self.places_iter().enumerate() {
            let line = if i == last_index {
                // Last place line has a semicolon and an empty line.
                format!("    {};\n\n", place_ref.as_string())
            } else {
                format!("    {},\n", place_ref.as_string())
            };
            writer.write(line.as_bytes())?;
        }
        Ok(())
    }

    /// Write the lines that define the initial marking
    /// to a trait object which implements `std::io::Write`.
    fn write_lola_initial_marking<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        if self.get_cardinality_places() == 0 {
            return Ok(());
        }
        writer.write("MARKING\n".as_bytes())?;

        let last_index = self.get_cardinality_places() - 1;
        for (i, (place_ref, place)) in self.places_iter().enumerate() {
            let marking = place.marking();
            if marking == 0 {
                continue;
            }

            let line = if i == last_index {
                // Last marking line has a semicolon and an empty line.
                format!("    {} : {};\n\n", place_ref.as_string(), marking)
            } else {
                format!("    {} : {},\n", place_ref.as_string(), marking)
            };
            writer.write(line.as_bytes())?;
        }
        Ok(())
    }

    /// Write the lines that define the transitions
    /// to a trait object which implements `std::io::Write`.
    fn write_lola_transitions<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        for (transition_ref, transition) in self.transitions_iter() {
            let header_line = format!("TRANSITION {}\n", transition_ref.as_string());
            writer.write(header_line.as_bytes())?;

            Self::write_transition_arcs(transition.get_preset(), "CONSUME", writer)?;
            Self::write_transition_arcs(transition.get_postset(), "PRODUCE", writer)?;
        }
        Ok(())
    }

    // Write the lines corresponding to either the preset or the postset of a given transition
    // that define tokens from which places are consumed and produced when the transition is fired.
    // The result is written to a trait object which implements `std::io::Write`.
    fn write_transition_arcs<T>(
        set: &HashSet<PlaceRef>,
        header: &str,
        writer: &mut T,
    ) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        if set.is_empty() {
            return Ok(());
        }
        let header_line = format!("  {}\n", header);
        writer.write(header_line.as_bytes())?;

        let last_index = set.len() - 1;
        for (i, place_ref) in set.iter().enumerate() {
            // Multiplicity is always 1 for now.
            let line = if i == last_index {
                // Last line has a semicolon and an empty line.
                format!("    {} : {};\n", place_ref.as_string(), 1)
            } else {
                format!("    {} : {},\n", place_ref.as_string(), 1)
            };
            writer.write(line.as_bytes())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod lola_tests {
    use super::*;

    #[test]
    fn lola_string_empty_net() {
        let net = PetriNet::new();
        let result = net.to_lola_string();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), String::new());
    }
}
