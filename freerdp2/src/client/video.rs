use std::ptr;

use crate::sys;

pub struct GeometryClientContext {
    pub(crate) inner: ptr::NonNull<sys::GeometryClientContext>,
}

impl GeometryClientContext {
    /// # Safety
    ///
    /// * The memory pointed to by `ctxt` must contain a valid pointer.
    /// * `ctxt` must be [valid] for both reads and writes for the whole lifetime `'a` FIXME.
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
    /// # Safety
    ///
    /// * The memory pointed to by `ctxt` must contain a valid pointer.
    /// * `ctxt` must be [valid] for both reads and writes for the whole lifetime `'a` FIXME.
    pub unsafe fn from_ptr(ctxt: *mut sys::VideoClientContext) -> Self {
        Self {
            inner: ptr::NonNull::new(ctxt).unwrap(),
        }
    }
}
