mod cs {
    #[derive(VulkanoShader)]
    #[ty = "compute"]
    #[src = "
#version 450

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Data {
    uint data[];
} buf;

void main() {
    uint idx = gl_GlobalInvocationID.x;
    buf.data[idx] *= 12;
}"
    ]
    struct Dummy;
}

use std::sync::Arc;
use vulkano as vk;
use vk::pipeline::ComputePipeline;
use vk::buffer::{ BufferUsage, CpuAccessibleBuffer };
use vk::descriptor::descriptor_set::PersistentDescriptorSet;
use vk::command_buffer::{ CommandBuffer, AutoCommandBufferBuilder };
use vk::sync::GpuFuture;

pub fn run(device: &Arc<vk::device::Device>, queue: &Arc<vk::device::Queue>) {
    let data_iter = 0 .. 65536;
    let data_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                                 data_iter).expect("failed to create buffer");

    let shader = cs::Shader::load(device.clone())
        .expect("failed to create shader module");

    let compute_pipeline = Arc::new(ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
        .expect("failed to create compute pipeline"));

    let set = Arc::new(PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
        .add_buffer(data_buffer.clone()).unwrap()
        .build().unwrap()
    );

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .dispatch([1024, 1, 1], compute_pipeline.clone(), set.clone(), ()).unwrap()
        .build().unwrap();
    
    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let content = data_buffer.read().unwrap();
    for (n, val) in content.iter().enumerate() {
        assert_eq!(*val, n as u32 * 12);
    }

    println!("Everything succeeded!");
}