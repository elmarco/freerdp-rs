use std::ffi::c_void;

pub(super) struct Custom {
    pub(super) handler: *mut c_void,
    free: fn(*mut c_void),
}

impl Drop for Custom {
    fn drop(&mut self) {
        (self.free)(self.handler)
    }
}

impl Custom {
    #[allow(clippy::new_ret_no_self)]
    pub(super) fn new<H>(handler: H) -> *mut c_void {
        Box::into_raw(Box::new(Custom {
            handler: Box::into_raw(Box::new(handler)) as *mut _,
            free: |raw| unsafe { drop(Box::from_raw(raw as *mut H)) },
        })) as *mut _
    }
}
