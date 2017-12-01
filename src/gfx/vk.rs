extern crate vulkano;
extern crate winit;

extern crate vulkano_win;

use self::vulkano::instance::*;
use self::vulkano_win::VkSurfaceBuild;

use std::cell::Cell;
use std::sync::Arc;

use gfx::gfx_api::GfxApi;

struct ApiInfo {
    version: Version,
    name: String,
    type_: PhysicalDeviceType,
}

pub struct VkApi
{
    rid: Cell<u64>, //Resource ID counter, is used to access resources this backend generates
    // This is the physical device we are actually using. Multigpu would be done here I guess
    info: ApiInfo,
    // Event loop handles input and other stuff like resizing
    events: winit::EventsLoop,
    instance: Arc<Instance>,
}

impl VkApi {
    pub fn new() -> Self {
        let instance = {
            let extensions = vulkano_win::required_extensions();

            Instance::new(None, &extensions, None).expect("Vulkan Error: Instance creation failed")
        };

        let physical = PhysicalDevice::enumerate(&instance)
            .next().expect("Vulkan Error: No device available");

        VkApi {
            rid: Cell::new(0u64),
            info: ApiInfo {
                version: physical.api_version(),
                name: physical.name(),
                type_: physical.ty(),
            },
            events: winit::EventsLoop::new(),
            instance
        }
    }

    fn rid(&self) -> u64 {
        let mut value = self.rid.get();
        value += 1;
        self.rid.set(value);
        return value;
    }
}

impl GfxApi for VkApi {

    fn get_ver(&self) -> Vec<String> {
        vec![
            format!("Backend: Vulkan"),
            format!("Version: {}", self.info.version),
            format!("Device:  {}: {}", match self.info.type_ {
                PhysicalDeviceType::IntegratedGpu => { "Integrated" },
                PhysicalDeviceType::DiscreteGpu => { "Dedicated" },
                PhysicalDeviceType::VirtualGpu => { "Virtual" },
                PhysicalDeviceType::Cpu => { "CPU" },
                PhysicalDeviceType::Other => { "Unknown" },
            }, self.info.name),
        ]
    }
    fn open_window(&self, _width: u32, _height: u32, _title: String) -> Result<u64, String> {
        let window = winit::WindowBuilder::new().build_vk_surface(&self.events, self.instance.clone()).unwrap();

        Ok(self.rid())
    }

    fn poll_event(&mut self) {

    }
}