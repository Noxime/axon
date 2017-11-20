pub use self::gl::GlApi;

mod gl;

pub mod gfx_api
{
    pub fn get_api() -> impl GfxApi {
        return super::GlApi::new();
    }

    pub trait GfxApi
    {
        fn get_ver(self) -> String;
    }
}