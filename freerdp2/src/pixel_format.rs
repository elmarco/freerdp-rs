pub enum PixelFormatType {
    A,
    ARGB,
    ABGR,
    RGBA,
    BGRA,
}

impl From<&PixelFormatType> for u32 {
    fn from(ty: &PixelFormatType) -> Self {
        match ty {
            PixelFormatType::A => 0,
            PixelFormatType::ARGB => 1,
            PixelFormatType::ABGR => 2,
            PixelFormatType::RGBA => 3,
            PixelFormatType::BGRA => 4,
        }
    }
}

pub struct PixelFormat {
    bpp: u8,
    type_: PixelFormatType,
    a: u8,
    r: u8,
    g: u8,
    b: u8,
}

impl From<&PixelFormat> for u32 {
    fn from(fmt: &PixelFormat) -> Self {
        (fmt.bpp as u32) << 24
            | u32::from(&fmt.type_) << 16
            | (fmt.a as u32) << 12
            | (fmt.r as u32) << 8
            | (fmt.g as u32) << 4
            | (fmt.b as u32)
    }
}

pub const PIXEL_FORMAT_ARGB32: &PixelFormat = &PixelFormat {
    bpp: 32,
    type_: PixelFormatType::ARGB,
    a: 8,
    r: 8,
    g: 8,
    b: 8,
};

pub const PIXEL_FORMAT_XRGB32: &PixelFormat = &PixelFormat {
    bpp: 32,
    type_: PixelFormatType::ARGB,
    a: 0,
    r: 8,
    g: 8,
    b: 8,
};

pub const PIXEL_FORMAT_ABGR32: &PixelFormat = &PixelFormat {
    bpp: 32,
    type_: PixelFormatType::ABGR,
    a: 8,
    r: 8,
    g: 8,
    b: 8,
};

pub const PIXEL_FORMAT_XBGR32: &PixelFormat = &PixelFormat {
    bpp: 32,
    type_: PixelFormatType::ABGR,
    a: 0,
    r: 8,
    g: 8,
    b: 8,
};

pub const PIXEL_FORMAT_BGRA32: &PixelFormat = &PixelFormat {
    bpp: 32,
    type_: PixelFormatType::BGRA,
    a: 8,
    r: 8,
    g: 8,
    b: 8,
};

pub const PIXEL_FORMAT_BGRX32: &PixelFormat = &PixelFormat {
    bpp: 32,
    type_: PixelFormatType::BGRA,
    a: 0,
    r: 8,
    g: 8,
    b: 8,
};

pub const PIXEL_FORMAT_RGBA32: &PixelFormat = &PixelFormat {
    bpp: 32,
    type_: PixelFormatType::RGBA,
    a: 8,
    r: 8,
    g: 8,
    b: 8,
};

pub const PIXEL_FORMAT_RGBX32: &PixelFormat = &PixelFormat {
    bpp: 32,
    type_: PixelFormatType::RGBA,
    a: 0,
    r: 8,
    g: 8,
    b: 8,
};
