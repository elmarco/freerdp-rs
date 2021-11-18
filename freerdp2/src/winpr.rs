use std::{io, time::Duration};

use crate::{sys, RdpError, Result};

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Handle(sys::HANDLE);

impl Default for Handle {
    fn default() -> Self {
        Self(std::ptr::null_mut())
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
    handles: &[Handle],
    wait_all: bool,
    timeout: Option<&Duration>,
) -> Result<WaitResult> {
    let len = handles.len() as _;
    let res = unsafe {
        sys::WaitForMultipleObjects(
            len,
            handles.as_ptr() as _,
            wait_all as _,
            timeout.map_or(INFINITE, |t| t.as_millis() as _),
        )
    };
    match res {
        res if res >= sys::WAIT_OBJECT_0 && res < (sys::WAIT_OBJECT_0 + len) => {
            Ok(WaitResult::Object(res - sys::WAIT_OBJECT_0 + 1))
        }
        res if res >= sys::WAIT_ABANDONED && res < (sys::WAIT_ABANDONED + len) => {
            Ok(WaitResult::Abandoned(res - sys::WAIT_ABANDONED + 1))
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

fn last_error() -> i32 {
    unsafe { sys::GetLastError() as _ }
}
