use std::{
    ffi::{CStr, CString},
    ptr,
};

use crate::{locale::KeyboardLayout, sys, ConnectionType, RdpError, Result};

pub struct Settings {
    pub(crate) inner: ptr::NonNull<sys::rdpSettings>,
    owned: bool,
}

unsafe impl Send for Settings {}
unsafe impl Sync for Settings {}

impl Clone for Settings {
    fn clone(&self) -> Self {
        Self::new(true, unsafe {
            sys::freerdp_settings_clone(self.inner.as_ptr())
        })
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            sys::freerdp_settings_copy(self.inner.as_ptr(), source.inner.as_ptr());
        }
    }
}

impl Drop for Settings {
    fn drop(&mut self) {
        if !self.owned {
            return;
        }
        unsafe { sys::freerdp_settings_free(self.inner.as_ptr()) }
    }
}

macro_rules! str_setting {
    ($set:ident, $get:ident, $sys:ident, $field:ident) => {
        pub fn $set(&mut self, val: Option<&str>) -> Result<()> {
            if unsafe {
                let val = match val {
                    Some(s) => Some(CString::new(s)?),
                    None => None,
                };
                sys::freerdp_settings_set_string(
                    self.inner.as_ptr(),
                    sys::$sys as _,
                    val.as_ref().map_or(ptr::null(), |s| s.as_ptr()),
                )
            } != 0
            {
                Ok(())
            } else {
                Err(RdpError::Failed("Failed to set setting".into()))
            }
        }

        pub fn $get(&self) -> Option<String> {
            unsafe {
                let ptr = self.inner.as_ref().$field;
                if ptr.is_null() {
                    None
                } else {
                    Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
                }
            }
        }
    };
}

impl Settings {
    pub(crate) fn new(owned: bool, settings: *mut sys::rdpSettings) -> Self {
        Self {
            inner: std::ptr::NonNull::new(settings).unwrap(),
            owned,
        }
    }

    pub fn as_ptr(&self) -> *mut sys::rdpSettings {
        self.inner.as_ptr()
    }

    pub fn parse_command_line(&mut self, args: &[&str], allow_unknown: bool) -> Result<()> {
        let cargs: Vec<_> = args.iter().map(|a| CString::new(*a).unwrap()).collect();
        let argv: Vec<_> = cargs.iter().map(|a| a.as_ptr()).collect();

        let res = unsafe {
            sys::freerdp_client_settings_parse_command_line(
                self.inner.as_ptr(),
                argv.len() as _,
                argv.as_ptr() as _,
                allow_unknown as _,
            )
        };
        if res == 0 {
            Ok(())
        } else {
            Err(RdpError::Unsupported)
        }
    }

    str_setting!(
        set_server_hostname,
        server_hostname,
        FreeRDP_ServerHostname,
        ServerHostname
    );

    pub fn set_server_port(&mut self, port: u32) {
        unsafe {
            self.inner.as_mut().ServerPort = port;
        }
    }

    pub fn server_port(&self) -> u32 {
        unsafe { self.inner.as_ref().ServerPort }
    }

    str_setting!(set_username, username, FreeRDP_Username, Username);
    str_setting!(set_password, password, FreeRDP_Password, Password);
    str_setting!(set_domain, domain, FreeRDP_Domain, Domain);

    str_setting!(
        set_gateway_username,
        gateway_username,
        FreeRDP_GatewayUsername,
        GatewayUsername
    );
    str_setting!(
        set_gateway_password,
        gateway_password,
        FreeRDP_GatewayPassword,
        GatewayPassword
    );
    str_setting!(
        set_gateway_domain,
        gateway_domain,
        FreeRDP_GatewayDomain,
        GatewayDomain
    );

    pub fn set_remote_fx_codec(&mut self, remotefx: bool) {
        unsafe {
            self.inner.as_mut().RemoteFxCodec = remotefx as _;
        }
    }

    pub fn remote_fx_codec(&self) -> bool {
        unsafe { self.inner.as_ref().RemoteFxCodec != 0 }
    }

    pub fn set_allow_font_smoothing(&mut self, allow: bool) {
        unsafe {
            self.inner.as_mut().AllowFontSmoothing = allow as _;
        }
    }

    pub fn allow_font_smoothing(&self) -> bool {
        unsafe { self.inner.as_ref().AllowFontSmoothing != 0 }
    }

    pub fn set_allow_unanounced_orders_from_server(&mut self, allow: bool) {
        unsafe {
            self.inner.as_mut().AllowUnanouncedOrdersFromServer = allow as _;
        }
    }

    pub fn allow_unanounced_orders_from_server(&self) -> bool {
        unsafe { self.inner.as_ref().AllowUnanouncedOrdersFromServer != 0 }
    }

    pub fn set_os_major_type(&mut self, type_: u32) {
        unsafe {
            self.inner.as_mut().OsMajorType = type_;
        }
    }

    pub fn os_major_type(&self) -> u32 {
        unsafe { self.inner.as_ref().OsMajorType }
    }

    pub fn set_os_minor_type(&mut self, type_: u32) {
        unsafe {
            self.inner.as_mut().OsMinorType = type_;
        }
    }

    pub fn os_minor_type(&self) -> u32 {
        unsafe { self.inner.as_ref().OsMinorType }
    }

    pub fn smart_sizing(&self) -> bool {
        unsafe { self.inner.as_ref().SmartSizing != 0 }
    }

    pub fn smart_sizing_width(&self) -> u32 {
        unsafe { self.inner.as_ref().SmartSizingWidth }
    }

    pub fn smart_sizing_height(&self) -> u32 {
        unsafe { self.inner.as_ref().SmartSizingHeight }
    }

    pub fn fullscreen(&self) -> bool {
        unsafe { self.inner.as_ref().Fullscreen != 0 }
    }

    pub fn rdp_version(&self) -> u32 {
        unsafe { self.inner.as_ref().RdpVersion }
    }

    pub fn desktop_width(&self) -> u32 {
        unsafe { self.inner.as_ref().DesktopWidth }
    }

    pub fn desktop_height(&self) -> u32 {
        unsafe { self.inner.as_ref().DesktopHeight }
    }

    pub fn color_depth(&self) -> u32 {
        unsafe { self.inner.as_ref().ColorDepth }
    }

    pub fn set_connection_type(&mut self, type_: ConnectionType) -> Result<()> {
        if unsafe { sys::freerdp_set_connection_type(self.inner.as_ptr(), type_.into()) } == 0 {
            Err(RdpError::Failed("Failed to set connection type".into()))
        } else {
            Ok(())
        }
    }

    pub fn keyboard_layout(&self) -> KeyboardLayout {
        KeyboardLayout(unsafe { self.inner.as_ref().KeyboardLayout })
    }

    pub fn keyboard_remapping_list(&self) -> Option<String> {
        let remapping = unsafe { self.inner.as_ref().KeyboardRemappingList };
        if remapping.is_null() {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(remapping) }
                    .to_string_lossy()
                    .to_string(),
            )
        }
    }

    pub fn gfx_h264(&self) -> bool {
        unsafe { self.inner.as_ref().GfxH264 != 0 }
    }

    pub fn set_support_display_control(&mut self, enabled: bool) {
        unsafe {
            self.inner.as_mut().SupportDisplayControl = enabled as _;
        }
    }

    pub fn support_display_control(&self) -> bool {
        unsafe { self.inner.as_ref().SupportDisplayControl != 0 }
    }
}

impl std::fmt::Debug for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Settings")
            .field("server_hostname", &self.server_hostname())
            .field("server_port", &self.server_port())
            .field("username", &self.username())
            .field("password", &self.password())
            .finish()
    }
}
