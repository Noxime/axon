use glium::*;


use gfx::gfx_api::GfxApi;

use std::cell::Cell;

pub struct GlApi
{
    rid: Cell<u64>, //Resource ID counter, is used to access resources this backend generates
    events_loop: glutin::EventsLoop, //This is how we wait for events
    display: Display, //Our OpenGL context
}

impl GlApi {

    fn rid(&self) -> u64 {
        let mut value = self.rid.get();
        value += 1;
        self.rid.set(value);
        return value;
    }

    pub fn new() -> Self {

        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_visibility(false);
        let context = glutin::ContextBuilder::new();
        let display = Display::new(window, context, &events_loop).unwrap();

        return GlApi {
            rid: Cell::new(0u64),
            events_loop: events_loop,
            display: display,
        }
    }
}

impl GfxApi for GlApi {

    fn get_ver(&self) -> Vec<String> {
        vec![
            format!("Backend:  {}", match self.display.get_opengl_version() {
                &Version(Api::Gl, _, _) => "OpenGL",
                &Version(Api::GlEs, _, _) => "OpenGL ES",
            }),
            format!("Version:  {}", self.display.get_opengl_version_string()),
            format!("Renderer: {}", self.display.get_opengl_renderer_string()),
            format!("Vendor:   {}", self.display.get_opengl_vendor_string()),
        ]
    }

    fn open_window(&self, width: u32, height: u32, title: String) -> Result<u64, String> {

        self.display.gl_window().set_inner_size(width, height);
        self.display.gl_window().set_title(title.as_str());
        self.display.gl_window().show();

        Ok(self.rid())
    }
}