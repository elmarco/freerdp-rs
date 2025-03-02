use std::{
    fmt::Debug,
    marker::PhantomData,
    mem::{self, size_of},
    ptr, slice,
};

use crate::{
    client::{Context, Handler},
    gdi::GdiPalette,
    sys, PixelFormat, RdpError, Result,
};

struct RdpPointer<H: PointerHandler> {
    _pointer: sys::rdpPointer,
    handler: H,
}

pub struct Pointer<'a> {
    inner: ptr::NonNull<sys::rdpPointer>,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> Debug for Pointer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pointer")
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}

impl<'a> Pointer<'a> {
    pub(crate) fn new(pointer: *mut sys::rdpPointer) -> Self {
        Self {
            inner: ptr::NonNull::new(pointer).unwrap(),
            _lifetime: PhantomData,
        }
    }

    pub fn x(&self) -> u32 {
        unsafe { self.inner.as_ref() }.xPos
    }

    pub fn y(&self) -> u32 {
        unsafe { self.inner.as_ref() }.yPos
    }

    pub fn width(&self) -> u32 {
        unsafe { self.inner.as_ref() }.width
    }

    pub fn height(&self) -> u32 {
        unsafe { self.inner.as_ref() }.height
    }

    pub fn xor_bpp(&self) -> u32 {
        unsafe { self.inner.as_ref() }.xorBpp
    }

    pub fn xor_mask(&self) -> Option<&[u8]> {
        let p = unsafe { self.inner.as_ref() };
        if p.xorMaskData.is_null() {
            None
        } else {
            Some(unsafe { slice::from_raw_parts(p.xorMaskData, p.lengthXorMask as _) })
        }
    }

    pub fn and_mask(&self) -> Option<&[u8]> {
        let p = unsafe { self.inner.as_ref() };
        if p.andMaskData.is_null() {
            None
        } else {
            Some(unsafe { slice::from_raw_parts(p.andMaskData, p.lengthAndMask as _) })
        }
    }

    pub fn data(&self, palette: &GdiPalette, format: &PixelFormat) -> Result<Vec<u8>> {
        let len = self.height() * self.width() * 4;
        let p = unsafe { self.inner.as_ref() };
        let mut data = Vec::with_capacity(len as _);
        let res = unsafe {
            sys::freerdp_image_copy_from_pointer_data(
                data.as_mut_ptr(),
                format.into(),
                self.width() * 4,
                0,
                0,
                self.width(),
                self.height(),
                p.xorMaskData,
                p.lengthXorMask,
                p.andMaskData,
                p.lengthAndMask,
                self.xor_bpp(),
                palette.inner.as_ptr(),
            )
        };
        if res == 0 {
            Err(RdpError::Failed(
                "freerdp_image_copy_from_pointer_data() failed".into(),
            ))
        } else {
            unsafe { data.set_len(len as _) };
            Ok(data)
        }
    }
}

pub trait PointerHandler {
    type ContextHandler: Handler;

    #[allow(clippy::wrong_self_convention)]
    #[allow(clippy::new_ret_no_self)]
    fn new(
        &mut self,
        _context: &mut Context<Self::ContextHandler>,
        _pointer: &Pointer,
    ) -> Result<()> {
        Ok(())
    }

    fn free(&mut self, _context: &mut Context<Self::ContextHandler>, _pointer: &Pointer) {}

    fn set(
        &mut self,
        _context: &mut Context<Self::ContextHandler>,
        _pointer: &Pointer,
    ) -> Result<()> {
        Ok(())
    }

    fn set_null(_context: &mut Context<Self::ContextHandler>) -> Result<()> {
        Ok(())
    }

    fn set_default(_context: &mut Context<Self::ContextHandler>) -> Result<()> {
        Ok(())
    }

    fn set_position(_context: &mut Context<Self::ContextHandler>, _x: u32, _y: u32) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Graphics<'a> {
    inner: ptr::NonNull<sys::rdpGraphics>,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> Graphics<'a> {
    pub(crate) fn new(graphics: *mut sys::rdpGraphics) -> Self {
        Self {
            inner: ptr::NonNull::new(graphics).unwrap(),
            _lifetime: PhantomData,
        }
    }

    pub fn register_pointer<H: PointerHandler>(&mut self) {
        let mut ptr: sys::rdpPointer = unsafe { mem::zeroed() };
        ptr.New = Some(rdp_pointer_new::<H>);
        ptr.Free = Some(rdp_pointer_free::<H>);
        ptr.Set = Some(rdp_pointer_set::<H>);
        ptr.SetNull = Some(rdp_pointer_set_null::<H>);
        ptr.SetDefault = Some(rdp_pointer_set_default::<H>);
        ptr.SetPosition = Some(rdp_pointer_set_position::<H>);
        ptr.size = size_of::<RdpPointer<H>>() as _;

        unsafe { sys::graphics_register_pointer(self.inner.as_ptr(), &mut ptr) }
    }
}

extern "C" fn rdp_pointer_new<H: PointerHandler>(
    context: *mut sys::rdpContext,
    pointer: *mut sys::rdpPointer,
) -> sys::BOOL {
    let context = Context::from_ptr(context);
    let mut inner = ptr::NonNull::new(pointer as *mut _ as *mut RdpPointer<H>).unwrap();

    unsafe { inner.as_mut() }
        .handler
        .new(context, &Pointer::new(pointer))
        .is_ok() as _
}

extern "C" fn rdp_pointer_free<H: PointerHandler>(
    context: *mut sys::rdpContext,
    pointer: *mut sys::rdpPointer,
) {
    let context = Context::from_ptr(context);
    let mut inner = ptr::NonNull::new(pointer as *mut _ as *mut RdpPointer<H>).unwrap();

    unsafe { inner.as_mut() }
        .handler
        .free(context, &Pointer::new(pointer));
}

extern "C" fn rdp_pointer_set<H: PointerHandler>(
    context: *mut sys::rdpContext,
    pointer: *const sys::rdpPointer,
) -> i32 {
    let context = Context::from_ptr(context);
    let mut inner = ptr::NonNull::new(pointer as *mut RdpPointer<H>).unwrap();

    unsafe { inner.as_mut() }
        .handler
        .set(context, &Pointer::new(pointer as *mut _))
        .is_ok() as _
}

extern "C" fn rdp_pointer_set_null<H: PointerHandler>(context: *mut sys::rdpContext) -> i32 {
    let context = Context::from_ptr(context);

    H::set_null(context).is_ok() as _
}

extern "C" fn rdp_pointer_set_default<H: PointerHandler>(context: *mut sys::rdpContext) -> i32 {
    let context = Context::from_ptr(context);

    H::set_default(context).is_ok() as _
}

extern "C" fn rdp_pointer_set_position<H: PointerHandler>(
    context: *mut sys::rdpContext,
    x: u32,
    y: u32,
) -> i32 {
    let context = Context::from_ptr(context);

    H::set_position(context, x, y).is_ok() as _
}
