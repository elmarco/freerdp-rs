use std::ptr;

use crate::sys;

pub struct RdpgfxClientContext {
    pub(crate) inner: ptr::NonNull<sys::RdpgfxClientContext>,
}

impl RdpgfxClientContext {
    pub unsafe fn from_ptr(ctxt: *mut sys::RdpgfxClientContext) -> Self {
        Self {
            inner: ptr::NonNull::new(ctxt).unwrap(),
        }
    }
}
