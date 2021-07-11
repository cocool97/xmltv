#![crate_type = "lib"]
pub use xmltv::XMLTV;
pub use xmltv_channel::XMLTVChannel;
pub use xmltv_channel_display_name::XMLTVChannelDisplayName;
pub use xmltv_error::{Result, XMLTVError};
pub use xmltv_program::XMLTVProgram;

mod xmltv;
mod xmltv_channel;
mod xmltv_channel_display_name;
mod xmltv_error;
mod xmltv_program;
