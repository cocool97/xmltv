use crate::{XMLTVChannelDisplayName, xmltv_error::Result};
use xml_builder::XMLElement;

pub struct XMLTVChannel {
    channel: String,
    id: String,
    display_names: Vec<XMLTVChannelDisplayName>,
    icons: Vec<String>,
    urls: Vec<String>
}

impl XMLTVChannel {
    pub fn new(channel: String, id: String, display_name: XMLTVChannelDisplayName) -> Self {
        Self {
            channel,
            id,
            display_names: vec![display_name],
            icons: vec![],
            urls: vec![]
        }
    }

    pub fn add_display_name(&mut self, display_name: XMLTVChannelDisplayName) {
        self.display_names.push(display_name);
    }

    pub fn add_icon(&mut self, icon: String) {
        self.icons.push(icon);
    }

    pub fn add_url(&mut self, url: String) {
        self.urls.push(url);
    }

    pub fn channel(&self) -> String {
        self.channel.to_owned()
    }

    pub fn id(&self) -> String {
        self.id.to_owned()
    }

    pub fn as_xmlelement(self) -> Result<XMLElement> {
        let mut xml_channel = XMLElement::new("channel");
        xml_channel.add_attribute("channel", &self.channel());
        xml_channel.add_attribute("id", &self.id());

        for display_name in self.display_names {
            let mut child = XMLElement::new("display-name");
            if let Some(lang) = display_name.lang() {
                child.add_attribute("lang", &lang);
            }

            child.add_text(display_name.name())?;

            xml_channel.add_child(child)?;
        }

        for url in self.urls {
            let mut child = XMLElement::new("url");
            child.add_text(url).unwrap();

            xml_channel.add_child(child).unwrap();
        }

        for icon in self.icons {
            let mut child = XMLElement::new("icon");
            child.add_text(icon).unwrap();

            xml_channel.add_child(child).unwrap();
        }

        Ok(xml_channel)
    }
}
