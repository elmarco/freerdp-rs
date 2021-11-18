use crate::sys;

#[derive(Debug)]
pub enum ConnectionType {
    Modem,
    BroadbandLow,
    Satellite,
    BroadbandHigh,
    Wan,
    Lan,
    Auto,
}

impl From<ConnectionType> for u32 {
    fn from(type_: ConnectionType) -> Self {
        match type_ {
            ConnectionType::Modem => sys::CONNECTION_TYPE_MODEM,
            ConnectionType::BroadbandLow => sys::CONNECTION_TYPE_BROADBAND_LOW,
            ConnectionType::Satellite => sys::CONNECTION_TYPE_SATELLITE,
            ConnectionType::BroadbandHigh => sys::CONNECTION_TYPE_BROADBAND_HIGH,
            ConnectionType::Wan => sys::CONNECTION_TYPE_WAN,
            ConnectionType::Lan => sys::CONNECTION_TYPE_LAN,
            ConnectionType::Auto => sys::CONNECTION_TYPE_AUTODETECT,
        }
    }
}
