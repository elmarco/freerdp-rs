use std::ptr;

use crate::{channels::disp::MonitorLayout, sys, RdpError, Result};

#[derive(Debug)]
pub struct DispClientContext {
    pub(crate) inner: ptr::NonNull<sys::DispClientContext>,
}

unsafe impl Send for DispClientContext {}
unsafe impl Sync for DispClientContext {}

impl DispClientContext {
    /// # Safety
    ///
    /// * The memory pointed to by `ctxt` must contain a valid pointer.
    /// * `ctxt` must be [valid] for both reads and writes for the whole lifetime `'a` FIXME.
    pub unsafe fn from_ptr(ctxt: *mut sys::DispClientContext) -> Self {
        Self {
            inner: ptr::NonNull::new(ctxt).unwrap(),
        }
    }

    // register_display_control_caps()?

    pub fn send_monitor_layout(&mut self, monitors: &[MonitorLayout]) -> Result<()> {
        let res = unsafe {
            let f = self.inner.as_ref().SendMonitorLayout.unwrap();
            f(
                self.inner.as_ptr(),
                monitors.len() as _,
                monitors.as_ptr() as _,
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
