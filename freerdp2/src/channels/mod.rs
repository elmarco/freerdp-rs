use std::ffi::{CStr, CString};

use crate::{sys, FreeRdp, RdpError, Result};

pub mod audin;

pub mod cliprdr;

pub mod disp;

pub mod drdynvc;

pub mod echo;

pub mod encomsp;

pub mod geometry;

pub mod gfxredir;

pub mod rail;

pub mod rdp2tcp;

pub mod rdpdr;

pub mod rdpecam;

pub mod rdpei;

pub mod rdpgfx;

pub mod rdpsnd;

pub mod remdesk;

pub mod tsmf;

pub mod urbdrc;

pub mod video;

pub fn id_by_name(freerdp: &FreeRdp, name: &str) -> Result<u16> {
    let name = CString::new(name)?;
    let id = unsafe { sys::freerdp_channels_get_id_by_name(freerdp.inner.as_ptr(), name.as_ptr()) };
    Ok(id)
}

pub fn name_by_id(freerdp: &FreeRdp, id: u16) -> Result<String> {
    let name = unsafe { sys::freerdp_channels_get_name_by_id(freerdp.inner.as_ptr(), id) };
    if name.is_null() {
        return Err(RdpError::Failed("Unknown channel ID name".into()));
    }
    let name = unsafe { CStr::from_ptr(name) };
    Ok(name.to_string_lossy().to_string())
}
