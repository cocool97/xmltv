use std::io::Write;

use xml_builder::{XML, XMLElement, XMLError};

type Result<T> = std::result::Result<T, XMLTVError>;

pub enum XMLTVError {
    XMLError(XMLError)
}

impl From<XMLError> for XMLTVError {
    fn from(e: XMLError) -> Self {
        Self::XMLError(e)
    }
}

pub struct XMLTV {
    xml: XML,
    root_element: XMLElement
}

impl Default for XMLTV {
    fn default() -> Self {
        let xml = XML::new();
        let root_element = XMLElement::new("tv");

        Self {
            xml,
            root_element
        }
    }
}

impl XMLTV {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_date(&mut self, date: String) {
        self.root_element.add_attribute("date", &date);
    }

    pub fn add_source_info_url(&mut self, source_info_url: String) {
        self.root_element.add_attribute("source-info-url", &source_info_url);
    }

    pub fn add_source_info_name(&mut self, source_info_name: String) {
        self.root_element.add_attribute("source-info-name", &source_info_name);
    }

    pub fn add_source_data_url(&mut self, source_data_url: String) {
        self.root_element.add_attribute("source-data-url", &source_data_url);
    }

    pub fn add_generator_info_name(&mut self, generator_info_name: String) {
        self.root_element.add_attribute("generator-info-name", &generator_info_name);
    }

    pub fn add_generator_info_url(&mut self, generator_info_url: String) {
        self.root_element.add_attribute("generator-info-url", &generator_info_url);
    }

    pub fn add_channel(&mut self, channel: XMLTVChannel) -> Result<()> {
        let mut xml_channel = XMLElement::new("channel");
        xml_channel.add_attribute("channel", &channel.channel);
        xml_channel.add_attribute("id", &channel.id);

        Ok(self.root_element.add_child(xml_channel)?)
    }

    pub fn add_program(&mut self, program: XMLTVProgram) -> Result<()> {
        let mut xml_program = XMLElement::new("programme");
        xml_program.add_attribute("start", &program.start);
        xml_program.add_attribute("channel", &program.channel);

        Ok(self.root_element.add_child(xml_program)?)
    }

    pub fn build<W: Write>(mut self, writer: W) -> Result<()> {
        self.xml.set_root_element(self.root_element);

        Ok(self.xml.build(writer)?)
    }
}

pub struct XMLTVChannel {
    channel: String,
    id: String
}

impl XMLTVChannel {
    pub fn new(channel: String, id: String) -> Self {
        Self {
            channel,
            id
        }
    }
}

pub struct XMLTVProgram {
    start: String,
    stop: Option<String>,
    pdc_start: Option<String>,
    vps_start: Option<String>,
    showview: Option<String>,
    videoplus: Option<String>,
    channel: String,
    clumpidx: Option<String>,
}

impl XMLTVProgram {
    pub fn new(start: String, channel: String) -> Self {
        Self {
            start,
            stop: None,
            pdc_start: None,
            vps_start: None,
            showview: None,
            videoplus: None,
            channel,
            clumpidx: None
        }
    }

    pub fn add_stop_date(&mut self, stop: String) {
        self.stop = Some(stop);
    }

    pub fn add_pdc_start(&mut self, pdc_start: String) {
        self.pdc_start = Some(pdc_start);
    }

    pub fn add_vps_start(&mut self, vps_start: String) {
        self.vps_start = Some(vps_start);
    }

    pub fn add_showview(&mut self, showview: String) {
        self.showview = Some(showview);
    }

    pub fn add_videoplus(&mut self, videoplus: String) {
        self.videoplus = Some(videoplus);
    }

    pub fn add_clumpidx(&mut self, clumpidx: String) {
        self.clumpidx = Some(clumpidx);
    }
}

#[cfg(test)]
mod tests {}
