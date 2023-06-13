use core::slice;
use std::{
    ffi::{CStr, CString},
    mem, ptr,
};

use sys::CB_CAPSTYPE_GENERAL_LEN;

use crate::{
    channels::cliprdr::{Format, GeneralCapabilities},
    client::custom::Custom,
    sys, RdpError, Result,
};

#[derive(Debug)]
pub struct CliprdrFormat {
    pub id: Option<Format>,
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct CliprdrClientContext {
    pub(crate) inner: ptr::NonNull<sys::CliprdrClientContext>,
    owned: bool,
}

unsafe impl Send for CliprdrClientContext {}
unsafe impl Sync for CliprdrClientContext {}

impl Drop for CliprdrClientContext {
    fn drop(&mut self) {
        if !self.owned {
            return;
        }
        unsafe {
            let inner = self.inner.as_mut();
            drop(Box::from_raw(inner.custom as *mut Custom));
        }
    }
}

pub trait CliprdrHandler {
    fn monitor_ready(&mut self, _context: &mut CliprdrClientContext) -> Result<()> {
        Ok(())
    }

    fn server_capabilities(
        &mut self,
        _context: &mut CliprdrClientContext,
        _capabilities: Option<&GeneralCapabilities>,
    ) -> Result<()> {
        Ok(())
    }

    fn server_format_list(
        &mut self,
        _context: &mut CliprdrClientContext,
        _formats: &[CliprdrFormat],
    ) -> Result<()> {
        Ok(())
    }

    fn server_format_list_response(&mut self, _context: &mut CliprdrClientContext) -> Result<()> {
        Ok(())
    }

    fn server_format_data_request(
        &mut self,
        _context: &mut CliprdrClientContext,
        _format: Format,
    ) -> Result<()> {
        Err(RdpError::Unsupported)
    }

    fn server_format_data_response(
        &mut self,
        _context: &mut CliprdrClientContext,
        _data: &[u8],
    ) -> Result<()> {
        Ok(())
    }
}

impl CliprdrClientContext {
    /// # Safety
    ///
    /// * The memory pointed to by `ctxt` must contain a valid pointer.
    /// * `ctxt` must be [valid] for both reads and writes for the whole lifetime `'a` FIXME.
    pub unsafe fn from_ptr(ctxt: *mut sys::CliprdrClientContext, owned: bool) -> Self {
        Self {
            inner: ptr::NonNull::new(ctxt).unwrap(),
            owned,
        }
    }

    pub fn register_handler<H: CliprdrHandler>(&mut self, handler: H) {
        let inner = unsafe { self.inner.as_mut() };
        assert!(inner.custom.is_null());
        inner.MonitorReady = Some(rdp_cliprdr_monitor_ready::<H>);
        inner.ServerCapabilities = Some(rdp_cliprdr_server_capabilities::<H>);
        inner.ServerFormatList = Some(rdp_cliprdr_server_format_list::<H>);
        inner.ServerFormatListResponse = Some(rdp_cliprdr_server_format_list_response::<H>);
        inner.ServerFormatDataRequest = Some(rdp_cliprdr_server_format_data_request::<H>);
        inner.ServerFormatDataResponse = Some(rdp_cliprdr_server_format_data_response::<H>);
        inner.custom = Custom::new(handler);
    }

    // should be safe as long as inner.custom is set only once
    unsafe fn handler<'a, H: CliprdrHandler>(&mut self) -> &'a mut H {
        let custom = (self.inner.as_mut().custom as *mut Custom)
            .as_mut()
            .unwrap();
        (custom.handler as *mut H).as_mut().unwrap()
    }

    pub fn send_client_general_capabilities(
        &mut self,
        capabilities: &GeneralCapabilities,
    ) -> Result<()> {
        let mut cap: sys::CLIPRDR_CAPABILITIES = unsafe { mem::zeroed() };
        let mut general_cap: sys::CLIPRDR_GENERAL_CAPABILITY_SET = unsafe { mem::zeroed() };

        cap.cCapabilitiesSets = 1;
        cap.capabilitySets = &mut general_cap as *mut _ as *mut _;
        general_cap.capabilitySetType = sys::CB_CAPSTYPE_GENERAL as _;
        general_cap.capabilitySetLength = sys::CB_CAPSTYPE_GENERAL_LEN as _;
        general_cap.version = sys::CB_CAPS_VERSION_2;
        general_cap.generalFlags = capabilities.bits();

        let res = unsafe {
            let f = self.inner.as_ref().ClientCapabilities.unwrap();
            f(self.inner.as_ptr(), &cap)
        };

        if res == 0 {
            Ok(())
        } else {
            Err(RdpError::IOError(std::io::Error::from_raw_os_error(
                res as _,
            )))
        }
    }

    pub fn send_client_format_list(&mut self, formats: &[CliprdrFormat]) -> Result<()> {
        let mut list: sys::CLIPRDR_FORMAT_LIST = unsafe { mem::zeroed() };
        list.msgType = sys::CB_FORMAT_LIST as _;
        list.msgFlags = sys::CB_RESPONSE_OK as _;
        list.numFormats = formats.len() as _;
        let mut formats: Vec<_> = formats
            .iter()
            .map(|f| {
                let mut format: sys::CLIPRDR_FORMAT = unsafe { mem::zeroed() };
                if let Some(id) = f.id {
                    format.formatId = id as _;
                }
                if let Some(name) = &f.name {
                    format.formatName = CString::new(name.as_str()).unwrap().into_raw();
                }
                format
            })
            .collect();
        list.formats = formats.as_mut_ptr();
        let res = unsafe {
            let f = self.inner.as_ref().ClientFormatList.unwrap();
            f(self.inner.as_ptr(), &list)
        };
        for f in formats {
            if !f.formatName.is_null() {
                unsafe {
                    drop(CString::from_raw(f.formatName));
                }
            }
        }

        if res == 0 {
            Ok(())
        } else {
            Err(RdpError::IOError(std::io::Error::from_raw_os_error(
                res as _,
            )))
        }
    }

    pub fn send_client_format_list_response(&mut self, ok: bool) -> Result<()> {
        let mut rep: sys::CLIPRDR_FORMAT_LIST_RESPONSE = unsafe { mem::zeroed() };
        rep.msgType = sys::CB_FORMAT_LIST_RESPONSE as _;
        rep.msgFlags = if ok {
            sys::CB_RESPONSE_OK
        } else {
            sys::CB_RESPONSE_FAIL
        } as _;

        let res = unsafe {
            let f = self.inner.as_ref().ClientFormatListResponse.unwrap();
            f(self.inner.as_ptr(), &rep)
        };

        if res == 0 {
            Ok(())
        } else {
            Err(RdpError::IOError(std::io::Error::from_raw_os_error(
                res as _,
            )))
        }
    }

    pub fn send_client_format_data_request(&mut self, format: Format) -> Result<()> {
        let mut req: sys::CLIPRDR_FORMAT_DATA_REQUEST = unsafe { mem::zeroed() };
        req.requestedFormatId = format as _;
        let res = unsafe {
            let f = self.inner.as_ref().ClientFormatDataRequest.unwrap();
            f(self.inner.as_ptr(), &req)
        };

        if res == 0 {
            Ok(())
        } else {
            Err(RdpError::IOError(std::io::Error::from_raw_os_error(
                res as _,
            )))
        }
    }

    pub fn send_client_format_data_response(&mut self, data: Option<&[u8]>) -> Result<()> {
        let mut rep: sys::CLIPRDR_FORMAT_DATA_RESPONSE = unsafe { mem::zeroed() };
        rep.msgFlags = if let Some(data) = data {
            rep.dataLen = u32::try_from(data.len())?;
            rep.requestedFormatData = data.as_ptr();
            sys::CB_RESPONSE_OK
        } else {
            sys::CB_RESPONSE_FAIL
        } as _;

        let res = unsafe {
            let f = self.inner.as_ref().ClientFormatDataResponse.unwrap();
            f(self.inner.as_ptr(), &rep)
        };

        if res == 0 {
            Ok(())
        } else {
            Err(RdpError::IOError(std::io::Error::from_raw_os_error(
                res as _,
            )))
        }
    }
}

extern "C" fn rdp_cliprdr_monitor_ready<H: CliprdrHandler>(
    context: *mut sys::CliprdrClientContext,
    _ready: *const sys::CLIPRDR_MONITOR_READY,
) -> u32 {
    let mut ctxt = unsafe { CliprdrClientContext::from_ptr(context, false) };
    let handler = unsafe { ctxt.handler::<H>() };
    if handler.monitor_ready(&mut ctxt).is_ok() {
        0
    } else {
        1
    }
}

extern "C" fn rdp_cliprdr_server_capabilities<H: CliprdrHandler>(
    context: *mut sys::CliprdrClientContext,
    capabilities: *const sys::CLIPRDR_CAPABILITIES,
) -> u32 {
    let mut ctxt = unsafe { CliprdrClientContext::from_ptr(context, false) };
    let handler = unsafe { ctxt.handler::<H>() };
    let mut gen_caps = None;

    let capabilities = unsafe { capabilities.as_ref().unwrap() };
    let sets = unsafe {
        slice::from_raw_parts(
            capabilities.capabilitySets,
            capabilities.cCapabilitiesSets as _,
        )
    };
    for set in sets {
        if u32::from(set.capabilitySetType) == sys::CB_CAPSTYPE_GENERAL
            && u32::from(set.capabilitySetLength) >= CB_CAPSTYPE_GENERAL_LEN
        {
            let gen_set = unsafe {
                (set as *const _ as *const sys::CLIPRDR_GENERAL_CAPABILITY_SET)
                    .as_ref()
                    .unwrap()
            };
            gen_caps = Some(GeneralCapabilities::from_bits_truncate(
                gen_set.generalFlags,
            ));
            break;
        }
    }

    if handler
        .server_capabilities(&mut ctxt, gen_caps.as_ref())
        .is_ok()
    {
        0
    } else {
        1
    }
}

extern "C" fn rdp_cliprdr_server_format_list<H: CliprdrHandler>(
    context: *mut sys::CliprdrClientContext,
    list: *const sys::CLIPRDR_FORMAT_LIST,
) -> u32 {
    let mut ctxt = unsafe { CliprdrClientContext::from_ptr(context, false) };
    let handler = unsafe { ctxt.handler::<H>() };
    let list = unsafe { slice::from_raw_parts((*list).formats, (*list).numFormats as _) };

    let list: Vec<_> = list
        .iter()
        .map(|f| {
            let name = if f.formatName.is_null() {
                None
            } else {
                Some(
                    unsafe { CStr::from_ptr(f.formatName) }
                        .to_string_lossy()
                        .into(),
                )
            };
            let id = f.formatId.try_into().ok();
            CliprdrFormat { id, name }
        })
        .collect();

    if handler.server_format_list(&mut ctxt, &list).is_ok() {
        0
    } else {
        1
    }
}

extern "C" fn rdp_cliprdr_server_format_list_response<H: CliprdrHandler>(
    context: *mut sys::CliprdrClientContext,
    _resp: *const sys::CLIPRDR_FORMAT_LIST_RESPONSE,
) -> u32 {
    let mut ctxt = unsafe { CliprdrClientContext::from_ptr(context, false) };
    let handler = unsafe { ctxt.handler::<H>() };

    if handler.server_format_list_response(&mut ctxt).is_ok() {
        0
    } else {
        1
    }
}

extern "C" fn rdp_cliprdr_server_format_data_request<H: CliprdrHandler>(
    context: *mut sys::CliprdrClientContext,
    req: *const sys::CLIPRDR_FORMAT_DATA_REQUEST,
) -> u32 {
    let mut ctxt = unsafe { CliprdrClientContext::from_ptr(context, false) };
    let handler = unsafe { ctxt.handler::<H>() };
    if let Ok(format) = unsafe { (*req).requestedFormatId }.try_into() {
        if handler
            .server_format_data_request(&mut ctxt, format)
            .is_ok()
        {
            return 0;
        }
    }

    1
}

extern "C" fn rdp_cliprdr_server_format_data_response<H: CliprdrHandler>(
    context: *mut sys::CliprdrClientContext,
    resp: *const sys::CLIPRDR_FORMAT_DATA_RESPONSE,
) -> u32 {
    let mut ctxt = unsafe { CliprdrClientContext::from_ptr(context, false) };
    let handler = unsafe { ctxt.handler::<H>() };
    let data = unsafe { slice::from_raw_parts((*resp).requestedFormatData, (*resp).dataLen as _) };

    if handler.server_format_data_response(&mut ctxt, data).is_ok() {
        0
    } else {
        1
    }
}
