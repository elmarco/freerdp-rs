mod cliprdr;
pub use cliprdr::*;

mod context;
pub use context::*;

mod disp;
pub use disp::*;

mod encomsp;
pub use encomsp::*;

mod pub_sub;
pub use pub_sub::*;

mod rdpei;
pub use rdpei::*;

mod rdpgfx;
pub use rdpgfx::*;

mod video;
pub use video::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FreeRdp, Result};

    #[test]
    fn it_works() {
        #[derive(Debug)]
        struct MyHandler {}

        impl Handler for MyHandler {
            fn global_init() -> Result<()> {
                dbg!();
                Ok(())
            }

            fn global_uninit() {
                dbg!();
            }

            fn client_new(_instance: &FreeRdp) -> Result<()> {
                dbg!();
                Ok(())
            }

            fn client_free(_instance: &FreeRdp) {
                dbg!();
            }

            fn client_start(&mut self) -> std::result::Result<(), i32> {
                Ok(())
            }

            fn client_stop(&mut self) -> std::result::Result<(), i32> {
                Ok(())
            }

            fn post_connect(&mut self, _context: &mut Context<Self>) -> Result<()> {
                Ok(())
            }
        }

        let mut ctxt = Context::new(MyHandler {});
        dbg!(&ctxt);
        ctxt.client_start().unwrap();
        ctxt.client_stop().unwrap();
    }
}
