use xml_builder::XMLError;

pub type Result<T> = std::result::Result<T, XMLTVError>;

#[derive(Debug)]
pub enum XMLTVError {
    XMLError(XMLError),
}

impl From<XMLError> for XMLTVError {
    fn from(e: XMLError) -> Self {
        Self::XMLError(e)
    }
}
