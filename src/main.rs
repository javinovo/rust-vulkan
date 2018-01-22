
extern crate vulkano;

use vulkano as vk;

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


    use vk::buffer::{ BufferUsage, CpuAccessibleBuffer };

    // In order for the GPU to be able to access some data (either for reading, writing or both), we first need to create a buffer object and put the data in it.

    let source_content = 0 .. 64; // From 0 to 63
    let source = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), source_content).expect("failed to create buffer");

    let dest_content = (0 .. 64).map(|_| 0); // 64 zeroes
    let dest = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), dest_content).expect("failed to create buffer");


    use vk::command_buffer::{ AutoCommandBufferBuilder, CommandBuffer };

    // With Vulkan and vulkano you can't just execute commands one by one, as it would be too inefficient. 
    // Instead, we need to build a command buffer that contains a list of commands that we want to execute.
    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .copy_buffer(source.clone(), dest.clone()).unwrap()
        .build().unwrap();
    
    let finished = command_buffer.execute(queue.clone()).unwrap();
        

    use vk::sync::GpuFuture;

    // Submitting an operation doesn't wait for the operation to be complete.
    // Instead it just sends some kind of signal to the GPU to instruct it that it must start processing the command buffer, 
    // and the actual processing is performed asynchronously.
    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();
        
    let src_content = source.read().unwrap();
    let dest_content = dest.read().unwrap();
    assert_eq!(&*src_content, &*dest_content);
}
