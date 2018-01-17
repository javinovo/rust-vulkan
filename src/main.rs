
extern crate vulkano;

use vulkano as vk;

fn main() {
    use vk::instance::{ Instance, InstanceExtensions, PhysicalDevice };

    let instance = Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");
    
    for dev in PhysicalDevice::enumerate(&instance) {
        println!("Physical device: {}", dev.name());
    }

    let physical = PhysicalDevice::enumerate(&instance).next().expect("no device available");

    for family in physical.queue_families() {
        println!("Found a queue family with {:?} queue(s)", family.queues_count());
    }

    let queue_family = physical.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");
    
    use vk::device::{ Device, DeviceExtensions };
    use vk::instance::Features;

    let (device, mut queues) = {
        Device::new(physical, &Features::none(), &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
    };

    let queue = queues.next().unwrap();

    use vk::buffer::{ BufferUsage, CpuAccessibleBuffer };

    let source_content = 0 .. 64;
    let source = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), source_content).expect("failed to create buffer");

    let dest_content = (0 .. 64).map(|_| 0);
    let dest = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), dest_content).expect("failed to create buffer");

    use vk::command_buffer::{ AutoCommandBufferBuilder, CommandBuffer };

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .copy_buffer(source.clone(), dest.clone()).unwrap()
        .build().unwrap();
    
    let finished = command_buffer.execute(queue.clone()).unwrap();
        
    use vk::sync::GpuFuture;

    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();
        
    let src_content = source.read().unwrap();
    let dest_content = dest.read().unwrap();
    assert_eq!(&*src_content, &*dest_content);
}
