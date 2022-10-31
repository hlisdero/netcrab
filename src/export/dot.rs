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
        let string = String::from_utf8(writer);
        match string {
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
        writer.write_all("}".as_bytes())?;
        Ok(())
    }

    /// Write the lines that define the places
    /// to a trait object which implements `std::io::Write`.
    fn write_dot_places<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        for (_, place) in self.places_iter() {
            let label = Self::label_to_string(&place.label);
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
        for (_, transition) in self.transitions_iter() {
            let label = Self::label_to_string(&transition.label);
            let line = format!("    {} [shape=\"box\" xlabel=\"{}\"];\n", label, label);
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
        let edges = self.find_edges_place_transition();
        for (place_ref, transition_ref) in edges {
            let line = format!(
                "    {} -> {};\n",
                place_ref.to_string(),
                transition_ref.to_string()
            );
            writer.write(line.as_bytes())?;
        }

        let edges = self.find_edges_transition_place();
        for (transition_ref, place_ref) in edges {
            let line = format!(
                "    {} -> {};\n",
                transition_ref.to_string(),
                place_ref.to_string()
            );
            writer.write(line.as_bytes())?;
        }

        Ok(())
    }

    /// Convert the label if present to a valid `String`.
    /// Remove newlines and quotes from the label.
    ///
    /// Using escape sequences it is possible to achieve special behavior.
    /// [More info](https://graphviz.org/docs/attr-types/escString/)
    fn label_to_string(option: &Option<String>) -> String {
        match option {
            Some(label) => label.replace('\n', "").replace('\"', "\\\""),
            None => String::from("unnamed"),
        }
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
