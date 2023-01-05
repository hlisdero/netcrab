use crate::petri_net::PetriNet;

const MAX_TOKENS_AS_DOT: usize = 5;

impl PetriNet {
    /// Converts the net to a string in DOT format and returns it.
    ///
    /// # Errors
    ///
    /// If the writer fails to write the contents of the net, then an error is returned.
    pub fn to_dot_string(&self) -> Result<String, std::io::Error> {
        let mut writer = Vec::new();
        self.to_dot(&mut writer)?;
        String::from_utf8(writer).map_err(|_|
            // This error could only be due to a bug, map it to a more standard error type.
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not convert the string to UTF-8",
            ))
    }

    /// Converts the net to the dot format.
    /// Writes the output to a trait object which implements `std::io::Write`.
    ///
    /// # Errors
    ///
    /// If the writer fails to write the contents of the net, then an error is returned.
    pub fn to_dot<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        writer.write_all(b"digraph petrinet {\n")?;
        self.write_dot_places(writer)?;
        self.write_dot_transitions(writer)?;
        self.write_dot_arcs(writer)?;
        writer.write_all(b"}\n")?;
        Ok(())
    }

    /// Writes the lines that define the places
    /// to a trait object which implements `std::io::Write`.
    fn write_dot_places<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        for (place_ref, place) in self.places_iter() {
            let label = Self::sanitize_string(place_ref.label());
            let marking = Self::marking_to_string(place.marking());
            let line =
                format!("    {label} [shape=\"circle\" xlabel=\"{label}\" label=\"{marking}\"];\n");
            writer.write_all(line.as_bytes())?;
        }
        Ok(())
    }

    /// Writes the lines that define the transitions
    /// to a trait object which implements `std::io::Write`.
    fn write_dot_transitions<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        for (transition_ref, _) in self.transitions_iter() {
            let label = Self::sanitize_string(transition_ref.label());
            let line = format!("    {label} [shape=\"box\" xlabel=\"{label}\" label=\"\"];\n");
            writer.write_all(line.as_bytes())?;
        }
        Ok(())
    }

    /// Writes the lines that define the arcs
    /// to a trait object which implements `std::io::Write`.
    fn write_dot_arcs<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        let arcs = self.find_arcs_place_transition();
        for (place_ref, transition_ref) in arcs {
            let line = format!(
                "    {} -> {};\n",
                Self::sanitize_string(place_ref.label()),
                Self::sanitize_string(transition_ref.label())
            );
            writer.write_all(line.as_bytes())?;
        }

        let arcs = self.find_arcs_transition_place();
        for (transition_ref, place_ref) in arcs {
            let line = format!(
                "    {} -> {};\n",
                Self::sanitize_string(transition_ref.label()),
                Self::sanitize_string(place_ref.label()),
            );
            writer.write_all(line.as_bytes())?;
        }

        Ok(())
    }

    /// Converts the label if present to a valid `String`.
    /// Removes newlines and quotes from the label.
    ///
    /// Using escape sequences it is possible to achieve special behavior.
    /// [More info](https://graphviz.org/docs/attr-types/escString/)
    fn sanitize_string(string: &str) -> String {
        string.replace('\n', "").replace('\"', "\\\"")
    }

    /// Convert the marking to a valid string.
    fn marking_to_string(marking: usize) -> String {
        match marking {
            0 => String::new(),
            1..=MAX_TOKENS_AS_DOT => "•".repeat(marking),
            _ => marking.to_string(),
        }
    }
}

#[cfg(test)]
mod dot_tests {
    use super::*;
    use crate::export::test_export_examples::*;
    use crate::net_creator::*;

    #[test]
    fn dot_string_empty_net() {
        let net = PetriNet::new();
        let result = net.to_dot_string();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), DOT_STRING_EMPTY_NET);
    }

    #[test]
    fn dot_string_only_empty_places_net() {
        let (net, _, _) = create_basic_unconnected_net(5, 0);
        let result = net.to_dot_string();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), DOT_STRING_ONLY_EMPTY_PLACES_NET);
    }

    #[test]
    fn dot_string_marked_places_net() {
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
        let result = net.to_dot_string();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), DOT_STRING_MARKED_PLACES_NET);
    }

    #[test]
    fn dot_string_only_empty_transitions_net() {
        let (net, _, _) = create_basic_unconnected_net(0, 5);
        let result = net.to_dot_string();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), DOT_STRING_ONLY_EMPTY_TRANSITIONS_NET);
    }

    #[test]
    fn dot_string_net_with_chain_topology() {
        let (net, _, _) = create_net_chain_topology(3);
        let result = net.to_dot_string();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), DOT_STRING_NET_WITH_CHAIN_TOPOLOPY);
    }

    #[test]
    fn dot_string_net_with_loop_topology() {
        let (net, _, _) = create_net_loop_topology();
        let result = net.to_dot_string();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), DOT_STRING_NET_WITH_LOOP_TOPOLOGY);
    }
}
