#![crate_type = "lib"]
mod channels;
mod programs;
mod xmltv;
mod xmltv_error;

pub use channels::{XMLTVChannel, XMLTVChannelDisplayName};
pub use programs::{
    XMLTVProgram, XMLTVProgramDescription, XMLTVProgramSubTitle, XMLTVProgramTitle,
};
pub use xmltv::XMLTV;
pub use xmltv_error::{Result, XMLTVError};
