use xml_builder::XMLElement;

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
            clumpidx: None,
        }
    }

    pub fn start(&self) -> String {
        self.start.to_owned()
    }

    pub fn channel(&self) -> String {
        self.channel.to_owned()
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

    pub fn as_xmlelement(self) -> XMLElement {
        let mut xml_program = XMLElement::new("programme");
        xml_program.add_attribute("start", &self.start());
        xml_program.add_attribute("channel", &self.channel());

        xml_program
    }
}
