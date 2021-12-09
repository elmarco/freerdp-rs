use bitflags::bitflags;
use std::{marker::PhantomData, ptr};

use crate::{sys, RdpError, Result};

bitflags! {
    pub struct SyncFlags: u32 {
        const SCROLL = 0b00000001;
        const NUM = 0b00000010;
        const CAPS = 0b00000100;
        const KANA = 0b00001000;
    }
}

bitflags! {
    pub struct KbdFlags: u16 {
        const EXTENDED = 0x0100;
        const EXTENDED1 = 0x0200;
        const DOWN = 0x4000;
        const RELEASE = 0x8000;
    }
}

bitflags! {
    pub struct PtrFlags: u16 {
        const HWHEEL = 0x0400;
        const WHEEL = 0x0200;
        const WHEEL_NEGATIVE = 0x0100;
        const MOVE = 0x0800;
        const DOWN = 0x8000;
        const BUTTON1 = 0x1000; // left
        const BUTTON2 = 0x2000; // right
        const BUTTON3 = 0x4000; // middle
    }
}

pub const WHEEL_ROTATION_MASK: u16 = 0x01FF;

bitflags! {
    pub struct PtrXFlags: u16 {
        const DOWN = 0x8000;
        const BUTTON1 = 0x0001;
        const BUTTON2 = 0x0002;
    }
}

#[derive(Debug)]
pub struct Input<'a> {
    inner: ptr::NonNull<sys::rdpInput>,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> Input<'a> {
    pub(crate) fn new(input: *mut sys::rdpInput) -> Self {
        Self {
            inner: ptr::NonNull::new(input).unwrap(),
            _lifetime: PhantomData,
        }
    }

    pub fn send_synchronize_event(&mut self, flags: SyncFlags) -> Result<()> {
        if unsafe { sys::freerdp_input_send_synchronize_event(self.inner.as_ptr(), flags.bits()) }
            != 0
        {
            Ok(())
        } else {
            Err(RdpError::Failed("send_synchronize_event() failed".into()))
        }
    }

    pub fn send_keyboard_event(&mut self, flags: KbdFlags, code: u16) -> Result<()> {
        if unsafe {
            sys::freerdp_input_send_keyboard_event(self.inner.as_ptr(), flags.bits(), code)
        } != 0
        {
            Ok(())
        } else {
            Err(RdpError::Failed("send_keyboard_event() failed".into()))
        }
    }

    pub fn send_keyboard_pause_event(&mut self) -> Result<()> {
        if unsafe { sys::freerdp_input_send_keyboard_pause_event(self.inner.as_ptr()) } != 0 {
            Ok(())
        } else {
            Err(RdpError::Failed(
                "send_keyboard_pause_event() failed".into(),
            ))
        }
    }

    pub fn send_unicode_keyboard_event(&mut self, flags: KbdFlags, code: u16) -> Result<()> {
        if unsafe {
            sys::freerdp_input_send_unicode_keyboard_event(self.inner.as_ptr(), flags.bits(), code)
        } != 0
        {
            Ok(())
        } else {
            Err(RdpError::Failed(
                "send_unicode_keyboard_event() failed".into(),
            ))
        }
    }

    pub fn send_mouse_event(&mut self, flags: PtrFlags, x: u16, y: u16) -> Result<()> {
        if unsafe { sys::freerdp_input_send_mouse_event(self.inner.as_ptr(), flags.bits(), x, y) }
            != 0
        {
            Ok(())
        } else {
            Err(RdpError::Failed("send_mouse_event() failed".into()))
        }
    }

    pub fn send_extended_mouse_event(&mut self, flags: PtrXFlags, x: u16, y: u16) -> Result<()> {
        if unsafe {
            sys::freerdp_input_send_extended_mouse_event(self.inner.as_ptr(), flags.bits(), x, y)
        } != 0
        {
            Ok(())
        } else {
            Err(RdpError::Failed(
                "send_extended_mouse_event() failed".into(),
            ))
        }
    }

    pub fn send_focus_in_event(&mut self, flags: SyncFlags) -> Result<()> {
        if unsafe { sys::freerdp_input_send_focus_in_event(self.inner.as_ptr(), flags.bits() as _) }
            != 0
        {
            Ok(())
        } else {
            Err(RdpError::Failed("send_focus_in_event() failed".into()))
        }
    }
}
