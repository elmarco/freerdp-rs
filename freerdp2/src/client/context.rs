use std::{
    ffi::CStr,
    mem::{size_of, MaybeUninit},
    os::raw::{c_char, c_int},
    ptr,
};

use crate::{
    channels,
    client::{
        CliprdrClientContext, DispClientContext, EncomspClientContext, EventChannelConnected,
        EventChannelDisconnected, GeometryClientContext, PubSub, PubSubHandle, PubSubHandler,
        RdpeiClientContext, RdpgfxClientContext, VideoClientContext,
    },
    gdi::{self, Gdi},
    graphics::Graphics,
    input::Input,
    sys,
    update::Update,
    winpr::{self, Handle},
    FreeRdp, RdpCode, RdpError, Result, Settings,
};

// this struct is allocated from C/freerdp, to improve
#[repr(C)]
#[derive(Debug)]
pub(crate) struct RdpContext<H: Handler> {
    rdp_context: sys::rdpContext,
    context: ptr::NonNull<Context<H>>,
}

unsafe impl<H> Send for RdpContext<H> where H: Handler + Send {}
unsafe impl<H> Sync for RdpContext<H> where H: Handler + Sync {}

impl<H: Handler> RdpContext<H> {
    fn from_ptr<'a>(ptr: *mut sys::rdpContext) -> &'a mut Self {
        let ptr = ptr as *mut Self;
        unsafe { ptr::NonNull::new(ptr).unwrap().as_mut() }
    }

    fn context(&mut self) -> &mut Context<H> {
        unsafe { self.context.as_mut() }
    }
}

#[derive(Debug)]
pub struct Context<H: Handler> {
    pub handler: H,
    pub settings: Settings,
    pub instance: FreeRdp,
    pub pub_sub: PubSub<H>,
    pub rdpei: Option<RdpeiClientContext>,
    pub disp: Option<DispClientContext>,
    pub cliprdr: Option<CliprdrClientContext>,
    pub encomsp: Option<EncomspClientContext>,

    rdp_context: ptr::NonNull<RdpContext<H>>,
    default_channel_connected: Option<PubSubHandle>,
    default_channel_disconnected: Option<PubSubHandle>,
}

unsafe impl<H> Send for Context<H> where H: Handler + Send {}
unsafe impl<H> Sync for Context<H> where H: Handler + Sync {}

impl<H: Handler> Drop for Context<H> {
    fn drop(&mut self) {
        unsafe {
            sys::freerdp_client_context_free(self.rdp_context.as_ptr().cast());
        }
    }
}

pub trait Handler {
    fn global_init() -> Result<()>
    where
        Self: Sized,
    {
        Ok(())
    }

    fn global_uninit()
    where
        Self: Sized,
    {
    }

    fn client_new(_instance: &FreeRdp) -> Result<()>
    where
        Self: Sized,
    {
        Ok(())
    }

    fn client_free(_instance: &FreeRdp)
    where
        Self: Sized,
    {
    }

    fn client_start(&mut self) -> std::result::Result<(), i32> {
        Ok(())
    }

    fn client_stop(&mut self) -> std::result::Result<(), i32> {
        Ok(())
    }

    fn pre_connect(&mut self, context: &mut Context<Self>) -> Result<()>
    where
        Self: Sized,
    {
        struct ChannelConnected;
        impl<'a> PubSubHandler<'a> for ChannelConnected {
            type Event = EventChannelConnected;

            fn handle<H: Handler>(
                context: &mut Context<H>,
                event: &Self::Event,
                _sender: Option<&str>,
            ) {
                match event.name.as_str() {
                    channels::rdpei::DVC_CHANNEL_NAME => {
                        let iface =
                            unsafe { RdpeiClientContext::from_ptr(event.interface as *mut _) };
                        context.rdpei = Some(iface);
                    }
                    channels::rdpgfx::DVC_CHANNEL_NAME => {
                        let iface =
                            unsafe { RdpgfxClientContext::from_ptr(event.interface as *mut _) };
                        gdi::gfx::graphics_pipeline_init(&context.gdi().unwrap(), &iface).unwrap()
                    }
                    channels::rail::SVC_CHANNEL_NAME => {}
                    channels::cliprdr::SVC_CHANNEL_NAME => {
                        let mut iface = unsafe {
                            CliprdrClientContext::from_ptr(event.interface as *mut _, true)
                        };
                        context.handler.clipboard_connected(&mut iface);
                        context.cliprdr = Some(iface);
                    }
                    channels::encomsp::SVC_CHANNEL_NAME => {
                        let mut iface = unsafe {
                            EncomspClientContext::from_ptr(event.interface as *mut _, true)
                        };
                        context.handler.encomsp_connected(&mut iface);
                        context.encomsp = Some(iface);
                    }
                    channels::disp::DVC_CHANNEL_NAME => {
                        let iface =
                            unsafe { DispClientContext::from_ptr(event.interface as *mut _) };
                        context.disp = Some(iface);
                    }
                    channels::geometry::DVC_CHANNEL_NAME => {
                        let iface =
                            unsafe { GeometryClientContext::from_ptr(event.interface as *mut _) };
                        gdi::video::geometry_init(&context.gdi().unwrap(), &iface).unwrap()
                    }
                    channels::video::CONTROL_DVC_CHANNEL_NAME => {
                        let iface =
                            unsafe { VideoClientContext::from_ptr(event.interface as *mut _) };
                        gdi::video::control_init(&context.gdi().unwrap(), &iface).unwrap()
                    }
                    channels::video::DATA_DVC_CHANNEL_NAME => {
                        let iface =
                            unsafe { VideoClientContext::from_ptr(event.interface as *mut _) };
                        gdi::video::data_init(&context.gdi().unwrap(), &iface).unwrap()
                    }
                    name => {
                        dbg!(name);
                    }
                }
            }
        }
        context.default_channel_connected = Some(context.pub_sub.subscribe::<ChannelConnected>());

        struct ChannelDisconnected;
        impl<'a> PubSubHandler<'a> for ChannelDisconnected {
            type Event = EventChannelDisconnected;

            fn handle<H: Handler>(
                context: &mut Context<H>,
                event: &Self::Event,
                _sender: Option<&str>,
            ) {
                match event.name.as_str() {
                    channels::rdpei::DVC_CHANNEL_NAME => {
                        context.rdpei = None;
                    }
                    channels::rdpgfx::DVC_CHANNEL_NAME => {
                        let iface =
                            unsafe { RdpgfxClientContext::from_ptr(event.interface as *mut _) };
                        gdi::gfx::graphics_pipeline_uninit(&context.gdi().unwrap(), &iface).unwrap()
                    }
                    channels::rail::SVC_CHANNEL_NAME => {}
                    channels::cliprdr::SVC_CHANNEL_NAME => {
                        context.cliprdr = None;
                    }
                    channels::encomsp::SVC_CHANNEL_NAME => {
                        context.encomsp = None;
                    }
                    channels::disp::DVC_CHANNEL_NAME => {
                        context.disp = None;
                    }
                    channels::geometry::DVC_CHANNEL_NAME => {
                        let iface =
                            unsafe { GeometryClientContext::from_ptr(event.interface as *mut _) };
                        gdi::video::geometry_uninit(&context.gdi().unwrap(), &iface).unwrap()
                    }
                    channels::video::CONTROL_DVC_CHANNEL_NAME => {
                        let iface =
                            unsafe { VideoClientContext::from_ptr(event.interface as *mut _) };
                        gdi::video::control_uninit(&context.gdi().unwrap(), &iface).unwrap()
                    }
                    channels::video::DATA_DVC_CHANNEL_NAME => {
                        let iface =
                            unsafe { VideoClientContext::from_ptr(event.interface as *mut _) };
                        gdi::video::data_uninit(&context.gdi().unwrap(), &iface).unwrap()
                    }
                    name => {
                        dbg!(name);
                    }
                }
            }
        }
        context.default_channel_disconnected =
            Some(context.pub_sub.subscribe::<ChannelDisconnected>());

        context.load_addins()?;
        Ok(())
    }

    fn clipboard_connected(&mut self, _clip: &mut CliprdrClientContext)
    where
        Self: Sized,
    {
    }

    fn encomsp_connected(&mut self, _encomsp: &mut EncomspClientContext)
    where
        Self: Sized,
    {
    }

    fn authenticate(&mut self, _context: &mut Context<Self>) -> Result<()>
    where
        Self: Sized,
    {
        Err(RdpError::Unsupported)
    }

    fn post_connect(&mut self, context: &mut Context<Self>) -> Result<()>
    where
        Self: Sized;

    fn post_disconnect(&mut self, context: &mut Context<Self>)
    where
        Self: Sized,
    {
        context.instance.gdi_uninit();
    }

    #[allow(clippy::too_many_arguments)]
    fn verify_certificate(
        &mut self,
        _host: &str,
        _port: u16,
        _common_name: Option<&str>,
        _subject: &str,
        _issuer: &str,
        _fingerprint: &str,
        _flags: u32,
    ) -> VerifyCertificateResult {
        VerifyCertificateResult::AcceptOnlyThisSession
    }

    #[allow(clippy::too_many_arguments)]
    fn verify_certificate_changed(
        &mut self,
        _host: &str,
        _port: u16,
        _common_name: Option<&str>,
        _subject: &str,
        _issuer: &str,
        _new_fingerprint: &str,
        _old_subject: &str,
        _old_issuer: &str,
        _old_fingerprint: &str,
        _flags: u32,
    ) -> VerifyCertificateResult {
        VerifyCertificateResult::AcceptOnlyThisSession
    }

    fn present_gateway_message(
        &mut self,
        _type: u32,
        _is_display_mandatory: bool,
        _is_consent_mandatory: bool,
        msg: &str,
    ) -> Result<()> {
        eprintln!("{}", msg);
        Ok(())
    }

    fn logon_error_info(&mut self, _data: u32, _type: u32) -> i32 {
        1
    }
}

fn cvt_nz(error: u32) -> Result<()> {
    if error == 0 {
        Ok(())
    } else {
        Err(RdpError::Code(RdpCode(error)))
    }
}

impl<H: Handler> Context<H> {
    pub fn client_start(&mut self) -> Result<()> {
        cvt_nz(unsafe { sys::freerdp_client_start(self.rdp_context.as_ptr().cast()) } as _)
    }

    pub fn client_stop(&mut self) -> Result<()> {
        cvt_nz(unsafe { sys::freerdp_client_stop(self.rdp_context.as_ptr().cast()) } as _)
    }

    pub fn event_handles(&self) -> Result<Vec<Handle>> {
        let mut handles: [MaybeUninit<sys::HANDLE>; winpr::MAX_WAIT_OBJECTS] =
            [MaybeUninit::uninit(); winpr::MAX_WAIT_OBJECTS];
        let res = unsafe {
            sys::freerdp_get_event_handles(
                self.rdp_context.as_ptr().cast(),
                handles.as_mut_ptr() as _,
                handles.len() as _,
            )
        };
        match res {
            0 => Err(RdpError::Failed(
                "freerdp_get_event_handles() failed".into(),
            )),
            _ => Ok(handles[0..(res as _)]
                .iter()
                .map(|h| Handle::new(unsafe { h.assume_init() }, false))
                .collect()),
        }
    }

    pub fn check_event_handles(&mut self) -> bool {
        unsafe { sys::freerdp_check_event_handles(self.rdp_context.as_ptr().cast()) > 0 }
    }

    pub fn last_error(&self) -> Option<crate::RdpErr> {
        match cvt_nz(unsafe { sys::freerdp_get_last_error(self.rdp_context.as_ptr().cast()) as _ })
        {
            Err(RdpError::Code(code)) => code.as_err(),
            _ => None,
        }
    }

    pub fn input(&self) -> Option<Input> {
        let input = unsafe { self.rdp_context.as_ref() }.rdp_context.input;
        if input.is_null() {
            None
        } else {
            Some(Input::new(input))
        }
    }

    pub fn gdi(&self) -> Option<Gdi> {
        let gdi = unsafe { self.rdp_context.as_ref() }.rdp_context.gdi;
        if gdi.is_null() {
            None
        } else {
            Some(Gdi::new(gdi))
        }
    }

    pub fn graphics(&self) -> Option<Graphics> {
        let graphics = unsafe { self.rdp_context.as_ref() }.rdp_context.graphics;
        if graphics.is_null() {
            None
        } else {
            Some(Graphics::new(graphics))
        }
    }

    pub fn update(&self) -> Option<Update> {
        let update = unsafe { self.rdp_context.as_ref() }.rdp_context.update;
        if update.is_null() {
            None
        } else {
            Some(Update::new(update))
        }
    }

    fn load_addins(&mut self) -> Result<()> {
        unsafe {
            if sys::freerdp_client_load_addins(
                self.rdp_context.as_ref().rdp_context.channels,
                self.rdp_context.as_ref().rdp_context.settings,
            ) != 0
            {
                Ok(())
            } else {
                Err(RdpError::Failed("Failed to load client addins".into()))
            }
        }
    }
}

impl<H: Handler> Context<H> {
    pub(crate) fn from_ptr<'a>(ptr: *mut sys::rdpContext) -> &'a mut Self {
        RdpContext::<H>::from_ptr(ptr).context()
    }

    pub fn new(handler: H) -> Box<Self> {
        let mut entry_points = sys::rdp_client_entry_points_v1 {
            Size: size_of::<sys::rdp_client_entry_points_v1>() as _,
            Version: sys::RDP_CLIENT_INTERFACE_VERSION,
            settings: std::ptr::null_mut(),
            GlobalInit: Some(rdp_global_init::<H>),
            GlobalUninit: Some(rdp_global_uninit::<H>),
            ClientNew: Some(rdp_client_new::<H>),
            ClientFree: Some(rdp_client_free::<H>),
            ClientStart: Some(rdp_client_start::<H>),
            ClientStop: Some(rdp_client_stop::<H>),
            ContextSize: size_of::<RdpContext<H>>() as _,
        };

        let ptr = unsafe { sys::freerdp_client_context_new(&mut entry_points) };
        let rdp_context = RdpContext::<H>::from_ptr(ptr).rdp_context;
        let settings = Settings::new(false, rdp_context.settings);
        let instance = FreeRdp::new(rdp_context.instance);
        let pub_sub = PubSub::new(rdp_context.pubSub);

        let res = Box::new(Self {
            settings,
            instance,
            handler,
            pub_sub,
            rdp_context: ptr::NonNull::new(ptr as *mut _).unwrap(),
            rdpei: None,
            disp: None,
            cliprdr: None,
            encomsp: None,
            default_channel_connected: None,
            default_channel_disconnected: None,
        });
        RdpContext::<H>::from_ptr(ptr).context =
            ptr::NonNull::new(&*res as *const _ as *mut _).unwrap();
        res
    }
}

extern "C" fn rdp_global_init<H: Handler>() -> sys::BOOL {
    H::global_init().is_ok() as _
}

extern "C" fn rdp_global_uninit<H: Handler>() {
    H::global_uninit()
}

extern "C" fn rdp_instance_pre_connect<H: Handler>(instance: *mut sys::freerdp) -> sys::BOOL {
    let ptr = unsafe { (*instance).context };
    // XXX: alias context, ok as long as handler isn't changed...
    // similar below and elsewhere
    let ctxt = Context::<H>::from_ptr(ptr);

    ctxt.handler
        .pre_connect(Context::<H>::from_ptr(ptr))
        .is_ok() as _
}

extern "C" fn rdp_instance_post_connect<H: Handler>(instance: *mut sys::freerdp) -> sys::BOOL {
    let ptr = unsafe { (*instance).context };
    let ctxt = Context::<H>::from_ptr(ptr);

    ctxt.handler
        .post_connect(Context::<H>::from_ptr(ptr))
        .is_ok() as _
}

extern "C" fn rdp_instance_post_disconnect<H: Handler>(instance: *mut sys::freerdp) {
    let ptr = unsafe { (*instance).context };
    let ctxt = Context::<H>::from_ptr(ptr);

    ctxt.handler.post_disconnect(Context::<H>::from_ptr(ptr))
}

extern "C" fn rdp_instance_authenticate<H: Handler>(
    instance: *mut sys::freerdp,
    _username: *mut *mut c_char,
    _password: *mut *mut c_char,
    _domain: *mut *mut c_char,
) -> sys::BOOL {
    let ptr = unsafe { (*instance).context };
    let ctxt = Context::<H>::from_ptr(ptr);

    ctxt.handler
        .authenticate(Context::<H>::from_ptr(ptr))
        .is_ok() as _
}

extern "C" fn rdp_instance_verify_certificate<H: Handler>(
    instance: *mut sys::freerdp,
    host: *const ::std::os::raw::c_char,
    port: sys::UINT16,
    common_name: *const ::std::os::raw::c_char,
    subject: *const ::std::os::raw::c_char,
    issuer: *const ::std::os::raw::c_char,
    fingerprint: *const ::std::os::raw::c_char,
    flags: sys::DWORD,
) -> sys::DWORD {
    let ctxt = Context::<H>::from_ptr(unsafe { (*instance).context });
    let common_name = if common_name.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(common_name).to_str().ok() }
    };

    ctxt.handler
        .verify_certificate(
            unsafe { CStr::from_ptr(host).to_str().unwrap() },
            port,
            common_name,
            unsafe { CStr::from_ptr(subject).to_str().unwrap() },
            unsafe { CStr::from_ptr(issuer).to_str().unwrap() },
            unsafe { CStr::from_ptr(fingerprint).to_str().unwrap() },
            flags,
        )
        .into()
}

extern "C" fn rdp_instance_verify_changed_certificate<H: Handler>(
    instance: *mut sys::freerdp,
    host: *const ::std::os::raw::c_char,
    port: sys::UINT16,
    common_name: *const ::std::os::raw::c_char,
    subject: *const ::std::os::raw::c_char,
    issuer: *const ::std::os::raw::c_char,
    new_fingerprint: *const ::std::os::raw::c_char,
    old_subject: *const ::std::os::raw::c_char,
    old_issuer: *const ::std::os::raw::c_char,
    old_fingerprint: *const ::std::os::raw::c_char,
    flags: sys::DWORD,
) -> sys::DWORD {
    let ctxt = Context::<H>::from_ptr(unsafe { (*instance).context });
    let common_name = if common_name.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(common_name).to_str().ok() }
    };

    ctxt.handler
        .verify_certificate_changed(
            unsafe { CStr::from_ptr(host).to_str().unwrap() },
            port,
            common_name,
            unsafe { CStr::from_ptr(subject).to_str().unwrap() },
            unsafe { CStr::from_ptr(issuer).to_str().unwrap() },
            unsafe { CStr::from_ptr(new_fingerprint).to_str().unwrap() },
            unsafe { CStr::from_ptr(old_subject).to_str().unwrap() },
            unsafe { CStr::from_ptr(old_issuer).to_str().unwrap() },
            unsafe { CStr::from_ptr(old_fingerprint).to_str().unwrap() },
            flags,
        )
        .into()
}

extern "C" fn rdp_instance_present_gateway_message<H: Handler>(
    instance: *mut sys::freerdp,
    type_: sys::UINT32,
    is_display_mandatory: sys::BOOL,
    is_consent_mandatory: sys::BOOL,
    length: sys::size_t,
    message: *const sys::WCHAR,
) -> sys::BOOL {
    let ctxt = Context::<H>::from_ptr(unsafe { (*instance).context });
    let msg = String::from_utf16_lossy(unsafe { std::slice::from_raw_parts(message, length as _) });

    ctxt.handler
        .present_gateway_message(
            type_,
            is_display_mandatory != 0,
            is_consent_mandatory != 0,
            &msg,
        )
        .is_ok() as _
}

extern "C" fn rdp_instance_logon_error_info<H: Handler>(
    instance: *mut sys::freerdp,
    data: sys::UINT32,
    type_: sys::UINT32,
) -> i32 {
    let ctxt = Context::<H>::from_ptr(unsafe { (*instance).context });

    ctxt.handler.logon_error_info(data, type_)
}

extern "C" fn rdp_client_new<H: Handler>(
    instance: *mut sys::freerdp,
    _context: *mut sys::rdpContext,
) -> sys::BOOL {
    unsafe {
        let mut instance = ptr::NonNull::new(instance).unwrap().as_mut();
        instance.PreConnect = Some(rdp_instance_pre_connect::<H>);
        instance.PostConnect = Some(rdp_instance_post_connect::<H>);
        instance.PostDisconnect = Some(rdp_instance_post_disconnect::<H>);
        instance.Authenticate = Some(rdp_instance_authenticate::<H>);
        instance.VerifyCertificateEx = Some(rdp_instance_verify_certificate::<H>);
        instance.VerifyChangedCertificateEx = Some(rdp_instance_verify_changed_certificate::<H>);
        instance.PresentGatewayMessage = Some(rdp_instance_present_gateway_message::<H>);
        instance.LogonErrorInfo = Some(rdp_instance_logon_error_info::<H>);
    }

    // can't call self.client_new() since it isn't yet returned from context_new...
    H::client_new(&FreeRdp::new(instance)).is_ok() as _
}

extern "C" fn rdp_client_free<H: Handler>(
    instance: *mut sys::freerdp,
    _context: *mut sys::rdpContext,
) {
    // can't call self.client_free() since it may not yet be returned from context_new...
    H::client_free(&FreeRdp::new(instance))
}

extern "C" fn rdp_client_start<H: Handler>(context: *mut sys::rdpContext) -> c_int {
    let ctxt = Context::<H>::from_ptr(context);

    match ctxt.handler.client_start() {
        Ok(_) => 0,
        Err(e) => e,
    }
}

extern "C" fn rdp_client_stop<H: Handler>(context: *mut sys::rdpContext) -> c_int {
    let ctxt = Context::<H>::from_ptr(context);

    match ctxt.handler.client_stop() {
        Ok(_) => 0,
        Err(e) => e,
    }
}

pub enum VerifyCertificateResult {
    AcceptAndStore,
    AcceptOnlyThisSession,
    Fail,
}

impl From<VerifyCertificateResult> for u32 {
    fn from(res: VerifyCertificateResult) -> Self {
        match res {
            VerifyCertificateResult::AcceptAndStore => 1,
            VerifyCertificateResult::AcceptOnlyThisSession => 2,
            VerifyCertificateResult::Fail => 0,
        }
    }
}
