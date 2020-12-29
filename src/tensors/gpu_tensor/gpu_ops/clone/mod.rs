use crate::gpu_internals::shader_runner::{BufferType, ShaderBinding, ThreadGroup};
use crate::gpu_internals::GpuInstance;
use crate::{GpuAllocated, GpuTensor, ShapeStrideTrait};

#[cfg(test)]
mod tests;

pub async fn clone<'a>(gpu: &GpuInstance, data: &GpuTensor) -> GpuTensor {
    let cs_module = gpu.shader_from_file_bytes(wgpu::include_spirv!("clone.spv"));
    let nb_output_numbers = data.numel();
    let output = gpu.new_empty_gpu_buffer(std::mem::size_of::<f32>() * nb_output_numbers);
    // gpu.run_shader(
    //     &cs_module,
    //     vec![
    //         ShaderBinding {
    //             binding_id: 0,
    //             gpu_buffer: BufferType::Storage(data.internal_gpu_buffer()),
    //         },
    //         ShaderBinding {
    //             binding_id: 1,
    //             gpu_buffer: BufferType::Storage(&output),
    //         },
    //     ],
    //     None,
    //     ThreadGroup {
    //         x: nb_output_numbers,
    //         y: 1,
    //         z: 1,
    //     },
    // );
    // GpuTensor {
    //     buffer: output,
    //     shape_strides: data.shape_strides.clone(),
    // }
    unimplemented!()
}
