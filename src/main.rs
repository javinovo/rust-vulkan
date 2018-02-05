#[macro_use]
extern crate vulkano_shader_derive;

extern crate vulkano;
use vulkano as vk;

mod lib;
use lib::*;

mod compute;
mod mandelbrot;

fn main() {
    
    use vk::instance::{ Instance, InstanceExtensions, PhysicalDevice };

    // Creating an instance tries to load Vulkan from the system and reads the list of available implementations.
    let instance = Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");
    
    // The machine you run your program on may have multiple devices that support Vulkan. 
    // In reality a physical device can be a dedicated graphics card, but also an integrated graphics processor or a software implementation. 
    // It can be basically anything that allows running Vulkan operations.
    // Keep in mind that the list of physical devices can be empty. This happens if Vulkan is installed on the system, but none of the physical devices of the machine are capable of supporting Vulkan. 
    for dev in PhysicalDevice::enumerate(&instance) {
        println!("Physical device: {}", dev.name());
    }

    //  A device is an object that represents an open channel of communication with a physical device, and it is probably the most important object of the Vulkan API.
    let physical = PhysicalDevice::enumerate(&instance).next().expect("no device available");

    // The Vulkan equivalent of a CPU thread is a queue. Queues are grouped by queue families.
    for family in physical.queue_families() {
        println!("Found a queue family with {:?} queue(s)", family.queues_count());
    }

    // Some queues support only graphical operations, some others support only compute operations, and some others support both.
    let queue_family = physical.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");
    

    use vk::device::{ Device, DeviceExtensions };
    use vk::instance::Features;

    // In order to create a device, we have to tell the Vulkan implementation which queues we want to use. 
    // Creating a device returns two things: the device itself, but also a list of queue objects that will later allow us to submit operations.
    let (device, mut queues) = {
        Device::new(physical, &Features::none(), &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
    };

    // The queues variable returned by the function is in fact an iterator. In this example code this iterator contains just one element, so let's extract it:
    let queue = queues.next().unwrap();


    test_buffer_copy(&device, &queue);

    create_image(&device, &queue, [0.0, 0.0, 1.0, 1.0], "blue.png");

    compute::run(&device, &queue);
    mandelbrot::run(&device, &queue, "fractal.png");
}

