use std::{ptr, string::FromUtf16Error};

use bitflags::bitflags;

use crate::sys;

pub const SVC_CHANNEL_NAME: &str = "encomsp";

bitflags! {
    pub struct ParticipantCreatedFlags: u16 {
        const MAY_VIEW = 0b00000001;
        const MAY_INTERACT = 0b00000010;
        const IS_PARTICIPANT = 0b00000100;
    }
}

#[derive(Debug)]
pub struct ParticipantCreated {
    pub(crate) inner: ptr::NonNull<sys::ENCOMSP_PARTICIPANT_CREATED_PDU>,
}

impl ParticipantCreated {
    /// # Safety
    ///
    /// * The memory pointed to by `ptr` must contain a valid pointer.
    /// * `ptr` must be [valid] for both reads and writes for the whole lifetime `'a` FIXME.
    pub unsafe fn from_ptr(ptr: *mut sys::ENCOMSP_PARTICIPANT_CREATED_PDU) -> Self {
        Self {
            inner: ptr::NonNull::new(ptr).unwrap(),
        }
    }

    pub fn participant_id(&self) -> u32 {
        unsafe { self.inner.as_ref() }.ParticipantId
    }

    pub fn group_id(&self) -> u32 {
        unsafe { self.inner.as_ref() }.GroupId
    }

    pub fn flags(&self) -> ParticipantCreatedFlags {
        let flags = unsafe { self.inner.as_ref() }.Flags;
        ParticipantCreatedFlags::from_bits_truncate(flags)
    }

    pub fn friendly_name(&self) -> Result<String, FromUtf16Error> {
        let name = unsafe { self.inner.as_ref() }.FriendlyName;
        let len = name.cchString as usize;
        String::from_utf16(&name.wString[0..len])
    }
}
