use crate::{
    xmltv_error::Result, XMLTVProgramCategory, XMLTVProgramCredits, XMLTVProgramDescription,
    XMLTVProgramIcon, XMLTVProgramKeyword, XMLTVProgramLanguage, XMLTVProgramLength,
    XMLTVProgramOrigLanguage, XMLTVProgramSubTitle, XMLTVProgramTitle,
};
use xml_builder::XMLElement;

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
    credits: Option<XMLTVProgramCredits>,
    date: Option<String>,
    categories: Vec<XMLTVProgramCategory>,
    keywords: Vec<XMLTVProgramKeyword>,
    language: Option<XMLTVProgramLanguage>,
    orig_language: Option<XMLTVProgramOrigLanguage>,
    length: Option<XMLTVProgramLength>,
    icons: Vec<XMLTVProgramIcon>,
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
            descs: vec![],
            credits: None,
            date: None,
            categories: vec![],
            keywords: vec![],
            language: None,
            orig_language: None,
            length: None,
            icons: vec![],
        }
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

    pub fn add_credits(&mut self, credits: XMLTVProgramCredits) {
        self.credits = Some(credits);
    }

    pub fn add_date(&mut self, date: String) {
        self.date = Some(date);
    }

    pub fn add_category(&mut self, category: XMLTVProgramCategory) {
        self.categories.push(category);
    }

    pub fn add_keyword(&mut self, keyword: XMLTVProgramKeyword) {
        self.keywords.push(keyword);
    }

    pub fn set_language(&mut self, language: XMLTVProgramLanguage) {
        self.language = Some(language);
    }

    pub fn set_orig_language(&mut self, orig_language: XMLTVProgramOrigLanguage) {
        self.orig_language = Some(orig_language);
    }

    pub fn set_length(&mut self, length: XMLTVProgramLength) {
        self.length = Some(length);
    }

    pub fn add_icon(&mut self, icon: XMLTVProgramIcon) {
        self.icons.push(icon);
    }

    pub fn to_xmlelement(self) -> Result<XMLElement> {
        let mut xml_program = XMLElement::new("programme");
        // Mandatory element attributes
        xml_program.add_attribute("start", &self.start);
        xml_program.add_attribute("channel", &self.channel);

        if let Some(stop) = self.stop {
            xml_program.add_attribute("stop", &stop);
        }

        if let Some(pdc_start) = self.pdc_start {
            xml_program.add_attribute("pdc-start", &pdc_start);
        }

        if let Some(vps_start) = self.vps_start {
            xml_program.add_attribute("vps-start", &vps_start);
        }

        if let Some(showview) = self.showview {
            xml_program.add_attribute("showview", &showview);
        }

        if let Some(videoplus) = self.videoplus {
            xml_program.add_attribute("videoplus", &videoplus);
        }

        if let Some(clumpidx) = self.clumpidx {
            xml_program.add_attribute("clumpidx", &clumpidx);
        }

        for title in self.titles {
            let mut element = XMLElement::new("title");
            if let Some(lang) = &title.lang {
                element.add_attribute("lang", lang);
            }

            element.add_text(title.title.clone())?;

            xml_program.add_child(element)?;
        }

        for sub_title in self.sub_titles {
            let mut element = XMLElement::new("sub-title");
            if let Some(lang) = &sub_title.lang {
                element.add_attribute("lang", lang);
            }

            element.add_text(sub_title.title.clone())?;

            xml_program.add_child(element)?;
        }

        for desc in self.descs {
            let mut element = XMLElement::new("desc");

            if let Some(lang) = &desc.lang {
                element.add_attribute("lang", lang);
            }

            element.add_text(desc.desc.clone())?;

            xml_program.add_child(element)?;
        }

        if let Some(credits) = self.credits {
            xml_program.add_child(credits.to_xmlelement()?)?;
        }

        if let Some(date) = self.date {
            let mut element = XMLElement::new("date");
            element.add_text(date.to_owned())?;

            xml_program.add_child(element)?;
        }

        for category in self.categories {
            let mut element = XMLElement::new("category");

            if let Some(lang) = &category.lang {
                element.add_attribute("lang", lang);
            }

            element.add_text(category.category.to_owned())?;

            xml_program.add_child(element)?;
        }

        for keyword in self.keywords {
            let mut element = XMLElement::new("keyword");

            if let Some(lang) = &keyword.lang {
                element.add_attribute("lang", lang);
            }

            element.add_text(keyword.keyword.to_owned())?;

            xml_program.add_child(element)?;
        }

        if let Some(language) = self.language {
            let mut element = XMLElement::new("language");

            if let Some(lang) = &language.lang {
                element.add_attribute("lang", lang);
            }

            element.add_text(language.language.to_owned())?;

            xml_program.add_child(element)?;
        }

        if let Some(orig_language) = self.orig_language {
            let mut element = XMLElement::new("orig_language");

            if let Some(lang) = &orig_language.lang {
                element.add_attribute("lang", lang);
            }

            element.add_text(orig_language.orig_language.to_owned())?;

            xml_program.add_child(element)?;
        }

        if let Some(length) = self.length {
            let mut element = XMLElement::new("length");

            element.add_attribute("units", &length.units.to_string());
            element.add_text(length.length.to_string())?;

            xml_program.add_child(element)?;
        }

        for icon in self.icons {
            let mut element = XMLElement::new("icon");
            element.add_attribute("src", &icon.src);

            if let Some(width) = icon.width {
                element.add_attribute("width", &width);
            }

            if let Some(height) = icon.height {
                element.add_attribute("height", &height);
            }

            xml_program.add_child(element)?;
        }

        Ok(xml_program)
    }
}
