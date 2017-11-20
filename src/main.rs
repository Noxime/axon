extern crate glium;
extern crate vulkano;

use gfx::*;
use gfx::gfx_api::*;

mod gfx;

fn pick_backend() -> gfx_api::GfxApi
{
    return GlApi::new();
}

fn main() {
    let api : gfx_api::GfxApi = pick_backend();
}
