use std::{
    borrow::Borrow,
    ffi::{CStr, CString},
    marker::PhantomData,
    mem::size_of,
    os::raw::{c_char, c_void},
    ptr,
};

use crate::{
    client::{Context, Handler, RdpContext},
    sys,
};

#[derive(Debug)]
pub struct PubSub<'a, C: Handler> {
    inner: ptr::NonNull<sys::wPubSub>,
    handler: PhantomData<C>,
    _lifetime: PhantomData<&'a ()>,
}

unsafe impl<C> Send for PubSub<'_, C> where C: Handler + Send {}
unsafe impl<C> Sync for PubSub<'_, C> where C: Handler + Sync {}

pub trait PubSubEvent<'a>: From<&'a sys::wEventArgs> + std::fmt::Debug {
    const NAME: &'static str;
    type CType;
}

pub trait PubSubHandler<'a> {
    type Event: PubSubEvent<'a>;

    fn handle<H: Handler>(context: &mut Context<H>, event: &Self::Event, sender: Option<&str>);
}

#[derive(Debug)]
pub struct EventChannelConnected {
    pub name: String,
    pub interface: *const c_void,
}

#[doc(hidden)]
#[derive(Debug)]
#[repr(C)]
pub struct CEventChannelConnected {
    event: sys::wEventArgs,
    name: *const c_char,
    interface: *const c_void,
}

impl From<&sys::wEventArgs> for EventChannelConnected {
    fn from(args: &sys::wEventArgs) -> Self {
        unsafe {
            let args = ptr::NonNull::new(args as *const _ as *mut CEventChannelConnected).unwrap();
            let cname = CStr::from_ptr(args.as_ref().name);
            Self {
                name: cname.to_string_lossy().into_owned(),
                interface: args.as_ref().interface,
            }
        }
    }
}

impl<'a> PubSubEvent<'a> for EventChannelConnected {
    const NAME: &'static str = "ChannelConnected";
    type CType = CEventChannelConnected;
}

#[derive(Debug)]
pub struct EventChannelDisconnected {
    pub name: String,
    pub interface: *const c_void,
}

#[doc(hidden)]
#[derive(Debug)]
#[repr(C)]
pub struct CEventChannelDisconnected {
    event: sys::wEventArgs,
    name: *const c_char,
    interface: *const c_void,
}

impl From<&sys::wEventArgs> for EventChannelDisconnected {
    fn from(args: &sys::wEventArgs) -> Self {
        unsafe {
            let args =
                ptr::NonNull::new(args as *const _ as *mut CEventChannelDisconnected).unwrap();
            let cname = CStr::from_ptr(args.as_ref().name);
            Self {
                name: cname.to_string_lossy().into_owned(),
                interface: args.as_ref().interface,
            }
        }
    }
}

impl<'a> PubSubEvent<'a> for EventChannelDisconnected {
    const NAME: &'static str = "ChannelDisconnected";
    type CType = CEventChannelDisconnected;
}

#[derive(Debug)]
pub struct PubSubHandle {
    pub_sub: *mut sys::wPubSub,
    name: CString,
    handler: sys::pEventHandler,
}

unsafe impl Send for PubSubHandle {}
unsafe impl Sync for PubSubHandle {}

impl PubSubHandle {
    fn new(pub_sub: *mut sys::wPubSub, name: CString, handler: sys::pEventHandler) -> Self {
        Self {
            pub_sub,
            name,
            handler,
        }
    }
}

impl Drop for PubSubHandle {
    fn drop(&mut self) {
        unsafe {
            sys::PubSub_Unsubscribe(self.pub_sub, self.name.as_ptr(), self.handler);
        }
    }
}

impl<'a, C: Handler> PubSub<'a, C> {
    pub(crate) fn new(pubsub: *mut sys::wPubSub) -> Self {
        Self {
            inner: ptr::NonNull::new(pubsub).unwrap(),
            handler: PhantomData,
            _lifetime: PhantomData,
        }
    }

    pub fn subscribe<H>(&mut self) -> PubSubHandle
    where
        H: for<'h> PubSubHandler<'h>,
    {
        unsafe extern "C" fn handler<H, C: Handler>(
            context: *mut ::std::os::raw::c_void,
            e: *mut sys::wEventArgs,
        ) where
            H: for<'h> PubSubHandler<'h>,
        {
            let e = ptr::NonNull::new(e).unwrap();
            assert_eq!(
                e.as_ref().Size as usize,
                size_of::<<<H as PubSubHandler<'_>>::Event as PubSubEvent>::CType>()
            );
            let event = e.as_ref().into();
            let csender = if e.as_ref().Sender.is_null() {
                None
            } else {
                let sender = CStr::from_ptr(e.as_ref().Sender);
                Some(sender)
            };
            let sender_str = csender.map(|s| s.to_string_lossy());
            let sender = sender_str.as_ref().map(|s| s.borrow());

            let ctxt = ptr::NonNull::new(context as *mut RdpContext<C>).unwrap();
            H::handle(&mut Context::from_context(false, ctxt), &event, sender);
        }

        let cname = CString::new(H::Event::NAME).unwrap();
        unsafe {
            sys::PubSub_Subscribe(self.inner.as_ptr(), cname.as_ptr(), Some(handler::<H, C>));
        }

        PubSubHandle::new(self.inner.as_ptr(), cname, Some(handler::<H, C>))
    }
}
