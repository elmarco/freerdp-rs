use crate::{client::RdpgfxClientContext, gdi::Gdi, sys, RdpError, Result};

pub fn graphics_pipeline_init(gdi: &Gdi, context: &RdpgfxClientContext) -> Result<()> {
    let res =
        unsafe { sys::gdi_graphics_pipeline_init(gdi.inner.as_ptr(), context.inner.as_ptr()) };
    if res == 0 {
        return Err(RdpError::Failed(
            "gdi_graphics_pipeline_init() failed".into(),
        ));
    }
    Ok(())
}

pub fn graphics_pipeline_uninit(gdi: &Gdi, context: &RdpgfxClientContext) -> Result<()> {
    unsafe { sys::gdi_graphics_pipeline_uninit(gdi.inner.as_ptr(), context.inner.as_ptr()) };
    Ok(())
}
