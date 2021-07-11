use std::io::Write;

use xml_builder::{XMLElement, XML};

use crate::{Result, XMLTVChannel, XMLTVProgram};

pub struct XMLTV {
    xml: XML,
    root_element: XMLElement,
}

impl Default for XMLTV {
    fn default() -> Self {
        let mut xml = XML::new();
        xml.set_header_rendering(false);
        xml.set_attribute_sorting(true);

        let root_element = XMLElement::new("tv");

        Self { xml, root_element }
    }
}

impl XMLTV {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_date(&mut self, date: String) {
        self.root_element.add_attribute("date", &date);
    }

    pub fn add_source_info_url(&mut self, source_info_url: String) {
        self.root_element
            .add_attribute("source-info-url", &source_info_url);
    }

    pub fn add_source_info_name(&mut self, source_info_name: String) {
        self.root_element
            .add_attribute("source-info-name", &source_info_name);
    }

    pub fn add_source_data_url(&mut self, source_data_url: String) {
        self.root_element
            .add_attribute("source-data-url", &source_data_url);
    }

    pub fn add_generator_info_name(&mut self, generator_info_name: String) {
        self.root_element
            .add_attribute("generator-info-name", &generator_info_name);
    }

    pub fn add_generator_info_url(&mut self, generator_info_url: String) {
        self.root_element
            .add_attribute("generator-info-url", &generator_info_url);
    }

    pub fn add_channel(&mut self, channel: XMLTVChannel) -> Result<()> {
        Ok(self.root_element.add_child(channel.as_xmlelement()?)?)
    }

    pub fn add_program(&mut self, program: XMLTVProgram) -> Result<()> {
        Ok(self.root_element.add_child(program.as_xmlelement())?)
    }

    pub fn render<W: Write>(mut self, writer: &mut W) -> Result<()> {
        self.xml.set_root_element(self.root_element);

        Ok(self.xml.build(writer)?)
    }

    pub fn build<W: Write>(self, writer: &mut W) -> Result<()> {
        self.render(writer)
    }
}
