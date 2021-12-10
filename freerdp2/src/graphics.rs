use std::{
    fmt::Debug,
    marker::PhantomData,
    mem::{self, size_of},
    ptr, slice,
};

use crate::{
    client::{Context, Handler, RdpContext},
    gdi::GdiPalette,
    sys, RdpError, Result, PIXEL_FORMAT_BGRA32,
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

    pub fn xor_mask(&self) -> &[u8] {
        let p = unsafe { self.inner.as_ref() };
        unsafe { slice::from_raw_parts(p.xorMaskData, p.lengthXorMask as _) }
    }

    pub fn and_mask(&self) -> &[u8] {
        let p = unsafe { self.inner.as_ref() };
        unsafe { slice::from_raw_parts(p.andMaskData, p.lengthAndMask as _) }
    }

    pub fn bgra_data(&self, palette: &GdiPalette) -> Result<Vec<u8>> {
        let len = self.height() * self.width() * 4;
        let mut data = Vec::with_capacity(len as _);
        let res = unsafe {
            sys::freerdp_image_copy_from_pointer_data(
                data.as_mut_ptr(),
                PIXEL_FORMAT_BGRA32.into(),
                self.width() * 4,
                0,
                0,
                self.width(),
                self.height(),
                self.xor_mask().as_ptr(),
                self.xor_mask().len() as _,
                self.and_mask().as_ptr(),
                self.and_mask().len() as _,
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
pub struct Graphics {
    inner: ptr::NonNull<sys::rdpGraphics>,
}

impl Graphics {
    pub(crate) fn new(graphics: *mut sys::rdpGraphics) -> Self {
        Self {
            inner: ptr::NonNull::new(graphics).unwrap(),
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
    let context = ptr::NonNull::new(context as *mut RdpContext<H::ContextHandler>).unwrap();
    let mut context = Context::from_context(false, context);
    let mut inner = ptr::NonNull::new(pointer as *mut _ as *mut RdpPointer<H>).unwrap();

    unsafe { inner.as_mut() }
        .handler
        .new(&mut context, &Pointer::new(pointer))
        .is_ok() as _
}

extern "C" fn rdp_pointer_free<H: PointerHandler>(
    context: *mut sys::rdpContext,
    pointer: *mut sys::rdpPointer,
) {
    let context = ptr::NonNull::new(context as *mut RdpContext<H::ContextHandler>).unwrap();
    let mut context = Context::from_context(false, context);
    let mut inner = ptr::NonNull::new(pointer as *mut _ as *mut RdpPointer<H>).unwrap();

    unsafe { inner.as_mut() }
        .handler
        .free(&mut context, &Pointer::new(pointer));
}

extern "C" fn rdp_pointer_set<H: PointerHandler>(
    context: *mut sys::rdpContext,
    pointer: *const sys::rdpPointer,
) -> i32 {
    let context = ptr::NonNull::new(context as *mut RdpContext<H::ContextHandler>).unwrap();
    let mut context = Context::from_context(false, context);
    let mut inner = ptr::NonNull::new(pointer as *mut RdpPointer<H>).unwrap();

    unsafe { inner.as_mut() }
        .handler
        .set(&mut context, &Pointer::new(pointer as *mut _))
        .is_ok() as _
}

extern "C" fn rdp_pointer_set_null<H: PointerHandler>(context: *mut sys::rdpContext) -> i32 {
    let context = ptr::NonNull::new(context as *mut RdpContext<H::ContextHandler>).unwrap();
    let mut context = Context::from_context(false, context);

    H::set_null(&mut context).is_ok() as _
}

extern "C" fn rdp_pointer_set_default<H: PointerHandler>(context: *mut sys::rdpContext) -> i32 {
    let context = ptr::NonNull::new(context as *mut RdpContext<H::ContextHandler>).unwrap();
    let mut context = Context::from_context(false, context);

    H::set_default(&mut context).is_ok() as _
}

extern "C" fn rdp_pointer_set_position<H: PointerHandler>(
    context: *mut sys::rdpContext,
    x: u32,
    y: u32,
) -> i32 {
    let context = ptr::NonNull::new(context as *mut RdpContext<H::ContextHandler>).unwrap();
    let mut context = Context::from_context(false, context);

    H::set_position(&mut context, x, y).is_ok() as _
}
