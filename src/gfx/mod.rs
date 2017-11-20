pub use self::gl::GlApi;
mod gl;

pub mod gfx_api
{
    pub trait GfxApi
    {
        fn new() -> Self;
        fn get_ver() -> String;
    }
}