use std::{fmt::Debug, marker::PhantomData, ptr};

use crate::{
    client::{Context, Handler},
    sys, Result,
};

pub struct Update<'a> {
    inner: ptr::NonNull<sys::rdpUpdate>,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> Debug for Update<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Update")
            // .field("y", &self.y())
            .finish()
    }
}

impl<'a> Update<'a> {
    pub(crate) fn new(update: *mut sys::rdpUpdate) -> Self {
        Self {
            inner: ptr::NonNull::new(update).unwrap(),
            _lifetime: PhantomData,
        }
    }

    pub fn register<H: UpdateHandler>(&mut self) {
        let inner = unsafe { self.inner.as_mut() };

        inner.BeginPaint = Some(rdp_update_begin_paint::<H>);
        inner.EndPaint = Some(rdp_update_end_paint::<H>);
        inner.SetBounds = Some(rdp_update_set_bounds::<H>);
        inner.Synchronize = Some(rdp_update_synchronize::<H>);
        inner.DesktopResize = Some(rdp_update_desktop_resize::<H>);
    }
}

pub trait UpdateHandler {
    type ContextHandler: Handler;

    fn begin_paint(_context: &mut Context<Self::ContextHandler>) -> Result<()> {
        Ok(())
    }

    fn end_paint(_context: &mut Context<Self::ContextHandler>) -> Result<()> {
        Ok(())
    }

    fn set_bounds(_context: &mut Context<Self::ContextHandler>, _bounds: &Bounds) -> Result<()> {
        Ok(())
    }

    fn synchronize(_context: &mut Context<Self::ContextHandler>) -> Result<()> {
        Ok(())
    }

    fn desktop_resize(_context: &mut Context<Self::ContextHandler>) -> Result<()> {
        Ok(())
    }
}

extern "C" fn rdp_update_begin_paint<H: UpdateHandler>(context: *mut sys::rdpContext) -> sys::BOOL {
    let context = Context::from_ptr(context);

    H::begin_paint(context).is_ok() as _
}

extern "C" fn rdp_update_end_paint<H: UpdateHandler>(context: *mut sys::rdpContext) -> sys::BOOL {
    let context = Context::from_ptr(context);

    H::end_paint(context).is_ok() as _
}

extern "C" fn rdp_update_set_bounds<H: UpdateHandler>(
    context: *mut sys::rdpContext,
    bounds: *const sys::rdpBounds,
) -> sys::BOOL {
    let context = Context::from_ptr(context);
    let bounds = unsafe { bounds.as_ref() }.unwrap();

    H::set_bounds(context, &bounds.into()).is_ok() as _
}

extern "C" fn rdp_update_synchronize<H: UpdateHandler>(context: *mut sys::rdpContext) -> sys::BOOL {
    let context = Context::from_ptr(context);

    H::synchronize(context).is_ok() as _
}

extern "C" fn rdp_update_desktop_resize<H: UpdateHandler>(
    context: *mut sys::rdpContext,
) -> sys::BOOL {
    let context = Context::from_ptr(context);

    H::desktop_resize(context).is_ok() as _
}

#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl From<&sys::rdpBounds> for Bounds {
    fn from(bounds: &sys::rdpBounds) -> Self {
        Self {
            left: bounds.left,
            top: bounds.top,
            right: bounds.right,
            bottom: bounds.bottom,
        }
    }
}
