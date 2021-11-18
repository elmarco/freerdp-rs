use std::{
    fmt::Debug,
    marker::PhantomData,
    mem::{self, size_of},
    ptr, slice,
};

use crate::{
    client::{Context, Handler, RdpContext},
    sys, Result,
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
}

pub trait PointerHandler {
    type ContextHandler: Handler;

    fn new(&mut self, context: &mut Context<Self::ContextHandler>, pointer: &Pointer)
        -> Result<()>;
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
    _context: *mut sys::rdpContext,
    _pointer: *mut sys::rdpPointer,
) {
    todo!()
}

extern "C" fn rdp_pointer_set<H: PointerHandler>(
    _context: *mut sys::rdpContext,
    _pointer: *const sys::rdpPointer,
) -> i32 {
    todo!()
}

extern "C" fn rdp_pointer_set_null<H: PointerHandler>(_context: *mut sys::rdpContext) -> i32 {
    todo!()
}

extern "C" fn rdp_pointer_set_default<H: PointerHandler>(_context: *mut sys::rdpContext) -> i32 {
    todo!()
}

extern "C" fn rdp_pointer_set_position<H: PointerHandler>(
    _context: *mut sys::rdpContext,
    _x: u32,
    _y: u32,
) -> i32 {
    todo!()
}
