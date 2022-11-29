use std::{ffi::CStr, num::TryFromIntError};

use crate::sys;

#[derive(Debug)]
pub enum RdpCodeClass {
    Base = 0,
    Info = 1,
    Connect = 2,
}

impl TryFrom<u32> for RdpCodeClass {
    type Error = RdpError;

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            v if v == RdpCodeClass::Base as u32 => Ok(RdpCodeClass::Base),
            v if v == RdpCodeClass::Info as u32 => Ok(RdpCodeClass::Info),
            v if v == RdpCodeClass::Connect as u32 => Ok(RdpCodeClass::Connect),
            _ => Err(RdpError::Unsupported),
        }
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum RdpErrBase {
    Success = sys::ERRBASE_SUCCESS,
    None = sys::ERRBASE_NONE,
}

impl TryFrom<u32> for RdpErrBase {
    type Error = RdpError;

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            v if v == RdpErrBase::Success as u32 => Ok(RdpErrBase::Success),
            v if v == RdpErrBase::None as u32 => Ok(RdpErrBase::None),
            _ => Err(RdpError::Unsupported),
        }
    }
}

impl TryFrom<u32> for RdpErrInfo {
    type Error = RdpError;

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            v if v == RdpErrInfo::RpcInitiatedDisconnected as u32 => {
                Ok(RdpErrInfo::RpcInitiatedDisconnected)
            }
            v if v == RdpErrInfo::RpcInitiatedLogoff as u32 => Ok(RdpErrInfo::RpcInitiatedLogoff),
            v if v == RdpErrInfo::IdleTimeout as u32 => Ok(RdpErrInfo::IdleTimeout),
            v if v == RdpErrInfo::LogonTimeout as u32 => Ok(RdpErrInfo::LogonTimeout),
            v if v == RdpErrInfo::DisconnectedByOtherConnection as u32 => {
                Ok(RdpErrInfo::DisconnectedByOtherConnection)
            }
            v if v == RdpErrInfo::OutOfMemory as u32 => Ok(RdpErrInfo::OutOfMemory),
            v if v == RdpErrInfo::ServerDeniedConnection as u32 => {
                Ok(RdpErrInfo::ServerDeniedConnection)
            }
            v if v == RdpErrInfo::ServerInsufficientPrivileges as u32 => {
                Ok(RdpErrInfo::ServerInsufficientPrivileges)
            }
            v if v == RdpErrInfo::ServerFreshCredentialsRequired as u32 => {
                Ok(RdpErrInfo::ServerFreshCredentialsRequired)
            }
            v if v == RdpErrInfo::RpcInitiatedDisconnectByUser as u32 => {
                Ok(RdpErrInfo::RpcInitiatedDisconnectByUser)
            }
            v if v == RdpErrInfo::LogoffByUser as u32 => Ok(RdpErrInfo::LogoffByUser),
            v if v == RdpErrInfo::CloseStackOnDriverNotReady as u32 => {
                Ok(RdpErrInfo::CloseStackOnDriverNotReady)
            }
            v if v == RdpErrInfo::ServerDwmCrash as u32 => Ok(RdpErrInfo::ServerDwmCrash),
            v if v == RdpErrInfo::CloseStackOnDriverFailure as u32 => {
                Ok(RdpErrInfo::CloseStackOnDriverFailure)
            }
            v if v == RdpErrInfo::CloseStackOnDriverIfaceFailure as u32 => {
                Ok(RdpErrInfo::CloseStackOnDriverIfaceFailure)
            }
            v if v == RdpErrInfo::ServerWinlogonCrash as u32 => Ok(RdpErrInfo::ServerWinlogonCrash),
            v if v == RdpErrInfo::ServerCsrssCrash as u32 => Ok(RdpErrInfo::ServerCsrssCrash),
            v if v == RdpErrInfo::LicenseInternal as u32 => Ok(RdpErrInfo::LicenseInternal),
            v if v == RdpErrInfo::LicenseNoLicenseServer as u32 => {
                Ok(RdpErrInfo::LicenseNoLicenseServer)
            }
            v if v == RdpErrInfo::LicenseNoLicense as u32 => Ok(RdpErrInfo::LicenseNoLicense),
            v if v == RdpErrInfo::LicenseBadClientMsg as u32 => Ok(RdpErrInfo::LicenseBadClientMsg),
            v if v == RdpErrInfo::LicenseHwidDoesntMatchLicense as u32 => {
                Ok(RdpErrInfo::LicenseHwidDoesntMatchLicense)
            }
            v if v == RdpErrInfo::LicenseBadClientLicense as u32 => {
                Ok(RdpErrInfo::LicenseBadClientLicense)
            }
            v if v == RdpErrInfo::LicenseCantFinishProtocol as u32 => {
                Ok(RdpErrInfo::LicenseCantFinishProtocol)
            }
            v if v == RdpErrInfo::LicenseClientEndedProtocol as u32 => {
                Ok(RdpErrInfo::LicenseClientEndedProtocol)
            }
            v if v == RdpErrInfo::LicenseBadClientEncryption as u32 => {
                Ok(RdpErrInfo::LicenseBadClientEncryption)
            }
            v if v == RdpErrInfo::LicenseCantUpgradeLicense as u32 => {
                Ok(RdpErrInfo::LicenseCantUpgradeLicense)
            }
            v if v == RdpErrInfo::LicenseNoRemoteConnections as u32 => {
                Ok(RdpErrInfo::LicenseNoRemoteConnections)
            }
            v if v == RdpErrInfo::CbDestinationNotFound as u32 => {
                Ok(RdpErrInfo::CbDestinationNotFound)
            }
            v if v == RdpErrInfo::CbLoadingDestination as u32 => {
                Ok(RdpErrInfo::CbLoadingDestination)
            }
            v if v == RdpErrInfo::CbRedirectingToDestination as u32 => {
                Ok(RdpErrInfo::CbRedirectingToDestination)
            }
            v if v == RdpErrInfo::CbSessionOnlineVmWake as u32 => {
                Ok(RdpErrInfo::CbSessionOnlineVmWake)
            }
            v if v == RdpErrInfo::CbSessionOnlineVmBoot as u32 => {
                Ok(RdpErrInfo::CbSessionOnlineVmBoot)
            }
            v if v == RdpErrInfo::CbSessionOnlineVmNoDns as u32 => {
                Ok(RdpErrInfo::CbSessionOnlineVmNoDns)
            }
            v if v == RdpErrInfo::CbDestinationPoolNotFree as u32 => {
                Ok(RdpErrInfo::CbDestinationPoolNotFree)
            }
            v if v == RdpErrInfo::CbConnectionCancelled as u32 => {
                Ok(RdpErrInfo::CbConnectionCancelled)
            }
            v if v == RdpErrInfo::CbConnectionErrorInvalidSettings as u32 => {
                Ok(RdpErrInfo::CbConnectionErrorInvalidSettings)
            }
            v if v == RdpErrInfo::CbSessionOnlineVmBootTimeout as u32 => {
                Ok(RdpErrInfo::CbSessionOnlineVmBootTimeout)
            }
            v if v == RdpErrInfo::CbSessionOnlineVmSessmonFailed as u32 => {
                Ok(RdpErrInfo::CbSessionOnlineVmSessmonFailed)
            }
            v if v == RdpErrInfo::UnknownDataPduType as u32 => Ok(RdpErrInfo::UnknownDataPduType),
            v if v == RdpErrInfo::UnknownPduType as u32 => Ok(RdpErrInfo::UnknownPduType),
            v if v == RdpErrInfo::DataPduSequence as u32 => Ok(RdpErrInfo::DataPduSequence),
            v if v == RdpErrInfo::ControlPduSequence as u32 => Ok(RdpErrInfo::ControlPduSequence),
            v if v == RdpErrInfo::InvalidControlPduAction as u32 => {
                Ok(RdpErrInfo::InvalidControlPduAction)
            }
            v if v == RdpErrInfo::InvalidInputPduType as u32 => Ok(RdpErrInfo::InvalidInputPduType),
            v if v == RdpErrInfo::InvalidInputPduMouse as u32 => {
                Ok(RdpErrInfo::InvalidInputPduMouse)
            }
            v if v == RdpErrInfo::InvalidRefreshRectPdu as u32 => {
                Ok(RdpErrInfo::InvalidRefreshRectPdu)
            }
            v if v == RdpErrInfo::CreateUserDataFailed as u32 => {
                Ok(RdpErrInfo::CreateUserDataFailed)
            }
            v if v == RdpErrInfo::ConnectFailed as u32 => Ok(RdpErrInfo::ConnectFailed),
            v if v == RdpErrInfo::ConfirmActiveHasWrongShareid as u32 => {
                Ok(RdpErrInfo::ConfirmActiveHasWrongShareid)
            }
            v if v == RdpErrInfo::ConfirmActiveHasWrongOriginator as u32 => {
                Ok(RdpErrInfo::ConfirmActiveHasWrongOriginator)
            }
            v if v == RdpErrInfo::PersistentKeyPduBadLength as u32 => {
                Ok(RdpErrInfo::PersistentKeyPduBadLength)
            }
            v if v == RdpErrInfo::PersistentKeyPduIllegalFirst as u32 => {
                Ok(RdpErrInfo::PersistentKeyPduIllegalFirst)
            }
            v if v == RdpErrInfo::PersistentKeyPduTooManyTotalKeys as u32 => {
                Ok(RdpErrInfo::PersistentKeyPduTooManyTotalKeys)
            }
            v if v == RdpErrInfo::PersistentKeyPduTooManyCacheKeys as u32 => {
                Ok(RdpErrInfo::PersistentKeyPduTooManyCacheKeys)
            }
            v if v == RdpErrInfo::InputPduBadLength as u32 => Ok(RdpErrInfo::InputPduBadLength),
            v if v == RdpErrInfo::BitmapCacheErrorPduBadLength as u32 => {
                Ok(RdpErrInfo::BitmapCacheErrorPduBadLength)
            }
            v if v == RdpErrInfo::SecurityDataTooShort as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort)
            }
            v if v == RdpErrInfo::VchannelDataTooShort as u32 => {
                Ok(RdpErrInfo::VchannelDataTooShort)
            }
            v if v == RdpErrInfo::ShareDataTooShort as u32 => Ok(RdpErrInfo::ShareDataTooShort),
            v if v == RdpErrInfo::BadSuppressOutputPdu as u32 => {
                Ok(RdpErrInfo::BadSuppressOutputPdu)
            }
            v if v == RdpErrInfo::ConfirmActivePduTooShort as u32 => {
                Ok(RdpErrInfo::ConfirmActivePduTooShort)
            }
            v if v == RdpErrInfo::CapabilitySetTooSmall as u32 => {
                Ok(RdpErrInfo::CapabilitySetTooSmall)
            }
            v if v == RdpErrInfo::CapabilitySetTooLarge as u32 => {
                Ok(RdpErrInfo::CapabilitySetTooLarge)
            }
            v if v == RdpErrInfo::NoCursorCache as u32 => Ok(RdpErrInfo::NoCursorCache),
            v if v == RdpErrInfo::BadCapabilities as u32 => Ok(RdpErrInfo::BadCapabilities),
            v if v == RdpErrInfo::VirtualChannelDecompression as u32 => {
                Ok(RdpErrInfo::VirtualChannelDecompression)
            }
            v if v == RdpErrInfo::InvalidVcCompressionType as u32 => {
                Ok(RdpErrInfo::InvalidVcCompressionType)
            }
            v if v == RdpErrInfo::InvalidChannelId as u32 => Ok(RdpErrInfo::InvalidChannelId),
            v if v == RdpErrInfo::VchannelsTooMany as u32 => Ok(RdpErrInfo::VchannelsTooMany),
            v if v == RdpErrInfo::RemoteappNotEnabled as u32 => Ok(RdpErrInfo::RemoteappNotEnabled),
            v if v == RdpErrInfo::CacheCapNotSet as u32 => Ok(RdpErrInfo::CacheCapNotSet),
            v if v == RdpErrInfo::BitmapCacheErrorPduBadLength2 as u32 => {
                Ok(RdpErrInfo::BitmapCacheErrorPduBadLength2)
            }
            v if v == RdpErrInfo::OffscreenCacheErrorPduBadLength as u32 => {
                Ok(RdpErrInfo::OffscreenCacheErrorPduBadLength)
            }
            v if v == RdpErrInfo::DrawninegridCacheErrorPduBadLength as u32 => {
                Ok(RdpErrInfo::DrawninegridCacheErrorPduBadLength)
            }
            v if v == RdpErrInfo::GdiplusPduBadLength as u32 => Ok(RdpErrInfo::GdiplusPduBadLength),
            v if v == RdpErrInfo::SecurityDataTooShort2 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort2)
            }
            v if v == RdpErrInfo::SecurityDataTooShort3 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort3)
            }
            v if v == RdpErrInfo::SecurityDataTooShort4 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort4)
            }
            v if v == RdpErrInfo::SecurityDataTooShort5 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort5)
            }
            v if v == RdpErrInfo::SecurityDataTooShort6 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort6)
            }
            v if v == RdpErrInfo::SecurityDataTooShort7 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort7)
            }
            v if v == RdpErrInfo::SecurityDataTooShort8 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort8)
            }
            v if v == RdpErrInfo::SecurityDataTooShort9 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort9)
            }
            v if v == RdpErrInfo::SecurityDataTooShort10 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort10)
            }
            v if v == RdpErrInfo::SecurityDataTooShort11 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort11)
            }
            v if v == RdpErrInfo::SecurityDataTooShort12 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort12)
            }
            v if v == RdpErrInfo::SecurityDataTooShort13 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort13)
            }
            v if v == RdpErrInfo::SecurityDataTooShort14 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort14)
            }
            v if v == RdpErrInfo::SecurityDataTooShort15 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort15)
            }
            v if v == RdpErrInfo::SecurityDataTooShort16 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort16)
            }
            v if v == RdpErrInfo::SecurityDataTooShort17 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort17)
            }
            v if v == RdpErrInfo::SecurityDataTooShort18 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort18)
            }
            v if v == RdpErrInfo::SecurityDataTooShort19 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort19)
            }
            v if v == RdpErrInfo::SecurityDataTooShort20 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort20)
            }
            v if v == RdpErrInfo::SecurityDataTooShort21 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort21)
            }
            v if v == RdpErrInfo::SecurityDataTooShort22 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort22)
            }
            v if v == RdpErrInfo::SecurityDataTooShort23 as u32 => {
                Ok(RdpErrInfo::SecurityDataTooShort23)
            }
            v if v == RdpErrInfo::BadMonitorData as u32 => Ok(RdpErrInfo::BadMonitorData),
            v if v == RdpErrInfo::VcDecompressedReassembleFailed as u32 => {
                Ok(RdpErrInfo::VcDecompressedReassembleFailed)
            }
            v if v == RdpErrInfo::VcDataTooLong as u32 => Ok(RdpErrInfo::VcDataTooLong),
            v if v == RdpErrInfo::BadFrameAckData as u32 => Ok(RdpErrInfo::BadFrameAckData),
            v if v == RdpErrInfo::GraphicsModeNotSupported as u32 => {
                Ok(RdpErrInfo::GraphicsModeNotSupported)
            }
            v if v == RdpErrInfo::GraphicsSubsystemResetFailed as u32 => {
                Ok(RdpErrInfo::GraphicsSubsystemResetFailed)
            }
            v if v == RdpErrInfo::GraphicsSubsystemFailed as u32 => {
                Ok(RdpErrInfo::GraphicsSubsystemFailed)
            }
            v if v == RdpErrInfo::TimezoneKeyNameLengthTooShort as u32 => {
                Ok(RdpErrInfo::TimezoneKeyNameLengthTooShort)
            }
            v if v == RdpErrInfo::TimezoneKeyNameLengthTooLong as u32 => {
                Ok(RdpErrInfo::TimezoneKeyNameLengthTooLong)
            }
            v if v == RdpErrInfo::DynamicDstDisabledFieldMissing as u32 => {
                Ok(RdpErrInfo::DynamicDstDisabledFieldMissing)
            }
            v if v == RdpErrInfo::VcDecodingError as u32 => Ok(RdpErrInfo::VcDecodingError),
            v if v == RdpErrInfo::Virtualdesktoptoolarge as u32 => {
                Ok(RdpErrInfo::Virtualdesktoptoolarge)
            }
            v if v == RdpErrInfo::Monitorgeometryvalidationfailed as u32 => {
                Ok(RdpErrInfo::Monitorgeometryvalidationfailed)
            }
            v if v == RdpErrInfo::Invalidmonitorcount as u32 => Ok(RdpErrInfo::Invalidmonitorcount),
            v if v == RdpErrInfo::UpdateSessionKeyFailed as u32 => {
                Ok(RdpErrInfo::UpdateSessionKeyFailed)
            }
            v if v == RdpErrInfo::DecryptFailed as u32 => Ok(RdpErrInfo::DecryptFailed),
            v if v == RdpErrInfo::EncryptFailed as u32 => Ok(RdpErrInfo::EncryptFailed),
            v if v == RdpErrInfo::EncryptionPackageMismatch as u32 => {
                Ok(RdpErrInfo::EncryptionPackageMismatch)
            }
            v if v == RdpErrInfo::DecryptFailed2 as u32 => Ok(RdpErrInfo::DecryptFailed2),
            v if v == RdpErrInfo::PeerDisconnected as u32 => Ok(RdpErrInfo::PeerDisconnected),
            v if v == RdpErrInfo::Success as u32 => Ok(RdpErrInfo::Success),
            v if v == RdpErrInfo::None as u32 => Ok(RdpErrInfo::None),
            _ => Err(RdpError::Unsupported),
        }
    }
}

impl TryFrom<u32> for RdpErrConnect {
    type Error = RdpError;

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            v if v == RdpErrConnect::PreConnectFailed as u32 => Ok(RdpErrConnect::PreConnectFailed),
            v if v == RdpErrConnect::ConnectUndefined as u32 => Ok(RdpErrConnect::ConnectUndefined),
            v if v == RdpErrConnect::PostConnectFailed as u32 => {
                Ok(RdpErrConnect::PostConnectFailed)
            }
            v if v == RdpErrConnect::DnsError as u32 => Ok(RdpErrConnect::DnsError),
            v if v == RdpErrConnect::DnsNameNotFound as u32 => Ok(RdpErrConnect::DnsNameNotFound),
            v if v == RdpErrConnect::ConnectFailed as u32 => Ok(RdpErrConnect::ConnectFailed),
            v if v == RdpErrConnect::McsConnectInitialError as u32 => {
                Ok(RdpErrConnect::McsConnectInitialError)
            }
            v if v == RdpErrConnect::TlsConnectFailed as u32 => Ok(RdpErrConnect::TlsConnectFailed),
            v if v == RdpErrConnect::AuthenticationFailed as u32 => {
                Ok(RdpErrConnect::AuthenticationFailed)
            }
            v if v == RdpErrConnect::InsufficientPrivileges as u32 => {
                Ok(RdpErrConnect::InsufficientPrivileges)
            }
            v if v == RdpErrConnect::ConnectCancelled as u32 => Ok(RdpErrConnect::ConnectCancelled),
            v if v == RdpErrConnect::SecurityNegoConnectFailed as u32 => {
                Ok(RdpErrConnect::SecurityNegoConnectFailed)
            }
            v if v == RdpErrConnect::ConnectTransportFailed as u32 => {
                Ok(RdpErrConnect::ConnectTransportFailed)
            }
            v if v == RdpErrConnect::PasswordExpired as u32 => Ok(RdpErrConnect::PasswordExpired),
            v if v == RdpErrConnect::PasswordCertainlyExpired as u32 => {
                Ok(RdpErrConnect::PasswordCertainlyExpired)
            }
            v if v == RdpErrConnect::ClientRevoked as u32 => Ok(RdpErrConnect::ClientRevoked),
            v if v == RdpErrConnect::KdcUnreachable as u32 => Ok(RdpErrConnect::KdcUnreachable),
            v if v == RdpErrConnect::AccountDisabled as u32 => Ok(RdpErrConnect::AccountDisabled),
            v if v == RdpErrConnect::PasswordMustChange as u32 => {
                Ok(RdpErrConnect::PasswordMustChange)
            }
            v if v == RdpErrConnect::LogonFailure as u32 => Ok(RdpErrConnect::LogonFailure),
            v if v == RdpErrConnect::WrongPassword as u32 => Ok(RdpErrConnect::WrongPassword),
            v if v == RdpErrConnect::AccessDenied as u32 => Ok(RdpErrConnect::AccessDenied),
            v if v == RdpErrConnect::AccountRestriction as u32 => {
                Ok(RdpErrConnect::AccountRestriction)
            }
            v if v == RdpErrConnect::AccountLockedOut as u32 => Ok(RdpErrConnect::AccountLockedOut),
            v if v == RdpErrConnect::AccountExpired as u32 => Ok(RdpErrConnect::AccountExpired),
            v if v == RdpErrConnect::LogonTypeNotGranted as u32 => {
                Ok(RdpErrConnect::LogonTypeNotGranted)
            }
            v if v == RdpErrConnect::NoOrMissingCredentials as u32 => {
                Ok(RdpErrConnect::NoOrMissingCredentials)
            }
            v if v == RdpErrConnect::Success as u32 => Ok(RdpErrConnect::Success),
            v if v == RdpErrConnect::None as u32 => Ok(RdpErrConnect::None),
            _ => Err(RdpError::Unsupported),
        }
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum RdpErrInfo {
    RpcInitiatedDisconnected = sys::ERRINFO_RPC_INITIATED_DISCONNECT,
    RpcInitiatedLogoff = sys::ERRINFO_RPC_INITIATED_LOGOFF,
    IdleTimeout = sys::ERRINFO_IDLE_TIMEOUT,
    LogonTimeout = sys::ERRINFO_LOGON_TIMEOUT,
    DisconnectedByOtherConnection = sys::ERRINFO_DISCONNECTED_BY_OTHER_CONNECTION,
    OutOfMemory = sys::ERRINFO_OUT_OF_MEMORY,
    ServerDeniedConnection = sys::ERRINFO_SERVER_DENIED_CONNECTION,
    ServerInsufficientPrivileges = sys::ERRINFO_SERVER_INSUFFICIENT_PRIVILEGES,
    ServerFreshCredentialsRequired = sys::ERRINFO_SERVER_FRESH_CREDENTIALS_REQUIRED,
    RpcInitiatedDisconnectByUser = sys::ERRINFO_RPC_INITIATED_DISCONNECT_BY_USER,
    LogoffByUser = sys::ERRINFO_LOGOFF_BY_USER,
    CloseStackOnDriverNotReady = sys::ERRINFO_CLOSE_STACK_ON_DRIVER_NOT_READY,
    ServerDwmCrash = sys::ERRINFO_SERVER_DWM_CRASH,
    CloseStackOnDriverFailure = sys::ERRINFO_CLOSE_STACK_ON_DRIVER_FAILURE,
    CloseStackOnDriverIfaceFailure = sys::ERRINFO_CLOSE_STACK_ON_DRIVER_IFACE_FAILURE,
    ServerWinlogonCrash = sys::ERRINFO_SERVER_WINLOGON_CRASH,
    ServerCsrssCrash = sys::ERRINFO_SERVER_CSRSS_CRASH,
    LicenseInternal = sys::ERRINFO_LICENSE_INTERNAL,
    LicenseNoLicenseServer = sys::ERRINFO_LICENSE_NO_LICENSE_SERVER,
    LicenseNoLicense = sys::ERRINFO_LICENSE_NO_LICENSE,
    LicenseBadClientMsg = sys::ERRINFO_LICENSE_BAD_CLIENT_MSG,
    LicenseHwidDoesntMatchLicense = sys::ERRINFO_LICENSE_HWID_DOESNT_MATCH_LICENSE,
    LicenseBadClientLicense = sys::ERRINFO_LICENSE_BAD_CLIENT_LICENSE,
    LicenseCantFinishProtocol = sys::ERRINFO_LICENSE_CANT_FINISH_PROTOCOL,
    LicenseClientEndedProtocol = sys::ERRINFO_LICENSE_CLIENT_ENDED_PROTOCOL,
    LicenseBadClientEncryption = sys::ERRINFO_LICENSE_BAD_CLIENT_ENCRYPTION,
    LicenseCantUpgradeLicense = sys::ERRINFO_LICENSE_CANT_UPGRADE_LICENSE,
    LicenseNoRemoteConnections = sys::ERRINFO_LICENSE_NO_REMOTE_CONNECTIONS,
    CbDestinationNotFound = sys::ERRINFO_CB_DESTINATION_NOT_FOUND,
    CbLoadingDestination = sys::ERRINFO_CB_LOADING_DESTINATION,
    CbRedirectingToDestination = sys::ERRINFO_CB_REDIRECTING_TO_DESTINATION,
    CbSessionOnlineVmWake = sys::ERRINFO_CB_SESSION_ONLINE_VM_WAKE,
    CbSessionOnlineVmBoot = sys::ERRINFO_CB_SESSION_ONLINE_VM_BOOT,
    CbSessionOnlineVmNoDns = sys::ERRINFO_CB_SESSION_ONLINE_VM_NO_DNS,
    CbDestinationPoolNotFree = sys::ERRINFO_CB_DESTINATION_POOL_NOT_FREE,
    CbConnectionCancelled = sys::ERRINFO_CB_CONNECTION_CANCELLED,
    CbConnectionErrorInvalidSettings = sys::ERRINFO_CB_CONNECTION_ERROR_INVALID_SETTINGS,
    CbSessionOnlineVmBootTimeout = sys::ERRINFO_CB_SESSION_ONLINE_VM_BOOT_TIMEOUT,
    CbSessionOnlineVmSessmonFailed = sys::ERRINFO_CB_SESSION_ONLINE_VM_SESSMON_FAILED,
    UnknownDataPduType = sys::ERRINFO_UNKNOWN_DATA_PDU_TYPE,
    UnknownPduType = sys::ERRINFO_UNKNOWN_PDU_TYPE,
    DataPduSequence = sys::ERRINFO_DATA_PDU_SEQUENCE,
    ControlPduSequence = sys::ERRINFO_CONTROL_PDU_SEQUENCE,
    InvalidControlPduAction = sys::ERRINFO_INVALID_CONTROL_PDU_ACTION,
    InvalidInputPduType = sys::ERRINFO_INVALID_INPUT_PDU_TYPE,
    InvalidInputPduMouse = sys::ERRINFO_INVALID_INPUT_PDU_MOUSE,
    InvalidRefreshRectPdu = sys::ERRINFO_INVALID_REFRESH_RECT_PDU,
    CreateUserDataFailed = sys::ERRINFO_CREATE_USER_DATA_FAILED,
    ConnectFailed = sys::ERRINFO_CONNECT_FAILED,
    ConfirmActiveHasWrongShareid = sys::ERRINFO_CONFIRM_ACTIVE_HAS_WRONG_SHAREID,
    ConfirmActiveHasWrongOriginator = sys::ERRINFO_CONFIRM_ACTIVE_HAS_WRONG_ORIGINATOR,
    PersistentKeyPduBadLength = sys::ERRINFO_PERSISTENT_KEY_PDU_BAD_LENGTH,
    PersistentKeyPduIllegalFirst = sys::ERRINFO_PERSISTENT_KEY_PDU_ILLEGAL_FIRST,
    PersistentKeyPduTooManyTotalKeys = sys::ERRINFO_PERSISTENT_KEY_PDU_TOO_MANY_TOTAL_KEYS,
    PersistentKeyPduTooManyCacheKeys = sys::ERRINFO_PERSISTENT_KEY_PDU_TOO_MANY_CACHE_KEYS,
    InputPduBadLength = sys::ERRINFO_INPUT_PDU_BAD_LENGTH,
    BitmapCacheErrorPduBadLength = sys::ERRINFO_BITMAP_CACHE_ERROR_PDU_BAD_LENGTH,
    SecurityDataTooShort = sys::ERRINFO_SECURITY_DATA_TOO_SHORT,
    VchannelDataTooShort = sys::ERRINFO_VCHANNEL_DATA_TOO_SHORT,
    ShareDataTooShort = sys::ERRINFO_SHARE_DATA_TOO_SHORT,
    BadSuppressOutputPdu = sys::ERRINFO_BAD_SUPPRESS_OUTPUT_PDU,
    ConfirmActivePduTooShort = sys::ERRINFO_CONFIRM_ACTIVE_PDU_TOO_SHORT,
    CapabilitySetTooSmall = sys::ERRINFO_CAPABILITY_SET_TOO_SMALL,
    CapabilitySetTooLarge = sys::ERRINFO_CAPABILITY_SET_TOO_LARGE,
    NoCursorCache = sys::ERRINFO_NO_CURSOR_CACHE,
    BadCapabilities = sys::ERRINFO_BAD_CAPABILITIES,
    VirtualChannelDecompression = sys::ERRINFO_VIRTUAL_CHANNEL_DECOMPRESSION,
    InvalidVcCompressionType = sys::ERRINFO_INVALID_VC_COMPRESSION_TYPE,
    InvalidChannelId = sys::ERRINFO_INVALID_CHANNEL_ID,
    VchannelsTooMany = sys::ERRINFO_VCHANNELS_TOO_MANY,
    RemoteappNotEnabled = sys::ERRINFO_REMOTEAPP_NOT_ENABLED,
    CacheCapNotSet = sys::ERRINFO_CACHE_CAP_NOT_SET,
    BitmapCacheErrorPduBadLength2 = sys::ERRINFO_BITMAP_CACHE_ERROR_PDU_BAD_LENGTH2,
    OffscreenCacheErrorPduBadLength = sys::ERRINFO_OFFSCREEN_CACHE_ERROR_PDU_BAD_LENGTH,
    DrawninegridCacheErrorPduBadLength = sys::ERRINFO_DRAWNINEGRID_CACHE_ERROR_PDU_BAD_LENGTH,
    GdiplusPduBadLength = sys::ERRINFO_GDIPLUS_PDU_BAD_LENGTH,
    SecurityDataTooShort2 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT2,
    SecurityDataTooShort3 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT3,
    SecurityDataTooShort4 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT4,
    SecurityDataTooShort5 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT5,
    SecurityDataTooShort6 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT6,
    SecurityDataTooShort7 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT7,
    SecurityDataTooShort8 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT8,
    SecurityDataTooShort9 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT9,
    SecurityDataTooShort10 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT10,
    SecurityDataTooShort11 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT11,
    SecurityDataTooShort12 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT12,
    SecurityDataTooShort13 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT13,
    SecurityDataTooShort14 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT14,
    SecurityDataTooShort15 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT15,
    SecurityDataTooShort16 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT16,
    SecurityDataTooShort17 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT17,
    SecurityDataTooShort18 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT18,
    SecurityDataTooShort19 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT19,
    SecurityDataTooShort20 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT20,
    SecurityDataTooShort21 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT21,
    SecurityDataTooShort22 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT22,
    SecurityDataTooShort23 = sys::ERRINFO_SECURITY_DATA_TOO_SHORT23,
    BadMonitorData = sys::ERRINFO_BAD_MONITOR_DATA,
    VcDecompressedReassembleFailed = sys::ERRINFO_VC_DECOMPRESSED_REASSEMBLE_FAILED,
    VcDataTooLong = sys::ERRINFO_VC_DATA_TOO_LONG,
    BadFrameAckData = sys::ERRINFO_BAD_FRAME_ACK_DATA,
    GraphicsModeNotSupported = sys::ERRINFO_GRAPHICS_MODE_NOT_SUPPORTED,
    GraphicsSubsystemResetFailed = sys::ERRINFO_GRAPHICS_SUBSYSTEM_RESET_FAILED,
    GraphicsSubsystemFailed = sys::ERRINFO_GRAPHICS_SUBSYSTEM_FAILED,
    TimezoneKeyNameLengthTooShort = sys::ERRINFO_TIMEZONE_KEY_NAME_LENGTH_TOO_SHORT,
    TimezoneKeyNameLengthTooLong = sys::ERRINFO_TIMEZONE_KEY_NAME_LENGTH_TOO_LONG,
    DynamicDstDisabledFieldMissing = sys::ERRINFO_DYNAMIC_DST_DISABLED_FIELD_MISSING,
    VcDecodingError = sys::ERRINFO_VC_DECODING_ERROR,
    Virtualdesktoptoolarge = sys::ERRINFO_VIRTUALDESKTOPTOOLARGE,
    Monitorgeometryvalidationfailed = sys::ERRINFO_MONITORGEOMETRYVALIDATIONFAILED,
    Invalidmonitorcount = sys::ERRINFO_INVALIDMONITORCOUNT,
    UpdateSessionKeyFailed = sys::ERRINFO_UPDATE_SESSION_KEY_FAILED,
    DecryptFailed = sys::ERRINFO_DECRYPT_FAILED,
    EncryptFailed = sys::ERRINFO_ENCRYPT_FAILED,
    EncryptionPackageMismatch = sys::ERRINFO_ENCRYPTION_PACKAGE_MISMATCH,
    DecryptFailed2 = sys::ERRINFO_DECRYPT_FAILED2,
    PeerDisconnected = sys::ERRINFO_PEER_DISCONNECTED,
    Success = sys::ERRINFO_SUCCESS,
    None = sys::ERRINFO_NONE,
}

#[repr(u32)]
#[derive(Debug)]
pub enum RdpErrConnect {
    PreConnectFailed = sys::ERRCONNECT_PRE_CONNECT_FAILED,
    ConnectUndefined = sys::ERRCONNECT_CONNECT_UNDEFINED,
    PostConnectFailed = sys::ERRCONNECT_POST_CONNECT_FAILED,
    DnsError = sys::ERRCONNECT_DNS_ERROR,
    DnsNameNotFound = sys::ERRCONNECT_DNS_NAME_NOT_FOUND,
    ConnectFailed = sys::ERRCONNECT_CONNECT_FAILED,
    McsConnectInitialError = sys::ERRCONNECT_MCS_CONNECT_INITIAL_ERROR,
    TlsConnectFailed = sys::ERRCONNECT_TLS_CONNECT_FAILED,
    AuthenticationFailed = sys::ERRCONNECT_AUTHENTICATION_FAILED,
    InsufficientPrivileges = sys::ERRCONNECT_INSUFFICIENT_PRIVILEGES,
    ConnectCancelled = sys::ERRCONNECT_CONNECT_CANCELLED,
    SecurityNegoConnectFailed = sys::ERRCONNECT_SECURITY_NEGO_CONNECT_FAILED,
    ConnectTransportFailed = sys::ERRCONNECT_CONNECT_TRANSPORT_FAILED,
    PasswordExpired = sys::ERRCONNECT_PASSWORD_EXPIRED,
    PasswordCertainlyExpired = sys::ERRCONNECT_PASSWORD_CERTAINLY_EXPIRED,
    ClientRevoked = sys::ERRCONNECT_CLIENT_REVOKED,
    KdcUnreachable = sys::ERRCONNECT_KDC_UNREACHABLE,
    AccountDisabled = sys::ERRCONNECT_ACCOUNT_DISABLED,
    PasswordMustChange = sys::ERRCONNECT_PASSWORD_MUST_CHANGE,
    LogonFailure = sys::ERRCONNECT_LOGON_FAILURE,
    WrongPassword = sys::ERRCONNECT_WRONG_PASSWORD,
    AccessDenied = sys::ERRCONNECT_ACCESS_DENIED,
    AccountRestriction = sys::ERRCONNECT_ACCOUNT_RESTRICTION,
    AccountLockedOut = sys::ERRCONNECT_ACCOUNT_LOCKED_OUT,
    AccountExpired = sys::ERRCONNECT_ACCOUNT_EXPIRED,
    LogonTypeNotGranted = sys::ERRCONNECT_LOGON_TYPE_NOT_GRANTED,
    NoOrMissingCredentials = sys::ERRCONNECT_NO_OR_MISSING_CREDENTIALS,
    Success = sys::ERRCONNECT_SUCCESS,
    None = sys::ERRCONNECT_NONE,
}

#[derive(Debug)]
pub enum RdpErr {
    RdpErrBase(RdpErrBase),
    RdpErrInfo(RdpErrInfo),
    RdpErrConnect(RdpErrConnect),
}

#[repr(transparent)]
#[derive(Debug)]
pub struct RdpCode(pub u32);

impl std::fmt::Display for RdpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} ({:#x}): {}",
            unsafe {
                CStr::from_ptr(sys::freerdp_get_last_error_name(self.0))
                    .to_str()
                    .unwrap()
            },
            self.0,
            unsafe {
                CStr::from_ptr(sys::freerdp_get_last_error_string(self.0))
                    .to_str()
                    .unwrap()
            },
        )
    }
}

impl RdpCode {
    pub fn class(&self) -> Option<RdpCodeClass> {
        (self.0 >> 16).try_into().ok()
    }

    pub fn as_err(&self) -> Option<RdpErr> {
        let type_ = self.0 & 0xffff;
        self.class().and_then(|c| match c {
            RdpCodeClass::Base => type_.try_into().ok().map(RdpErr::RdpErrBase),
            RdpCodeClass::Info => type_.try_into().ok().map(RdpErr::RdpErrInfo),
            RdpCodeClass::Connect => type_.try_into().ok().map(RdpErr::RdpErrConnect),
        })
    }
}

/// RdpError enumerates all possible errors returned by this library.
#[derive(Debug)]
pub enum RdpError {
    /// Early return or missing impementation
    Unsupported,

    /// A generic error.
    Failed(String),

    /// A FreeRDP error code.
    Code(RdpCode),

    /// A FFI string error.
    NulError(std::ffi::NulError),

    /// Represents all other cases of `std::io::Error`.
    IOError(std::io::Error),

    /// Conversion error
    TryFromIntError(TryFromIntError),
}

impl std::error::Error for RdpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            RdpError::Unsupported => None,
            RdpError::Failed(_) => None,
            RdpError::Code(_) => None,
            RdpError::NulError(ref err) => Some(err),
            RdpError::IOError(ref err) => Some(err),
            RdpError::TryFromIntError(ref err) => Some(err),
        }
    }
}

impl std::fmt::Display for RdpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            RdpError::Unsupported => {
                write!(f, "Unsupported")
            }
            RdpError::Failed(ref err) => {
                write!(f, "{}", err)
            }
            RdpError::Code(ref err) => err.fmt(f),
            RdpError::NulError(ref err) => err.fmt(f),
            RdpError::IOError(ref err) => err.fmt(f),
            RdpError::TryFromIntError(ref err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for RdpError {
    fn from(err: std::io::Error) -> RdpError {
        RdpError::IOError(err)
    }
}

impl From<std::ffi::NulError> for RdpError {
    fn from(err: std::ffi::NulError) -> RdpError {
        RdpError::NulError(err)
    }
}

impl From<TryFromIntError> for RdpError {
    fn from(err: TryFromIntError) -> RdpError {
        RdpError::TryFromIntError(err)
    }
}

pub type Result<T> = std::result::Result<T, RdpError>;
