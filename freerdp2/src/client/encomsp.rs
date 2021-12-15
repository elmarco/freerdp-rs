use std::ptr;

use crate::{channels::encomsp::*, client::custom::Custom, sys, Result};

#[derive(Debug)]
pub struct EncomspClientContext {
    pub(crate) inner: ptr::NonNull<sys::EncomspClientContext>,
    owned: bool,
}

unsafe impl Send for EncomspClientContext {}
unsafe impl Sync for EncomspClientContext {}

impl Drop for EncomspClientContext {
    fn drop(&mut self) {
        if !self.owned {
            return;
        }
        unsafe {
            let inner = self.inner.as_mut();
            drop(Box::from_raw(inner.custom as *mut Custom));
        }
    }
}

pub trait EncomspHandler {
    fn participant_created(
        &mut self,
        _ctxt: &mut EncomspClientContext,
        _participant: &ParticipantCreated,
    ) -> Result<()> {
        Ok(())
    }
}

impl EncomspClientContext {
    pub unsafe fn from_ptr(context: *mut sys::EncomspClientContext, owned: bool) -> Self {
        Self {
            inner: ptr::NonNull::new(context).unwrap(),
            owned,
        }
    }

    pub fn register_handler<H: EncomspHandler>(&mut self, handler: H) {
        let inner = unsafe { self.inner.as_mut() };
        assert!(inner.custom.is_null());
        inner.ParticipantCreated = Some(rdp_participant_created::<H>);
        inner.custom = Custom::new(handler);
    }

    // should be safe as long as inner.custom is set only once
    unsafe fn handler<'a, 'b, H: EncomspHandler>(&'a mut self) -> &'b mut H {
        let custom = (self.inner.as_mut().custom as *mut Custom)
            .as_mut()
            .unwrap();
        (custom.handler as *mut H).as_mut().unwrap()
    }
}

extern "C" fn rdp_participant_created<H: EncomspHandler>(
    context: *mut sys::EncomspClientContext,
    participant: *const sys::ENCOMSP_PARTICIPANT_CREATED_PDU,
) -> u32 {
    let mut ctxt = unsafe { EncomspClientContext::from_ptr(context, false) };
    let participant = unsafe { ParticipantCreated::from_ptr(participant as *mut _) };
    let handler = unsafe { ctxt.handler::<H>() };
    if handler.participant_created(&mut ctxt, &participant).is_ok() {
        0
    } else {
        1
    }
}
