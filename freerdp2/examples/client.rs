use freerdp2::{
    client::*, graphics::*, locale::keyboard_init_ex, update::*, winpr::wait_for_multiple_objects,
    RdpError, Result, PIXEL_FORMAT_BGRA32,
};

#[derive(Debug)]
struct MyPointerHandler {
    _test: bool,
}

impl PointerHandler for MyPointerHandler {
    type ContextHandler = MyContextHandler;

    fn new(
        &mut self,
        context: &mut Context<Self::ContextHandler>,
        pointer: &Pointer,
    ) -> Result<()> {
        dbg!(self);
        dbg!(pointer);
        let h = context.handler_mut();
        dbg!(h);
        Ok(())
    }
}

#[derive(Debug)]
struct MyUpdateHandler;

impl UpdateHandler for MyUpdateHandler {
    type ContextHandler = MyContextHandler;

    fn begin_paint(context: &mut Context<Self::ContextHandler>) -> Result<()> {
        let gdi = context.gdi().ok_or(RdpError::Unsupported)?;
        let mut primary = gdi.primary().ok_or(RdpError::Unsupported)?;
        primary.hdc().hwnd().invalid().set_null(true);
        Ok(())
    }

    fn end_paint(context: &mut Context<Self::ContextHandler>) -> Result<()> {
        let gdi = context.gdi().ok_or(RdpError::Unsupported)?;
        let mut primary = gdi.primary().ok_or(RdpError::Unsupported)?;
        let invalid = primary.hdc().hwnd().invalid();
        if invalid.null() {
            return Ok(());
        }
        let (x, y, w, h) = (invalid.x(), invalid.y(), invalid.w(), invalid.h());

        let handler = context.handler_mut().unwrap();
        handler.update_buffer(x, y, w, h)
    }

    fn set_bounds(_context: &mut Context<Self::ContextHandler>, bounds: &Bounds) -> Result<()> {
        dbg!(bounds);
        Ok(())
    }

    fn synchronize(_context: &mut Context<Self::ContextHandler>) -> Result<()> {
        dbg!();
        Ok(())
    }

    fn desktop_resize(context: &mut Context<Self::ContextHandler>) -> Result<()> {
        let mut gdi = context.gdi().ok_or(RdpError::Unsupported)?;
        gdi.resize(
            context.settings.desktop_width(),
            context.settings.desktop_height(),
        )?;
        Ok(())
    }
}

#[derive(Debug)]
struct MyContextHandler {
    _test: u32,
}

impl MyContextHandler {
    fn update_buffer(&mut self, x: i32, y: i32, w: i32, h: i32) -> Result<()> {
        let x = u32::try_from(x)?;
        let y = u32::try_from(y)?;
        let w = u32::try_from(w)?;
        let h = u32::try_from(h)?;
        dbg!((x, y, w, h));
        Ok(())
    }
}

impl Handler for MyContextHandler {
    fn post_connect(&mut self, context: &mut Context<Self>) -> Result<()> {
        context.instance.gdi_init(PIXEL_FORMAT_BGRA32)?;

        let gdi = context.gdi().unwrap();
        let mut graphics = context.graphics().unwrap();
        let mut update = context.update().unwrap();

        let (w, h) = match (gdi.width(), gdi.height()) {
            (Some(w), Some(h)) => (w, h),
            _ => return Err(RdpError::Failed("No GDI dimensions".into())),
        };
        dbg!((w, h));

        graphics.register_pointer::<MyPointerHandler>();
        update.register::<MyUpdateHandler>();

        let _ = keyboard_init_ex(
            context.settings.keyboard_layout(),
            context.settings.keyboard_remapping_list().as_deref(),
        );

        Ok(())
    }
}

fn main() {
    let mut ctxt = Context::new(MyContextHandler { _test: 42 });

    ctxt.client_start().unwrap();
    ctxt.settings
        .set_server_hostname(Some("localhost"))
        .unwrap();
    ctxt.settings.set_server_port(3389);
    ctxt.settings.set_username(Some("user")).unwrap();
    ctxt.settings.set_password(Some("pass")).unwrap();

    let args: Vec<_> = std::env::args().collect();
    let args: Vec<_> = args.iter().map(|s| s.as_str()).collect();
    ctxt.settings.parse_command_line(&args, true).unwrap();

    ctxt.instance.connect().unwrap();

    while !ctxt.instance.shall_disconnect() {
        let handles = ctxt.event_handles().unwrap();
        let handles: Vec<_> = handles.iter().collect();
        wait_for_multiple_objects(&handles, false, None).unwrap();

        if !ctxt.check_event_handles() {
            if let Err(e) = ctxt.last_error() {
                eprintln!("{}", e);
                break;
            }
        }
    }

    ctxt.client_stop().unwrap();
}
