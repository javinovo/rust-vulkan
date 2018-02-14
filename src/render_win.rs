extern crate winit;
extern crate vulkano_win;
extern crate vulkano;

use vulkano::instance::Instance;
use self::vulkano_win::VkSurfaceBuild;
use self::winit::EventsLoop;
use self::winit::WindowBuilder;

pub fn run() {

    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
    };

    let mut events_loop = EventsLoop::new();
    let window = winit::WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();

    events_loop.run_forever(|event| {
        match event {
            winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => {
                winit::ControlFlow::Break
            },
            _ => winit::ControlFlow::Continue,
        }
    });
}