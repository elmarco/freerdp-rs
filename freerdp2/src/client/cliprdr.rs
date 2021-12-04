use std::{ptr, mem};

use crate::{sys, RdpError, Result, channels::cliprdr::GeneralCapabilities};

#[derive(Debug)]
pub struct CliprdrClientContext {
    pub(crate) inner: ptr::NonNull<sys::CliprdrClientContext>,
}

unsafe impl Send for CliprdrClientContext {}
unsafe impl Sync for CliprdrClientContext {}

impl CliprdrClientContext {
    pub unsafe fn from_ptr(ctxt: *mut sys::CliprdrClientContext) -> Self {
        Self {
            inner: ptr::NonNull::new(ctxt).unwrap(),
        }
    }

    pub fn send_client_general_capabilities(&mut self, capabilities: &GeneralCapabilities) -> Result<()> {
        let mut cap: sys::CLIPRDR_CAPABILITIES =  unsafe { mem::zeroed() };
        let mut general_cap: sys::CLIPRDR_GENERAL_CAPABILITY_SET =  unsafe { mem::zeroed() };

        cap.cCapabilitiesSets = 1;
        cap.capabilitySets = &mut general_cap as *mut _ as *mut _;
        general_cap.capabilitySetType = sys::CB_CAPSTYPE_GENERAL as _;
        general_cap.capabilitySetLength = sys::CB_CAPSTYPE_GENERAL_LEN as _;
        general_cap.version = sys::CB_CAPS_VERSION_2;
        general_cap.generalFlags = capabilities.bits();

        let res = unsafe {
            let f = self.inner.as_ref().ClientCapabilities.unwrap();
            f(
                self.inner.as_ptr(),
                &cap,
            )
        };

        if res == 0 {
            Ok(())
        } else {
            Err(RdpError::IOError(std::io::Error::from_raw_os_error(
                res as _,
            )))
        }
    }
}
