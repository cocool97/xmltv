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
    // title: Vec<String>,
    // sub_title: Vec<String>,
    // desc: Vec<String>,
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
    pub fn new(channel: String, start: String) -> Self {
        Self {
            start,
            stop: None,
            pdc_start: None,
            vps_start: None,
            showview: None,
            videoplus: None,
            channel,
            clumpidx: None,
        }
    }

    pub fn start(&self) -> String {
        self.start.to_owned()
    }

    pub fn stop(&self) -> Option<String> {
        self.stop.to_owned()
    }

    pub fn pdc_start(&self) -> Option<String> {
        self.pdc_start.to_owned()
    }

    pub fn vps_start(&self) -> Option<String> {
        self.vps_start.to_owned()
    }

    pub fn showview(&self) -> Option<String> {
        self.showview.to_owned()
    }

    pub fn videoplus(&self) -> Option<String> {
        self.videoplus.to_owned()
    }

    pub fn channel(&self) -> String {
        self.channel.to_owned()
    }

    pub fn clumpidx(&self) -> Option<String> {
        self.clumpidx.to_owned()
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

    pub(crate) fn as_xmlelement(self) -> XMLElement {
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

        xml_program
    }
}
