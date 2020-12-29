use crate::gpu_internals::shader_runner::{BufferType, ShaderBinding, ThreadGroup};
use crate::gpu_internals::GpuInstance;
use crate::{GpuAllocated, GpuTensor, ShapeStrideTrait};
use zerocopy::{AsBytes, FromBytes};
#[cfg(test)]
mod tests;

#[repr(C)]
#[derive(AsBytes, FromBytes, Clone, Debug)]
struct TensorStructure {
    nb_shapes: u32,
}

pub async fn transpose(gpu: &GpuInstance, data: &GpuTensor) -> GpuTensor {
    let cs_module = gpu.shader_from_file_bytes(wgpu::include_spirv!("transpose.spv"));
    let nb_output_numbers = data.numel();
    let out_buffer_store = gpu.new_empty_gpu_buffer(std::mem::size_of::<f32>() * nb_output_numbers);

    let shape_u32: Vec<u32> = data.shape().iter().map(|e| *e as u32).collect();
    let shapes = gpu.new_gpu_buffer_from_data(shape_u32.as_slice().as_bytes());

    let strides_u32: Vec<u32> = data.strides().iter().map(|e| *e as u32).collect();
    let strides = gpu.new_gpu_buffer_from_data(strides_u32.as_slice().as_bytes());

    let nb_shapes = gpu.new_gpu_buffer_from_data((shape_u32.len() as u32).as_bytes());
    //
    // gpu.run_shader(
    //     &cs_module,
    //     vec![
    //         ShaderBinding {
    //             binding_id: 0,
    //             gpu_buffer: BufferType::Storage(data.internal_gpu_buffer()), // tensor data
    //         },
    //         ShaderBinding {
    //             binding_id: 1,
    //             gpu_buffer: BufferType::Storage(&shapes), // tensor shape
    //         },
    //         ShaderBinding {
    //             binding_id: 2,
    //             gpu_buffer: BufferType::Storage(&strides), // tensor strides
    //         },
    //         ShaderBinding {
    //             binding_id: 3,
    //             gpu_buffer: BufferType::Storage(&nb_shapes), // number of shapes and strides
    //         },
    //         ShaderBinding {
    //             binding_id: 4,
    //             gpu_buffer: BufferType::Storage(&out_buffer_store),
    //         },
    //     ],
    //     None,
    //     ThreadGroup {
    //         x: nb_output_numbers,
    //         y: 1,
    //         z: 1,
    //     },
    // );
    // let mut shape = data.shape().clone();
    // shape.swap(shape.len() - 2, shape.len() - 1);
    // GpuTensor::from_buffer(out_buffer_store, shape.clone())
    unimplemented!()
}
