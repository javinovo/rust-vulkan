extern crate vulkano;
use vulkano as vk;

use std::sync::Arc;

pub fn test_buffer_copy(device: &Arc<vk::device::Device>, queue: &Arc<vk::device::Queue>) {

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

pub fn create_image(device: &Arc<vk::device::Device>, queue: &Arc<vk::device::Queue>, clear_color: [f32; 4], path: &str) {

    extern crate image;

    use vk::format::{ Format, ClearValue };
    use vk::image::{ Dimensions, StorageImage };
    use vk::buffer::{ BufferUsage, CpuAccessibleBuffer };
    use vk::command_buffer::{ CommandBuffer, AutoCommandBufferBuilder };
    use vk::sync::GpuFuture;

    // Creating an image is very similar to creating a buffer. Just like there are multiple different structs in vulkano that represent buffers, 
    // there are also multiple different structs that represent images. Here we are going to use a StorageImage, which is a general-purpose image.
    let image = StorageImage::new(device.clone(), Dimensions::Dim2d { width: 1024, height: 1024 },
                    Format::R8G8B8A8Unorm, // When you create an image you must also choose a format for its pixels. 
                    Some(queue.family())).unwrap(); // The queue family to use is similar to the parameter when creating a buffer. It indicates which queue families are going to access the image.
                                  

    let buf = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                         (0 .. 1024 * 1024 * 4).map(|_| 0u8))
                                         .expect("failed to create buffer");    

    // Contrary to buffers, images have an opaque implementation-specific memory layout. What this means is that you can't modify an image by directly writing to its memory. 
    // Therefore the only way to read or write to an image is to ask the GPU to do it. This is exactly what we are going to do by asking the GPU to fill our image with a specific color.
    // This is called clearing an image. Then we ask the CPU to copy it to a buffer accessible by the GPU.
    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .clear_color_image(image.clone(), ClearValue::Float(clear_color)).unwrap() 
        .copy_image_to_buffer(image.clone(), buf.clone()).unwrap()
        .build().unwrap();

    // Execute the command buffer and block until the operation is finished
    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
            .wait(None).unwrap();       

    // Read the buffer and save the image to a file
    let buffer_content = buf.read().unwrap();
    let image = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();

    image.save(path).unwrap();
}