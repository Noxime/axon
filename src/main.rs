//Disabled because OSX
//extern crate vulkano;

use gfx::gfx_api;
use gfx::gfx_api::GfxApi;

mod gfx;

fn main() {
    //Initialize different parts of the engine
    let mut api = gfx_api::get_api();

    //Print some engine info
    // The `.fold`s are for printing info in a nice, indented manner
    println!("Renderer info");
    api.get_ver().iter().fold((),|_, v| {
        println!("\t{}", v);
    });

    match api.open_window(800, 600, "Axon".to_string()) {
        Ok(_) => {},
        Err(why) => {
            eprintln!("Failed to open a window: {}", why);
            return;
        }
    };

    loop {
        api.poll_event();
    }
}
