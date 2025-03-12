use bitflags::bitflags;
use std::{io, time::Duration};

#[cfg(unix)]
use std::os::unix::prelude::{IntoRawFd, RawFd};

use crate::{sys, RdpError, Result};

#[derive(Clone, Debug)]
pub struct Handle {
    handle: sys::HANDLE,
    owned: bool,
}

unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

impl Default for Handle {
    fn default() -> Self {
        Self {
            handle: std::ptr::null_mut(),
            owned: false,
        }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            if self.owned {
                sys::CloseHandle(self.handle);
            }
        }
    }
}

pub const MAX_WAIT_OBJECTS: usize = 64;
pub const INFINITE: u32 = u32::MAX;

#[derive(Debug)]
pub enum WaitResult {
    Object(u32),
    Abandoned(u32),
    Timeout,
}

pub fn wait_for_multiple_objects(
    handles: &[&Handle],
    wait_all: bool,
    timeout: Option<&Duration>,
) -> Result<WaitResult> {
    let len = handles.len() as _;
    let handles: Vec<sys::HANDLE> = handles.iter().map(|h| h.handle).collect();
    let res = unsafe {
        sys::WaitForMultipleObjects(
            len,
            handles.as_ptr() as _,
            wait_all as _,
            timeout.map_or(INFINITE, |t| t.as_millis() as _),
        )
    };
    match res {
        res if res < (sys::WAIT_OBJECT_0 + len) => Ok(WaitResult::Object(res - sys::WAIT_OBJECT_0)),
        res if res >= sys::WAIT_ABANDONED && res < (sys::WAIT_ABANDONED + len) => {
            Ok(WaitResult::Abandoned(res - sys::WAIT_ABANDONED))
        }
        sys::WAIT_TIMEOUT => Ok(WaitResult::Timeout),
        u32::MAX => Err(RdpError::IOError(
            io::Error::from_raw_os_error(last_error()),
        )),
        _ => Err(RdpError::Failed(format!(
            "Unhandled WaitForMultipleObjects() return: {:x}",
            res
        ))),
    }
}

pub fn wait_for_single_object(handle: &Handle, timeout: Option<&Duration>) -> Result<WaitResult> {
    let res = unsafe {
        sys::WaitForSingleObject(
            handle.handle,
            timeout.map_or(INFINITE, |t| t.as_millis() as _),
        )
    };
    match res {
        sys::WAIT_OBJECT_0 => Ok(WaitResult::Object(sys::WAIT_OBJECT_0)),
        sys::WAIT_ABANDONED => Ok(WaitResult::Abandoned(0)),
        sys::WAIT_TIMEOUT => Ok(WaitResult::Timeout),
        u32::MAX => Err(RdpError::IOError(
            io::Error::from_raw_os_error(last_error()),
        )),
        _ => Err(RdpError::Failed(format!(
            "Unhandled WaitForMultipleObjects() return: {:x}",
            res
        ))),
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FdMode: u32 {
        const READ = 0b00000001;
        const WRITE = 0b00000010;
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct SecurityAttributes(sys::_SECURITY_ATTRIBUTES);

impl Handle {
    pub(crate) fn new(handle: sys::HANDLE, owned: bool) -> Self {
        Self { handle, owned }
    }

    #[cfg(windows)]
    pub fn new_from_raw(handle: std::os::windows::raw::HANDLE, owned: bool) -> Self {
        Self { handle, owned }
    }

    #[cfg(unix)]
    pub fn new_fd_event(
        event_attributes: &[SecurityAttributes],
        manual_reset: bool,
        initial_state: bool,
        fd: RawFd,
        mode: FdMode,
    ) -> Self {
        let file_descriptor = fd.into_raw_fd();
        if !event_attributes.is_empty() {
            unimplemented!();
        }
        let event_attributes = std::ptr::null_mut();
        Self::new(
            unsafe {
                sys::CreateFileDescriptorEventA(
                    event_attributes,
                    manual_reset as _,
                    initial_state as _,
                    file_descriptor,
                    mode.bits(),
                )
            },
            true,
        )
    }

    pub fn wait(&self, timeout: Option<&Duration>) -> Result<WaitResult> {
        wait_for_single_object(self, timeout)
    }
}

fn last_error() -> i32 {
    unsafe { sys::GetLastError() as _ }
}
