use crate::{
    client::{GeometryClientContext, VideoClientContext},
    gdi::Gdi,
    sys, Result,
};

pub fn geometry_init(gdi: &Gdi, context: &GeometryClientContext) -> Result<()> {
    unsafe { sys::gdi_video_geometry_init(gdi.inner.as_ptr(), context.inner.as_ptr()) };
    Ok(())
}

pub fn geometry_uninit(gdi: &Gdi, context: &GeometryClientContext) -> Result<()> {
    unsafe { sys::gdi_video_geometry_uninit(gdi.inner.as_ptr(), context.inner.as_ptr()) };
    Ok(())
}

pub fn control_init(gdi: &Gdi, context: &VideoClientContext) -> Result<()> {
    unsafe { sys::gdi_video_control_init(gdi.inner.as_ptr(), context.inner.as_ptr()) };
    Ok(())
}

pub fn control_uninit(gdi: &Gdi, context: &VideoClientContext) -> Result<()> {
    unsafe { sys::gdi_video_control_uninit(gdi.inner.as_ptr(), context.inner.as_ptr()) };
    Ok(())
}

pub fn data_init(gdi: &Gdi, context: &VideoClientContext) -> Result<()> {
    unsafe { sys::gdi_video_data_init(gdi.inner.as_ptr(), context.inner.as_ptr()) };
    Ok(())
}

pub fn data_uninit(gdi: &Gdi, context: &VideoClientContext) -> Result<()> {
    unsafe { sys::gdi_video_data_uninit(gdi.inner.as_ptr(), context.inner.as_ptr()) };
    Ok(())
}
