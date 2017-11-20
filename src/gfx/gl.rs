use gfx::gfx_api::GfxApi;

pub struct GlApi
{

}

impl GfxApi for GlApi {
    fn new() -> GfxApi {
        return GlApi {

        };
    }

    fn get_ver() -> String {
        return String::from("OpenGL backend v0.1.0");
    }
}