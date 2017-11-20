use gfx::gfx_api::GfxApi;

pub struct GlApi
{

}

impl GlApi {
    pub fn new() -> Self {
        return GlApi {

        }
    }
}

impl GfxApi for GlApi {

    fn get_ver(self) -> String {
        return String::from("OpenGL backend v0.1.0");
    }
}