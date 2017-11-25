use glium::*;

use gfx::gfx_api::GfxApi;

pub struct VkApi
{

}

impl VkApi {
    pub fn new() -> Self {
        unimplemented!()
    }
}

impl GfxApi for VkApi {

    fn get_ver(&self) -> Vec<String> {
        unimplemented!()
    }
    fn open_window(&self, _width: u32, _height: u32, _title: String) -> Result<u64, String> {
        unimplemented!()
    }
}