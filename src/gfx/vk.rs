use glium::*;

use gfx::gfx_api::GfxApi;

pub struct VkApi
{
    events_loop: glutin::EventsLoop,
}

impl VkApi {
    pub fn new() -> Self {

        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new();
        let context = glutin::ContextBuilder::new();
        let display = Display::new(window, context, &events_loop).unwrap();

        return VkApi {
            events_loop,
        }
    }
}

impl GfxApi for VkApi {

    fn get_ver(self) -> String {
        return String::from("Vulkan v0.1.0");
    }
    fn open_window(self, width: u32, height: u32, title: String) -> Result<u64, String> {
        unimplemented!()
    }
}