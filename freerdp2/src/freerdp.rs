use crate::{sys, PixelFormat, RdpError, Result};

#[derive(Debug)]
pub struct FreeRdp {
    pub(crate) inner: std::ptr::NonNull<sys::freerdp>,
}

unsafe impl Send for FreeRdp {}
unsafe impl Sync for FreeRdp {}

impl FreeRdp {
    pub(crate) fn new(instance: *mut sys::freerdp) -> Self {
        Self {
            inner: std::ptr::NonNull::new(instance).unwrap(),
        }
    }

    pub fn connect(&mut self) -> Result<()> {
        let success = unsafe { sys::freerdp_connect(self.inner.as_ptr()) };
        if success > 0 {
            Ok(())
        } else {
            Err(RdpError::Failed("Failed to connect".into()))
        }
    }

    pub fn abort_connect(&mut self) -> Result<()> {
        let success = unsafe { sys::freerdp_abort_connect(self.inner.as_ptr()) };
        if success > 0 {
            Ok(())
        } else {
            Err(RdpError::Failed("Failed to abort connect".into()))
        }
    }

    pub fn shall_disconnect(&mut self) -> bool {
        unsafe { sys::freerdp_shall_disconnect(self.inner.as_ptr()) > 0 }
    }

    pub fn disconnect(&mut self) -> Result<()> {
        let success = unsafe { sys::freerdp_disconnect(self.inner.as_ptr()) };
        if success > 0 {
            Ok(())
        } else {
            Err(RdpError::Failed("Failed to disconnect".into()))
        }
    }

    pub fn reconnect(&mut self) -> Result<()> {
        let success = unsafe { sys::freerdp_reconnect(self.inner.as_ptr()) };
        if success > 0 {
            Ok(())
        } else {
            Err(RdpError::Failed("Failed to reconnect".into()))
        }
    }

    pub fn gdi_init(&mut self, format: &PixelFormat) -> Result<()> {
        if unsafe { sys::gdi_init(self.inner.as_ptr(), format.into()) > 0 } {
            Ok(())
        } else {
            Err(RdpError::Failed("gdi_init() failed".into()))
        }
    }

    pub fn gdi_uninit(&mut self) {
        unsafe { sys::gdi_free(self.inner.as_ptr()) }
    }
}
