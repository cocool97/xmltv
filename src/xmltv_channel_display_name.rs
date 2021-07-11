pub struct XMLTVChannelDisplayName {
    name: String,
    lang: Option<String>,
}

impl XMLTVChannelDisplayName {
    pub fn new(name: String, lang: Option<String>) -> Self {
        XMLTVChannelDisplayName {
            name,
            lang
        }
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn lang(&self) -> Option<String> {
        self.lang.to_owned()
    }
}