use crate::xmltv_error::Result;
use xml_builder::XMLElement;

use super::{XMLTVProgramDescription, XMLTVProgramSubTitle, XMLTVProgramTitle};

/// Structure representing a XMLTV program element.
///
/// Two fields are mandatory :
/// * start
/// * channel
///
pub struct XMLTVProgram {
    // Attributes
    start: String,
    stop: Option<String>,
    pdc_start: Option<String>,
    vps_start: Option<String>,
    showview: Option<String>,
    videoplus: Option<String>,
    channel: String,
    clumpidx: Option<String>,
    // Childs
    titles: Vec<XMLTVProgramTitle>,
    sub_titles: Vec<XMLTVProgramSubTitle>,
    descs: Vec<XMLTVProgramDescription>,
    // credits: Option<String>,
    // date: Option<String>,
    // category: Vec<String>,
    // keyword: Vec<String>,
    // language: Option<String>,
    // orig_language: Option<String>,
    // length: Option<String>,
    // icon: Vec<String>,
    // url: Vec<String>,
    // country: Vec<String>,
    // episode_num: Vec<String>,
    // video: Option<String>,
    // audio: Option<String>,
    // previously_shown: Option<String>,
    // premiere: Option<String>,
    // last_chance: Option<String>,
    // new: Option<String>,
    // subtitles: Vec<String>,
    // rating: Vec<String>,
    // star_rating: Vec<String>,
    // review: Vec<String>
}

impl XMLTVProgram {
    pub fn new(channel: String, start: String, title: XMLTVProgramTitle) -> Self {
        Self {
            start,
            stop: None,
            pdc_start: None,
            vps_start: None,
            showview: None,
            videoplus: None,
            channel,
            clumpidx: None,
            titles: vec![title],
            sub_titles: vec![],
            descs: vec![]
        }
    }

    pub fn start(&self) -> &String {
        &self.start
    }

    pub fn stop(&self) -> Option<&String> {
        self.stop.as_ref()
    }

    pub fn pdc_start(&self) -> Option<&String> {
        self.pdc_start.as_ref()
    }

    pub fn vps_start(&self) -> Option<&String> {
        self.vps_start.as_ref()
    }

    pub fn showview(&self) -> Option<&String> {
        self.showview.as_ref()
    }

    pub fn videoplus(&self) -> Option<&String> {
        self.videoplus.as_ref()
    }

    pub fn channel(&self) -> &String {
        &self.channel
    }

    pub fn clumpidx(&self) -> Option<&String> {
        self.clumpidx.as_ref()
    }

    pub fn titles(&self) -> &Vec<XMLTVProgramTitle> {
        &self.titles
    }

    pub fn sub_titles(&self) -> &Vec<XMLTVProgramSubTitle> {
        &self.sub_titles
    }

    pub fn descs(&self) -> &Vec<XMLTVProgramDescription> {
        &self.descs
    }

    pub fn set_start(&mut self, start: String) {
        self.start = start;
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

    pub fn set_channel(&mut self, channel: String) {
        self.channel = channel;
    }

    pub fn set_clumpidx(&mut self, clumpidx: String) {
        self.clumpidx = Some(clumpidx);
    }

    pub fn add_title(&mut self, title: XMLTVProgramTitle) {
        self.titles.push(title);
    }

    pub fn add_sub_title(&mut self, sub_title: XMLTVProgramSubTitle) {
        self.sub_titles.push(sub_title);
    }

    pub fn add_desc(&mut self, desc: XMLTVProgramDescription) {
        self.descs.push(desc);
    }

    pub fn as_xmlelement(self) -> Result<XMLElement> {
        let mut xml_program = XMLElement::new("programme");
        // Mandatory element attributes
        xml_program.add_attribute("start", &self.start());
        xml_program.add_attribute("channel", &self.channel());

        if let Some(stop) = self.stop() {
            xml_program.add_attribute("stop", &stop);
        }

        if let Some(pdc_start) = self.pdc_start() {
            xml_program.add_attribute("pdc-start", &pdc_start);
        }

        if let Some(vps_start) = self.vps_start() {
            xml_program.add_attribute("vps-start", &vps_start);
        }

        if let Some(showview) = self.showview() {
            xml_program.add_attribute("showview", &showview);
        }

        if let Some(videoplus) = self.videoplus() {
            xml_program.add_attribute("videoplus", &videoplus);
        }

        if let Some(clumpidx) = self.clumpidx() {
            xml_program.add_attribute("clumpidx", &clumpidx);
        }

        for title in self.titles() {
            let mut element = XMLElement::new("title");
            if let Some(lang) = &title.lang {
                element.add_attribute("lang", lang);
            }

            element.add_text(title.title.clone())?;

            xml_program.add_child(element)?;
        }

        for sub_title in self.sub_titles() {
            let mut element = XMLElement::new("sub-title");
            if let Some(lang) = &sub_title.lang {
                element.add_attribute("lang", lang);
            }

            element.add_text(sub_title.title.clone())?;

            xml_program.add_child(element)?;
        }

        for desc in self.descs() {
            let mut element = XMLElement::new("desc");

            if let Some(lang) = &desc.lang {
                element.add_attribute("lang", lang);
            }

            element.add_text(desc.desc.clone())?;

            xml_program.add_child(element)?;
        }

        Ok(xml_program)
    }
}
