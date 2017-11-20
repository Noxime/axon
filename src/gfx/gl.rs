use glium::*;

use gfx::gfx_api::GfxApi;

pub struct GlApi
{
    events_loop: glutin::EventsLoop,
}

impl GlApi {
    pub fn new() -> Self {

        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new();
        let context = glutin::ContextBuilder::new();
        let display = Display::new(window, context, &events_loop).unwrap();

        return GlApi {
            events_loop,
        }
    }
}

impl GfxApi for GlApi {

    fn get_ver(self) -> String {
        return String::from("OpenGL v0.1.0");
    }
    fn open_window(self, width: u32, height: u32, title: String) -> Result<u64, String> {
        unimplemented!()
    }
}