use crate::petri_net::PetriNet;
use xml::writer::{EmitterConfig, EventWriter, Result as XmlResult, XmlEvent};

const XML_PNML_DEFAULT_NAMESPACE: &str = "http://www.pnml.org/version-2009/grammar/pnml";
const XML_PNML_DEFAULT_GRAMMAR: &str = "http://www.pnml.org/version-2009/grammar/ptnet";

impl PetriNet {
    /// Convert the net to a string in PNML format and return it.
    ///
    /// # Errors
    ///
    /// If the writer fails to write the contents of the net, then an error is returned.
    pub fn to_pnml_string(&self) -> Result<String, std::io::Error> {
        let mut writer = Vec::new();
        self.to_pnml(&mut writer).map_err(|_|
            std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not convert the net to PNML",
        ))?;
        String::from_utf8(writer).map_err(|_| 
            // This error could only be due to a bug, map it to a more standard error type.
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not convert the string to UTF-8",
            )
        )
    }

    /// Convert the net to the PNML format.
    /// Write the output to a trait object which implements `std::io::Write`.
    ///
    /// # Errors
    ///
    /// If the writer fails to write the contents of the net, then an error is returned.
    pub fn to_pnml<T>(&self, writer: &mut T) -> XmlResult<()>
    where
        T: std::io::Write,
    {
        let mut xml_writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(writer);

        // Initialize general properties of the XML.
        xml_writer.write(XmlEvent::start_element("pnml").default_ns(XML_PNML_DEFAULT_NAMESPACE))?;
        xml_writer.write(
            XmlEvent::start_element("net")
                .attr("id", "net0")
                .attr("type", XML_PNML_DEFAULT_GRAMMAR),
        )?;
        xml_writer.write(XmlEvent::start_element("page").attr("id", "page0"))?;

        self.write_pnml_places(&mut xml_writer)?;
        self.write_pnml_transitions(&mut xml_writer)?;
        self.write_pnml_arcs(&mut xml_writer)?;

        // Close the tags of the general properties of the XML.
        xml_writer.write(XmlEvent::end_element())?;
        xml_writer.write(XmlEvent::end_element())?;
        xml_writer.write(XmlEvent::end_element())?;
        Ok(())
    }

    /// Write the XML elements that define the places
    /// to an instance of `xml::writer::Writer`.
    fn write_pnml_places<T>(&self, writer: &mut EventWriter<T>) -> XmlResult<()>
    where
        T: std::io::Write,
    {
        for (place_ref, place) in self.places_iter() {
            let place_xml_element =
                XmlEvent::start_element("place").attr("id", place_ref.as_string());
            writer.write(place_xml_element)?;
            Self::label_to_pnml(place_ref.as_string(), writer)?;
            Self::marking_to_pnml(place.marking(), writer)?;
            writer.write(XmlEvent::end_element())?;
        }
        Ok(())
    }

    /// Write the XML elements that define the transitions
    /// to an instance of `xml::writer::Writer`.
    fn write_pnml_transitions<T>(&self, writer: &mut EventWriter<T>) -> XmlResult<()>
    where
        T: std::io::Write,
    {
        for (transition_ref, _) in self.transitions_iter() {
            let transition_xml_element =
                XmlEvent::start_element("transition").attr("id", transition_ref.as_string());
            writer.write(transition_xml_element)?;
            Self::label_to_pnml(transition_ref.as_string(), writer)?;
            writer.write(XmlEvent::end_element())?;
        }
        Ok(())
    }

    /// Write the XML elements that define the arcs
    /// to an instance of `xml::writer::Writer`.
    fn write_pnml_arcs<T>(&self, writer: &mut EventWriter<T>) -> XmlResult<()>
    where
        T: std::io::Write,
    {
        let arcs = self.find_arcs_place_transition();
        for (place_ref, transition_ref) in arcs {
            Self::write_arc(place_ref.as_string(), transition_ref.as_string(), writer)?;
        }

        let arcs = self.find_arcs_transition_place();
        for (transition_ref, place_ref) in arcs {
            Self::write_arc(transition_ref.as_string(), place_ref.as_string(), writer)?;
        }

        Ok(())
    }

    /// Write a single arc in the net as a XML node
    /// as required by the PNML standard.
    fn write_arc<T>(
        source: &String,
        dest: &String,
        xml_writer: &mut EventWriter<T>,
    ) -> XmlResult<()>
    where
        T: std::io::Write,
    {
        let arc_label = format!("({}, {})", source, dest);
        let start_element = XmlEvent::start_element("arc")
            .attr("source", source)
            .attr("target", dest)
            .attr("id", &arc_label);
        xml_writer.write(start_element)?;
        Self::label_to_pnml(&arc_label, xml_writer)?;
        xml_writer.write(XmlEvent::start_element("inscription"))?;
        xml_writer.write(XmlEvent::start_element("text"))?;
        // Weights in arcs are not supported for now.
        xml_writer.write(XmlEvent::Characters("1"))?;
        xml_writer.write(XmlEvent::end_element())?;
        xml_writer.write(XmlEvent::end_element())?;
        xml_writer.write(XmlEvent::end_element())?;
        Ok(())
    }

    /// Write the label of a place or transition as a XML node
    /// as required by the PNML standard.
    fn label_to_pnml<T>(name: &str, xml_writer: &mut EventWriter<T>) -> XmlResult<()>
    where
        T: std::io::Write,
    {
        xml_writer.write(XmlEvent::start_element("name"))?;
        xml_writer.write(XmlEvent::start_element("text"))?;
        xml_writer.write(XmlEvent::Characters(name))?;
        xml_writer.write(XmlEvent::end_element())?;
        xml_writer.write(XmlEvent::end_element())?;
        Ok(())
    }

    /// Write the marking of a place as a XML node
    /// as required by the PNML standard.
    fn marking_to_pnml<T>(marking: usize, xml_writer: &mut EventWriter<T>) -> XmlResult<()>
    where
        T: std::io::Write,
    {
        if marking == 0 {
            return Ok(());
        }
        xml_writer.write(XmlEvent::start_element("initialMarking"))?;
        xml_writer.write(XmlEvent::start_element("text"))?;
        xml_writer.write(XmlEvent::Characters(&marking.to_string()))?;
        xml_writer.write(XmlEvent::end_element())?;
        xml_writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

#[cfg(test)]
mod pnml_tests {
    use super::*;
    use crate::export::test_export_examples::*;
    use crate::net_creator::*;

    #[test]
    fn pnml_string_empty_net() {
        let net = PetriNet::new();
        let result = net.to_pnml_string();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PNML_STRING_EMPTY_NET);
    }

    #[test]
    fn pnml_string_only_empty_places_net() {
        let net = create_basic_unconnected_net(5, 0);
        let result = net.to_pnml_string();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PNML_STRING_ONLY_EMPTY_PLACES_NET);
    }

    #[test]
    fn pnml_string_marked_places_net() {
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
        let result = net.to_pnml_string();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PNML_STRING_MARKED_PLACES_NET);
    }

    #[test]
    fn pnml_string_only_empty_transitions_net() {
        let net = create_basic_unconnected_net(0, 5);
        let result = net.to_pnml_string();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PNML_STRING_ONLY_EMPTY_TRANSITIONS_NET);
    }

    #[test]
    fn pnml_string_net_with_chain_topology() {
        let net = create_net_chain_topology(3);
        let result = net.to_pnml_string();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PNML_STRING_NET_WITH_CHAIN_TOPOLOPY);
    }

    #[test]
    fn pnml_string_net_with_loop_topology() {
        let net = create_net_loop_topology();
        let result = net.to_pnml_string();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PNML_STRING_NET_WITH_LOOP_TOPOLOGY);
    }
}
