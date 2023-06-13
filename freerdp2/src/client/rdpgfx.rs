use std::ptr;

use crate::sys;

pub struct RdpgfxClientContext {
    pub(crate) inner: ptr::NonNull<sys::RdpgfxClientContext>,
}

impl RdpgfxClientContext {
    /// # Safety
    ///
    /// * The memory pointed to by `ctxt` must contain a valid pointer.
    /// * `ctxt` must be [valid] for both reads and writes for the whole lifetime `'a` FIXME.
    pub unsafe fn from_ptr(ctxt: *mut sys::RdpgfxClientContext) -> Self {
        Self {
            inner: ptr::NonNull::new(ctxt).unwrap(),
        }
    }
}
