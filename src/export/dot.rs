use crate::net::PetriNet;

const MAX_TOKENS_AS_DOT: usize = 5;

impl PetriNet {
    /// Convert the net to a string in DOT format and return it.
    ///
    /// # Errors
    ///
    /// If the writer fails to write the contents of the net, then an error is returned.
    pub fn to_dot_string(&self) -> Result<String, std::io::Error> {
        let mut writer = Vec::new();
        self.to_dot(&mut writer)?;
        match String::from_utf8(writer) {
            Ok(string) => Ok(string),
            // This error could only be due to a bug, map it to a different error type.
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not convert the string to UTF-8",
            )),
        }
    }

    /// Convert the net to the dot format.
    /// Write the output to a trait object which implements `std::io::Write`.
    ///
    /// # Errors
    ///
    /// If the writer fails to write the contents of the net, then an error is returned.
    pub fn to_dot<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        writer.write_all("digraph petrinet {\n".as_bytes())?;
        self.write_dot_places(writer)?;
        self.write_dot_transitions(writer)?;
        self.write_dot_arcs(writer)?;
        writer.write_all("}\n".as_bytes())?;
        Ok(())
    }

    /// Write the lines that define the places
    /// to a trait object which implements `std::io::Write`.
    fn write_dot_places<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        for (place_ref, place) in self.places_iter() {
            let label = Self::sanitize_string(place_ref.as_string());
            let marking = Self::marking_to_string(place.marking());
            let line = format!(
                "    {} [shape=\"circle\" xlabel=\"{}\" label=\"{}\"];\n",
                label, label, marking
            );
            writer.write_all(line.as_bytes())?;
        }
        Ok(())
    }

    /// Write the lines that define the transitions
    /// to a trait object which implements `std::io::Write`.
    fn write_dot_transitions<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        for (transition_ref, _) in self.transitions_iter() {
            let label = Self::sanitize_string(transition_ref.as_string());
            let line = format!(
                "    {} [shape=\"box\" xlabel=\"{}\" label=\"\"];\n",
                label, label
            );
            writer.write_all(line.as_bytes())?;
        }
        Ok(())
    }

    /// Write the lines that define the arcs
    /// to a trait object which implements `std::io::Write`.
    fn write_dot_arcs<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        let arcs = self.find_arcs_place_transition();
        for (place_ref, transition_ref) in arcs {
            let line = format!(
                "    {} -> {};\n",
                Self::sanitize_string(place_ref.as_string()),
                Self::sanitize_string(transition_ref.as_string())
            );
            writer.write_all(line.as_bytes())?;
        }

        let arcs = self.find_arcs_transition_place();
        for (transition_ref, place_ref) in arcs {
            let line = format!(
                "    {} -> {};\n",
                Self::sanitize_string(transition_ref.as_string()),
                Self::sanitize_string(place_ref.as_string()),
            );
            writer.write_all(line.as_bytes())?;
        }

        Ok(())
    }

    /// Convert the label if present to a valid `String`.
    /// Remove newlines and quotes from the label.
    ///
    /// Using escape sequences it is possible to achieve special behavior.
    /// [More info](https://graphviz.org/docs/attr-types/escString/)
    fn sanitize_string(string: &String) -> String {
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
    use crate::export::test_utils::assert_all_lines_arbitrary_order;

    #[test]
    fn dot_string_empty_net() {
        let net = PetriNet::new();
        let result = net.to_dot_string();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "digraph petrinet {\n}\n".to_string());
    }

    #[test]
    fn dot_string_only_empty_places_net() {
        let mut net = PetriNet::new();
        net.add_place(&"P1".to_string());
        net.add_place(&"P2".to_string());
        net.add_place(&"P3".to_string());
        net.add_place(&"P4".to_string());
        net.add_place(&"P5".to_string());
        let result = net.to_dot_string();

        assert!(result.is_ok());
        let expected_result = "digraph petrinet {\n\
            P1 [shape=\"circle\" xlabel=\"P1\" label=\"\"];\n\
            P2 [shape=\"circle\" xlabel=\"P2\" label=\"\"];\n\
            P3 [shape=\"circle\" xlabel=\"P3\" label=\"\"];\n\
            P4 [shape=\"circle\" xlabel=\"P4\" label=\"\"];\n\
            P5 [shape=\"circle\" xlabel=\"P5\" label=\"\"];\n\
        }\n"
        .to_string();

        assert_all_lines_arbitrary_order(result.unwrap(), expected_result);
    }

    #[test]
    fn dot_string_marked_places_net() {
        let mut net = PetriNet::new();
        let p1 = net.add_place(&"P1".to_string());
        let p2 = net.add_place(&"P2".to_string());
        let p3 = net.add_place(&"P3".to_string());
        let p4 = net.add_place(&"P4".to_string());
        let p5 = net.add_place(&"P5".to_string());

        assert!(net.add_token(&p1, 5).is_ok());
        assert!(net.add_token(&p2, 6).is_ok());
        assert!(net.add_token(&p3, 3).is_ok());
        assert!(net.add_token(&p4, 2).is_ok());
        assert!(net.add_token(&p5, 1).is_ok());
        let result = net.to_dot_string();

        assert!(result.is_ok());
        let expected_result = "digraph petrinet {\n\
            P1 [shape=\"circle\" xlabel=\"P1\" label=\"•••••\"];\n\
            P2 [shape=\"circle\" xlabel=\"P2\" label=\"6\"];\n\
            P3 [shape=\"circle\" xlabel=\"P3\" label=\"•••\"];\n\
            P4 [shape=\"circle\" xlabel=\"P4\" label=\"••\"];\n\
            P5 [shape=\"circle\" xlabel=\"P5\" label=\"•\"];\n\
        }\n"
        .to_string();

        assert_all_lines_arbitrary_order(result.unwrap(), expected_result);
    }

    #[test]
    fn dot_string_only_empty_transitions_net() {
        let mut net = PetriNet::new();
        net.add_transition(&"T1".to_string());
        net.add_transition(&"T2".to_string());
        net.add_transition(&"T3".to_string());
        net.add_transition(&"T4".to_string());
        net.add_transition(&"T5".to_string());
        let result = net.to_dot_string();

        assert!(result.is_ok());
        let expected_result = "digraph petrinet {\n\
            T1 [shape=\"box\" xlabel=\"T1\" label=\"\"];\n\
            T2 [shape=\"box\" xlabel=\"T2\" label=\"\"];\n\
            T3 [shape=\"box\" xlabel=\"T3\" label=\"\"];\n\
            T4 [shape=\"box\" xlabel=\"T4\" label=\"\"];\n\
            T5 [shape=\"box\" xlabel=\"T5\" label=\"\"];\n\
        }\n"
        .to_string();

        assert_all_lines_arbitrary_order(result.unwrap(), expected_result);
    }

    #[test]
    fn dot_string_net_with_chain_topology() {
        let net = create_net_chain_topology();
        let result = net.to_dot_string();

        assert!(result.is_ok());
        let expected_result = "digraph petrinet {\n\
            P1 [shape=\"circle\" xlabel=\"P1\" label=\"\"];\n\
            P2 [shape=\"circle\" xlabel=\"P2\" label=\"\"];\n\
            P3 [shape=\"circle\" xlabel=\"P3\" label=\"\"];\n\
            T1 [shape=\"box\" xlabel=\"T1\" label=\"\"];\n\
            T2 [shape=\"box\" xlabel=\"T2\" label=\"\"];\n\
            P1 -> T1;\n\
            T1 -> P2;\n\
            P2 -> T2;\n\
            T2 -> P3;\n\
        }\n"
        .to_string();

        assert_all_lines_arbitrary_order(result.unwrap(), expected_result);
    }

    #[test]
    fn dot_string_net_with_loop_topology() {
        let net = create_net_loop_topology();
        let result = net.to_dot_string();

        assert!(result.is_ok());
        let expected_result = "digraph petrinet {\n\
            P1 [shape=\"circle\" xlabel=\"P1\" label=\"\"];\n\
            T1 [shape=\"box\" xlabel=\"T1\" label=\"\"];\n\
            P1 -> T1;\n\
            T1 -> P1;\n\
        }\n"
        .to_string();

        assert_all_lines_arbitrary_order(result.unwrap(), expected_result);
    }

    fn create_net_chain_topology() -> PetriNet {
        let mut net = PetriNet::new();
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
        net
    }

    fn create_net_loop_topology() -> PetriNet {
        let mut net = PetriNet::new();
        let place_ref = net.add_place(&"P1".to_string());
        let transition_ref = net.add_transition(&"T1".to_string());

        let result = net.add_arc_place_transition(&place_ref, &transition_ref);
        assert!(result.is_ok());
        let result = net.add_arc_transition_place(&transition_ref, &place_ref);
        assert!(result.is_ok());
        net
    }
}
