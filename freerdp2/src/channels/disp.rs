use bitflags::bitflags;

use crate::sys;

pub const DVC_CHANNEL_NAME: &'static str = "Microsoft::Windows::RDS::DisplayControl";

bitflags! {
    pub struct MonitorFlags: u32 {
        const PRIMARY = 0b00000001;
    }
}

pub enum Orientation {
    Landscape,
    Portrait,
    LandscapeFlipped,
    PortraitFlipped,
}

impl From<Orientation> for u32 {
    fn from(o: Orientation) -> Self {
        match o {
            Orientation::Landscape => 0,
            Orientation::Portrait => 90,
            Orientation::LandscapeFlipped => 180,
            Orientation::PortraitFlipped => 270,
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct MonitorLayout(sys::DISPLAY_CONTROL_MONITOR_LAYOUT);

impl MonitorLayout {
    pub fn new(
        flags: MonitorFlags,
        left: i32,
        top: i32,
        width: u32,
        height: u32,
        physical_width: u32,
        physical_height: u32,
        orientation: Orientation,
        desktop_scale_factor: u32,
        device_scale_factor: u32,
    ) -> Self {
        Self(sys::DISPLAY_CONTROL_MONITOR_LAYOUT {
            Flags: flags.bits(),
            Left: left,
            Top: top,
            Width: width,
            Height: height,
            PhysicalWidth: physical_width,
            PhysicalHeight: physical_height,
            Orientation: orientation.into(),
            DesktopScaleFactor: desktop_scale_factor,
            DeviceScaleFactor: device_scale_factor,
        })
    }
}
