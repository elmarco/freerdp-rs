use core::slice;
use std::{marker::PhantomData, ptr};

use crate::{sys, RdpError, Result};

pub mod gfx;

pub mod video;

#[derive(Debug)]
pub struct GdiPalette {
    pub(crate) inner: ptr::NonNull<sys::gdi_palette>,
}

impl GdiPalette {
    pub(crate) fn new(palette: *mut sys::gdi_palette) -> Self {
        Self {
            // FIXME: const vs mut
            inner: ptr::NonNull::new(palette).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct GdiRgn {
    inner: ptr::NonNull<sys::GDI_RGN>,
}

impl GdiRgn {
    pub(crate) fn new(rgn: *mut sys::GDI_RGN) -> Self {
        Self {
            inner: ptr::NonNull::new(rgn).unwrap(),
        }
    }

    pub fn x(&self) -> i32 {
        let inner = unsafe { self.inner.as_ref() };
        inner.x
    }

    pub fn y(&self) -> i32 {
        let inner = unsafe { self.inner.as_ref() };
        inner.y
    }

    pub fn w(&self) -> i32 {
        let inner = unsafe { self.inner.as_ref() };
        inner.w
    }

    pub fn h(&self) -> i32 {
        let inner = unsafe { self.inner.as_ref() };
        inner.h
    }

    pub fn set_null(&mut self, null: bool) {
        let inner = unsafe { self.inner.as_mut() };
        inner.null = null as _;
    }

    pub fn null(&self) -> bool {
        let inner = unsafe { self.inner.as_ref() };
        inner.null != 0
    }
}

#[derive(Debug)]
pub struct GdiWnd {
    inner: ptr::NonNull<sys::GDI_WND>,
}

impl GdiWnd {
    pub(crate) fn new(wnd: *mut sys::GDI_WND) -> Self {
        Self {
            inner: ptr::NonNull::new(wnd).unwrap(),
        }
    }

    pub fn invalid(&mut self) -> GdiRgn {
        GdiRgn::new(unsafe { self.inner.as_mut() }.invalid)
    }
}

#[derive(Debug)]
pub struct GdiDC {
    inner: ptr::NonNull<sys::GDI_DC>,
}

impl GdiDC {
    pub(crate) fn new(dc: *mut sys::GDI_DC) -> Self {
        Self {
            inner: ptr::NonNull::new(dc).unwrap(),
        }
    }

    pub fn hwnd(&mut self) -> GdiWnd {
        GdiWnd::new(unsafe { self.inner.as_mut() }.hwnd)
    }
}

#[derive(Debug)]
pub struct GdiBitmap {
    inner: ptr::NonNull<sys::gdiBitmap>,
}

impl GdiBitmap {
    pub(crate) fn new(bitmap: *mut sys::gdiBitmap) -> Self {
        Self {
            inner: ptr::NonNull::new(bitmap).unwrap(),
        }
    }

    pub fn hdc(&mut self) -> GdiDC {
        GdiDC::new(unsafe { self.inner.as_mut() }.hdc)
    }
}

#[derive(Debug)]
pub struct Gdi<'a> {
    inner: ptr::NonNull<sys::rdpGdi>,
    palette: GdiPalette,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> Gdi<'a> {
    pub(crate) fn new(gdi: *mut sys::rdpGdi) -> Self {
        let mut inner = ptr::NonNull::new(gdi).unwrap();
        let palette = GdiPalette::new(&mut unsafe { inner.as_mut() }.palette);
        Self {
            inner,
            palette,
            _lifetime: PhantomData,
        }
    }

    pub fn primary(&self) -> Option<GdiBitmap> {
        let primary = unsafe { self.inner.as_ref() }.primary;
        if primary.is_null() {
            None
        } else {
            Some(GdiBitmap::new(primary))
        }
    }

    pub fn primary_buffer(&self) -> Option<&[u8]> {
        let buffer = unsafe { self.inner.as_ref() }.primary_buffer;
        if buffer.is_null() {
            None
        } else {
            Some(unsafe {
                slice::from_raw_parts(buffer, (self.stride() * self.height().unwrap()) as usize)
            })
        }
    }

    pub fn stride(&self) -> u32 {
        unsafe { self.inner.as_ref() }.stride
    }

    pub fn width(&self) -> Option<u32> {
        let w = unsafe { self.inner.as_ref() }.width;
        u32::try_from(w).ok()
    }

    pub fn height(&self) -> Option<u32> {
        let h = unsafe { self.inner.as_ref() }.height;
        u32::try_from(h).ok()
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        let res = unsafe { sys::gdi_resize(self.inner.as_ptr(), width, height) };
        if res == 0 {
            Err(RdpError::Failed("gdi_resize() failed".into()))
        } else {
            Ok(())
        }
    }

    pub fn palette(&self) -> &GdiPalette {
        &self.palette
    }
}
