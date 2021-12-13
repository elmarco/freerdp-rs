use std::ptr;

use crate::{channels::encomsp::*, sys, Result};

#[derive(Debug)]
pub struct EncomspClientContext {
    pub(crate) inner: ptr::NonNull<sys::EncomspClientContext>,
}

unsafe impl Send for EncomspClientContext {}
unsafe impl Sync for EncomspClientContext {}

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
    pub unsafe fn from_ptr(context: *mut sys::EncomspClientContext) -> Self {
        Self {
            inner: ptr::NonNull::new(context).unwrap(),
        }
    }

    pub fn register_handler<H: EncomspHandler>(&mut self, handler: H) {
        let inner = unsafe { self.inner.as_mut() };
        inner.ParticipantCreated = Some(rdp_participant_created::<H>);
        // FIXME: where is the dtor?
        inner.custom = Box::into_raw(Box::new(handler)) as *mut _;
    }
}

extern "C" fn rdp_participant_created<H: EncomspHandler>(
    context: *mut sys::EncomspClientContext,
    participant: *const sys::ENCOMSP_PARTICIPANT_CREATED_PDU,
) -> u32 {
    let mut ctxt = unsafe { EncomspClientContext::from_ptr(context) };
    let participant = unsafe { ParticipantCreated::from_ptr(participant as *mut _) };
    let self_ = unsafe { (ctxt.inner.as_mut().custom as *mut H).as_mut().unwrap() };
    if self_.participant_created(&mut ctxt, &participant).is_ok() {
        0
    } else {
        1
    }
}
