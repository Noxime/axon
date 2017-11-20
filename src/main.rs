#![feature(conservative_impl_trait)]

extern crate glium;
//Disabled because OSX
//extern crate vulkano;

use gfx::gfx_api;
use gfx::gfx_api::GfxApi;

mod gfx;

fn main() {
    let api = gfx_api::get_api();
    println!("Using renderer: {}", api.get_ver())
}
