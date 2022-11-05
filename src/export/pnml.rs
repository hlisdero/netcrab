use crate::net::PetriNet;
use xml::writer::{EmitterConfig, Error as XmlError, EventWriter, Result as XmlResult, XmlEvent};

const XML_PNML_DEFAULT_NAMESPACE: &str = "http://www.pnml.org/version-2009/grammar/pnml";
const XML_PNML_DEFAULT_GRAMMAR: &str = "http://www.pnml.org/version-2009/grammar/ptnet";

impl PetriNet {
    /// Convert the net to a string in PNML format and return it.
    ///
    /// # Errors
    ///
    /// If the writer fails to write the contents of the net, then an error is returned.
    pub fn to_pnml_string(&self) -> Result<String, XmlError> {
        let mut writer = Vec::new();
        self.to_pnml(&mut writer)?;
        match String::from_utf8(writer) {
            Ok(string) => Ok(string),
            // This error could only be due to a bug, map it to a different error type.
            // Use the Error class from the xml-rs library as a wrapper
            Err(_) => Err(XmlError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not convert the string to UTF-8",
            ))),
        }
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
            self.write_arc(place_ref.as_string(), transition_ref.as_string(), writer)?;
        }

        let arcs = self.find_arcs_transition_place();
        for (transition_ref, place_ref) in arcs {
            self.write_arc(transition_ref.as_string(), place_ref.as_string(), writer)?;
        }

        Ok(())
    }

    /// Write a single arc in the net as a XML node
    /// as required by the PNML standard.
    fn write_arc<T>(
        &self,
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
        xml_writer.write(XmlEvent::Characters(&"1"))?;
        xml_writer.write(XmlEvent::end_element())?;
        xml_writer.write(XmlEvent::end_element())?;
        xml_writer.write(XmlEvent::end_element())?;
        Ok(())
    }

    /// Write the label of a place or transition as a XML node
    /// as required by the PNML standard.
    fn label_to_pnml<T>(name: &String, xml_writer: &mut EventWriter<T>) -> XmlResult<()>
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
    use crate::export::test_utils::assert_all_lines_arbitrary_order;
    use crate::net_creator::*;

    #[test]
    fn pnml_string_empty_net() {
        let net = PetriNet::new();
        let result = net.to_pnml_string();
        assert!(result.is_ok());
        let expected_result = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
        <pnml xmlns=\"http://www.pnml.org/version-2009/grammar/pnml\">\n  \
        <net id=\"net0\" type=\"http://www.pnml.org/version-2009/grammar/ptnet\">\n    \
        <page id=\"page0\" />\n  \
        </net>\n\
        </pnml>"
            .to_string();

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn pnml_string_only_empty_places_net() {
        let mut net = PetriNet::new();
        net.add_place(&"P1".to_string());
        net.add_place(&"P2".to_string());
        net.add_place(&"P3".to_string());
        net.add_place(&"P4".to_string());
        net.add_place(&"P5".to_string());
        let result = net.to_pnml_string();

        assert!(result.is_ok());
        let expected_result = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
        <pnml xmlns=\"http://www.pnml.org/version-2009/grammar/pnml\">\n\
          <net id=\"net0\" type=\"http://www.pnml.org/version-2009/grammar/ptnet\">\n\
            <page id=\"page0\">\n\
              <place id=\"P1\">\n\
                <name>\n\
                  <text>P1</text>\n\
                </name>\n\
              </place>\n\
              <place id=\"P2\">\n\
                <name>\n\
                  <text>P2</text>\n\
                </name>\n\
              </place>\n\
              <place id=\"P3\">\n\
                <name>\n\
                  <text>P3</text>\n\
                </name>\n\
              </place>\n\
              <place id=\"P4\">\n\
                <name>\n\
                  <text>P4</text>\n\
                </name>\n\
              </place>\n\
              <place id=\"P5\">\n\
                <name>\n\
                  <text>P5</text>\n\
                </name>\n\
              </place>\n\
            </page>\n\
          </net>\n\
        </pnml>"
            .to_string();

        assert_all_lines_arbitrary_order(result.unwrap(), expected_result);
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
        let expected_result = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
        <pnml xmlns=\"http://www.pnml.org/version-2009/grammar/pnml\">\n\
          <net id=\"net0\" type=\"http://www.pnml.org/version-2009/grammar/ptnet\">\n\
            <page id=\"page0\">\n\
              <place id=\"P1\">\n\
                <name>\n\
                  <text>P1</text>\n\
                </name>\n\
                <initialMarking>\n\
                  <text>5</text>\n\
                </initialMarking>\n\
              </place>\n\
              <place id=\"P2\">\n\
                <name>\n\
                  <text>P2</text>\n\
                </name>\n\
                <initialMarking>\n\
                  <text>6</text>\n\
                </initialMarking>\n\
              </place>\n\
              <place id=\"P3\">\n\
                <name>\n\
                  <text>P3</text>\n\
                </name>\n\
                <initialMarking>\n\
                  <text>3</text>\n\
                </initialMarking>\n\
              </place>\n\
              <place id=\"P4\">\n\
                <name>\n\
                  <text>P4</text>\n\
                </name>\n\
                <initialMarking>\n\
                  <text>2</text>\n\
                </initialMarking>\n\
              </place>\n\
              <place id=\"P5\">\n\
                <name>\n\
                  <text>P5</text>\n\
                </name>\n\
                <initialMarking>\n\
                  <text>1</text>\n\
                </initialMarking>\n\
              </place>\n\
            </page>\n\
          </net>\n\
        </pnml>"
            .to_string();

        assert_all_lines_arbitrary_order(result.unwrap(), expected_result);
    }

    #[test]
    fn pnml_string_only_empty_transitions_net() {
        let mut net = PetriNet::new();
        net.add_transition(&"T1".to_string());
        net.add_transition(&"T2".to_string());
        net.add_transition(&"T3".to_string());
        net.add_transition(&"T4".to_string());
        net.add_transition(&"T5".to_string());
        let result = net.to_pnml_string();

        assert!(result.is_ok());
        let expected_result = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
        <pnml xmlns=\"http://www.pnml.org/version-2009/grammar/pnml\">\n\
          <net id=\"net0\" type=\"http://www.pnml.org/version-2009/grammar/ptnet\">\n\
            <page id=\"page0\">\n\
              <transition id=\"T1\">\n\
                <name>\n\
                  <text>T1</text>\n\
                </name>\n\
              </transition>\n\
              <transition id=\"T2\">\n\
                <name>\n\
                  <text>T2</text>\n\
                </name>\n\
              </transition>\n\
              <transition id=\"T3\">\n\
                <name>\n\
                  <text>T3</text>\n\
                </name>\n\
              </transition>\n\
              <transition id=\"T4\">\n\
                <name>\n\
                  <text>T4</text>\n\
                </name>\n\
              </transition>\n\
              <transition id=\"T5\">\n\
                <name>\n\
                  <text>T5</text>\n\
                </name>\n\
              </transition>\n\
            </page>\n\
          </net>\n\
        </pnml>"
            .to_string();

        assert_all_lines_arbitrary_order(result.unwrap(), expected_result);
    }

    #[test]
    fn pnml_string_net_with_chain_topology() {
        let net = create_net_chain_topology(3);
        let result = net.to_pnml_string();

        assert!(result.is_ok());
        let expected_result = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
        <pnml xmlns=\"http://www.pnml.org/version-2009/grammar/pnml\">\n\
          <net id=\"net0\" type=\"http://www.pnml.org/version-2009/grammar/ptnet\">\n\
            <page id=\"page0\">\n\
              <place id=\"P1\">\n\
                <name>\n\
                  <text>P1</text>\n\
                </name>\n\
              </place>\n\
              <place id=\"P2\">\n\
                <name>\n\
                  <text>P2</text>\n\
                </name>\n\
              </place>\n\
              <place id=\"P3\">\n\
                <name>\n\
                  <text>P3</text>\n\
                </name>\n\
              </place>\n\
              <transition id=\"T1\">\n\
                <name>\n\
                  <text>T1</text>\n\
                </name>\n\
              </transition>\n\
              <transition id=\"T2\">\n\
                <name>\n\
                  <text>T2</text>\n\
                </name>\n\
              </transition>\n\
              <arc source=\"P1\" target=\"T1\" id=\"(P1, T1)\">\n\
                <name>\n\
                  <text>(P1, T1)</text>\n\
                </name>\n\
                <inscription>\n\
                  <text>1</text>\n\
                </inscription>\n\
              </arc>\n\
              <arc source=\"P2\" target=\"T2\" id=\"(P2, T2)\">\n\
                <name>\n\
                  <text>(P2, T2)</text>\n\
                </name>\n\
                <inscription>\n\
                  <text>1</text>\n\
                </inscription>\n\
              </arc>\n\
              <arc source=\"T1\" target=\"P2\" id=\"(T1, P2)\">\n\
                <name>\n\
                  <text>(T1, P2)</text>\n\
                </name>\n\
                <inscription>\n\
                  <text>1</text>\n\
                </inscription>\n\
              </arc>\n\
              <arc source=\"T2\" target=\"P3\" id=\"(T2, P3)\">\n\
                <name>\n\
                  <text>(T2, P3)</text>\n\
                </name>\n\
                <inscription>\n\
                  <text>1</text>\n\
                </inscription>\n\
              </arc>\n\
            </page>\n\
          </net>\n\
        </pnml>"
            .to_string();

        assert_all_lines_arbitrary_order(result.unwrap(), expected_result);
    }

    #[test]
    fn pnml_string_net_with_loop_topology() {
        let net = create_net_loop_topology();
        let result = net.to_pnml_string();

        assert!(result.is_ok());
        let expected_result = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
        <pnml xmlns=\"http://www.pnml.org/version-2009/grammar/pnml\">\n\
          <net id=\"net0\" type=\"http://www.pnml.org/version-2009/grammar/ptnet\">\n\
            <page id=\"page0\">\n\
              <place id=\"P1\">\n\
                <name>\n\
                  <text>P1</text>\n\
                </name>\n\
              </place>\n\
              <transition id=\"T1\">\n\
                <name>\n\
                  <text>T1</text>\n\
                </name>\n\
              </transition>\n\
              <arc source=\"P1\" target=\"T1\" id=\"(P1, T1)\">\n\
                <name>\n\
                  <text>(P1, T1)</text>\n\
                </name>\n\
                <inscription>\n\
                  <text>1</text>\n\
                </inscription>\n\
              </arc>\n\
              <arc source=\"T1\" target=\"P1\" id=\"(T1, P1)\">\n\
                <name>\n\
                  <text>(T1, P1)</text>\n\
                </name>\n\
                <inscription>\n\
                  <text>1</text>\n\
                </inscription>\n\
              </arc>\n\
            </page>\n\
          </net>\n\
        </pnml>"
            .to_string();

        assert_all_lines_arbitrary_order(result.unwrap(), expected_result);
    }
}
