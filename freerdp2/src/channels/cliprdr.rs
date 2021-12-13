use bitflags::bitflags;

use crate::{sys, RdpError};

pub const SVC_CHANNEL_NAME: &'static str = "cliprdr";

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Format {
    Raw = sys::CF_RAW as _,
    Text = sys::CF_TEXT as _,
    Bitmap = sys::CF_BITMAP as _,
    MetaFilePict = sys::CF_METAFILEPICT as _,
    Sylk = sys::CF_SYLK as _,
    Dif = sys::CF_DIF as _,
    Tiff = sys::CF_TIFF as _,
    OemText = sys::CF_OEMTEXT as _,
    Dib = sys::CF_DIB as _,
    Palette = sys::CF_PALETTE as _,
    PenData = sys::CF_PENDATA as _,
    Riff = sys::CF_RIFF as _,
    Wave = sys::CF_WAVE as _,
    UnicodeText = sys::CF_UNICODETEXT as _,
    EnhMetaFile = sys::CF_ENHMETAFILE as _,
    HDrop = sys::CF_HDROP as _,
    Locale = sys::CF_LOCALE as _,
    DibV5 = sys::CF_DIBV5 as _,
    Max = sys::CF_MAX as _,
    OwnedDisplay = sys::CF_OWNERDISPLAY as _,
    DspText = sys::CF_DSPTEXT as _,
    Html = sys::CB_FORMAT_HTML as _, // D010
    Png = sys::CB_FORMAT_PNG as _,
    Jpeg = sys::CB_FORMAT_JPEG as _,
    Gif = sys::CB_FORMAT_GIF as _,
    TextUriList = sys::CB_FORMAT_TEXTURILIST as _,
}

impl TryFrom<u32> for Format {
    type Error = RdpError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            v if v == Format::Raw as u32 => Ok(Format::Raw),
            v if v == Format::Text as u32 => Ok(Format::Text),
            v if v == Format::Bitmap as u32 => Ok(Format::Bitmap),
            v if v == Format::MetaFilePict as u32 => Ok(Format::MetaFilePict),
            v if v == Format::Sylk as u32 => Ok(Format::Sylk),
            v if v == Format::Dif as u32 => Ok(Format::Dif),
            v if v == Format::Tiff as u32 => Ok(Format::Tiff),
            v if v == Format::OemText as u32 => Ok(Format::OemText),
            v if v == Format::Dib as u32 => Ok(Format::Dib),
            v if v == Format::Palette as u32 => Ok(Format::Palette),
            v if v == Format::PenData as u32 => Ok(Format::PenData),
            v if v == Format::Riff as u32 => Ok(Format::Riff),
            v if v == Format::Wave as u32 => Ok(Format::Wave),
            v if v == Format::UnicodeText as u32 => Ok(Format::UnicodeText),
            v if v == Format::EnhMetaFile as u32 => Ok(Format::EnhMetaFile),
            v if v == Format::HDrop as u32 => Ok(Format::HDrop),
            v if v == Format::Locale as u32 => Ok(Format::Locale),
            v if v == Format::DibV5 as u32 => Ok(Format::DibV5),
            v if v == Format::Max as u32 => Ok(Format::Max),
            v if v == Format::OwnedDisplay as u32 => Ok(Format::OwnedDisplay),
            v if v == Format::DspText as u32 => Ok(Format::DspText),
            v if v == Format::Html as u32 => Ok(Format::Html),
            v if v == Format::Png as u32 => Ok(Format::Png),
            v if v == Format::Jpeg as u32 => Ok(Format::Jpeg),
            v if v == Format::Gif as u32 => Ok(Format::Gif),
            v if v == Format::TextUriList as u32 => Ok(Format::TextUriList),
            _ => Err(RdpError::Unsupported),
        }
    }
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
