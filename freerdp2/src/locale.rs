use std::{
    ffi::{CStr, CString},
    str::FromStr,
};

use crate::{sys, RdpError, Result};

#[derive(Debug, Copy, Clone)]
pub struct KeyboardLayout(pub(crate) u32);

impl FromStr for KeyboardLayout {
    type Err = RdpError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let cstr = CString::new(s)?;
        let res = unsafe { sys::freerdp_keyboard_get_layout_id_from_name(cstr.as_ptr()) };
        Ok(KeyboardLayout(res))
    }
}

impl KeyboardLayout {
    pub fn name(&self) -> String {
        let name = unsafe { CStr::from_ptr(sys::freerdp_keyboard_get_layout_name_from_id(self.0)) };
        name.to_string_lossy().to_string()
    }
}

pub fn keyboard_init_ex(layout: KeyboardLayout, remapping: Option<&str>) -> Result<KeyboardLayout> {
    let cstr = match remapping {
        Some(s) => Some(CString::new(s)?),
        None => None,
    };
    let remapping = cstr.as_ref().map_or(std::ptr::null(), |s| s.as_ptr());
    let res = unsafe { sys::freerdp_keyboard_init_ex(layout.0, remapping) };
    Ok(KeyboardLayout(res))
}
