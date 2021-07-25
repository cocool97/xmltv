pub struct XMLTVProgramLength {
    pub(crate) length: u16,
    pub(crate) units: XMLTVProgramUnits,
}

pub enum XMLTVProgramUnits {
    Seconds,
    Minutes,
    Hours,
}

impl ToString for XMLTVProgramUnits {
    fn to_string(&self) -> String {
        match self {
            XMLTVProgramUnits::Seconds => String::from("seconds"),
            XMLTVProgramUnits::Minutes => String::from("minutes"),
            XMLTVProgramUnits::Hours => String::from("hours"),
        }
    }
}
