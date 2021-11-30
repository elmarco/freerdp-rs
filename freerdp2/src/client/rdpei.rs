use std::ptr;

use crate::{sys, RdpError, Result};

#[derive(Debug)]
pub struct RdpeiClientContext {
    pub(crate) inner: ptr::NonNull<sys::RdpeiClientContext>,
}

unsafe impl Send for RdpeiClientContext {}
unsafe impl Sync for RdpeiClientContext {}

impl RdpeiClientContext {
    pub unsafe fn from_ptr(ctxt: *mut sys::RdpeiClientContext) -> Self {
        Self {
            inner: ptr::NonNull::new(ctxt).unwrap(),
        }
    }

    pub fn version(&self) -> u32 {
        unsafe {
            let f = self.inner.as_ref().GetVersion.unwrap();
            f(self.inner.as_ptr())
        }
    }

    pub fn features(&self) -> u32 {
        unsafe {
            let f = self.inner.as_ref().GetFeatures.unwrap();
            f(self.inner.as_ptr())
        }
    }

    // FIXME: make &mut for Sync ?!
    pub fn add_contact(&self, contact: &Contact) -> Result<()> {
        let res = unsafe {
            let f = self.inner.as_ref().AddContact.unwrap();
            f(self.inner.as_ptr(), &contact.inner)
        };
        if res == 0 {
            Ok(())
        } else {
            Err(RdpError::IOError(std::io::Error::from_raw_os_error(
                res as _,
            )))
        }
    }

    pub fn touch_begin(&self, external_id: i32, x: i32, y: i32) -> Result<i32> {
        let mut id = 0;
        let res = unsafe {
            let f = self.inner.as_ref().TouchBegin.unwrap();
            f(self.inner.as_ptr(), external_id, x, y, &mut id)
        };
        if res == 0 {
            Ok(id)
        } else {
            Err(RdpError::IOError(std::io::Error::from_raw_os_error(
                res as _,
            )))
        }
    }

    pub fn touch_update(&self, external_id: i32, x: i32, y: i32) -> Result<i32> {
        let mut id = 0;
        let res = unsafe {
            let f = self.inner.as_ref().TouchUpdate.unwrap();
            f(self.inner.as_ptr(), external_id, x, y, &mut id)
        };
        if res == 0 {
            Ok(id)
        } else {
            Err(RdpError::IOError(std::io::Error::from_raw_os_error(
                res as _,
            )))
        }
    }

    pub fn touch_end(&self, external_id: i32, x: i32, y: i32) -> Result<i32> {
        let mut id = 0;
        let res = unsafe {
            let f = self.inner.as_ref().TouchEnd.unwrap();
            f(self.inner.as_ptr(), external_id, x, y, &mut id)
        };
        if res == 0 {
            Ok(id)
        } else {
            Err(RdpError::IOError(std::io::Error::from_raw_os_error(
                res as _,
            )))
        }
    }

    pub fn suspend_touch(&self) -> Result<()> {
        let res = unsafe {
            let f = self.inner.as_ref().SuspendTouch.unwrap();
            f(self.inner.as_ptr())
        };
        if res == 0 {
            Ok(())
        } else {
            Err(RdpError::IOError(std::io::Error::from_raw_os_error(
                res as _,
            )))
        }
    }

    pub fn resume_touch(&self) -> Result<()> {
        let res = unsafe {
            let f = self.inner.as_ref().ResumeTouch.unwrap();
            f(self.inner.as_ptr())
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

pub struct Contact {
    inner: sys::RDPINPUT_CONTACT_DATA,
}