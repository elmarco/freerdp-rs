use std::ptr;

use crate::sys;

pub struct GeometryClientContext {
    pub(crate) inner: ptr::NonNull<sys::GeometryClientContext>,
}

impl GeometryClientContext {
    pub unsafe fn from_ptr(ctxt: *mut sys::GeometryClientContext) -> Self {
        Self {
            inner: ptr::NonNull::new(ctxt).unwrap(),
        }
    }
}

pub struct VideoClientContext {
    pub(crate) inner: ptr::NonNull<sys::VideoClientContext>,
}

impl VideoClientContext {
    pub unsafe fn from_ptr(ctxt: *mut sys::VideoClientContext) -> Self {
        Self {
            inner: ptr::NonNull::new(ctxt).unwrap(),
        }
    }
}
