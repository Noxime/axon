pub use self::gl::GlApi;
pub use self::vk::VkApi;

mod gl;
mod vk;

pub mod gfx_api
{
    pub fn get_api() -> impl GfxApi {
        return super::VkApi::new();
        //return super::GlApi::new();
    }

    pub trait GfxApi
    {
        fn get_ver(self) -> String;
        fn open_window(self, width: u32, height: u32, title: String) -> Result<u64, String>;
    }
}