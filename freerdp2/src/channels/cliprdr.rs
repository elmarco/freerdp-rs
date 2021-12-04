use bitflags::bitflags;

use crate::sys;

pub const SVC_CHANNEL_NAME: &'static str = "cliprdr";

#[non_exhaustive]
#[derive(Debug)]
pub enum Format {
    Html = sys::CB_FORMAT_HTML as _,
    Png = sys::CB_FORMAT_PNG as _,
    Jpeg = sys::CB_FORMAT_JPEG as _,
    Gif = sys::CB_FORMAT_GIF as _,
    TextUriList = sys::CB_FORMAT_TEXTURILIST as _,
}

bitflags! {
    pub struct GeneralCapabilities: u32 {
        const USE_LONG_FORMAT_NAMES = 0b00000010;
        const STREAM_FILECLIP_ENABLED = 0b00000100;
        const FILECLIP_NO_FILE_PATHS = 0b00001000;
        const CAN_LOCK_CLIPDATA = 0b00010000;
        const HUGE_FILE_SUPPORT_ENABLED = 0b00100000;
    }
}


#[non_exhaustive]
#[derive(Debug)]
pub enum Version {
    V1 = sys::CB_CAPS_VERSION_1 as _,
    V2 = sys::CB_CAPS_VERSION_2 as _,
}
