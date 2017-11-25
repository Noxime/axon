pub use self::gl::GlApi;
pub use self::vk::VkApi;

mod gl;
mod vk;

pub mod gfx_api
{
    pub fn get_api() -> Box<GfxApi> {

        //TODO: Detect vk support
        let vk_supported = false;

        if vk_supported {
            return Box::new(super::VkApi::new());
        } else {
            return Box::new(super::GlApi::new());
        }

    }

    pub trait GfxApi
    {
        fn get_ver(&self) -> Vec<String>;
        fn open_window(&self, width: u32, height: u32, title: String) -> Result<u64, String>;
    }
}