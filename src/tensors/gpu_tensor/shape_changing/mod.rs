use crate::{GpuTensor, GpuTensorView, Tensor};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct DimStride{
    shape: VecDeque<usize>,
    stride: VecDeque<usize>,
}

impl GpuTensor{

    /// Tensor are broadcastable if:
    /// - Each tensor has at least one dimension.
    /// - When iterating over the dimension sizes, starting at the trailing dimension,
    /// the dimension sizes must either be equal, one of them is 1, or one of them does not exist.
    ///
    /// The broadcasting rules are:
    /// If the number of dimensions of x and y are not equal,
    /// prepend 1 to the dimensions of the tensor with fewer dimensions to make them equal length.
    /// Then, for each dimension size, the resulting dimension size is the max of the sizes of x and y along that dimension.
    pub fn broadcast<'a>(
        &'a self,
        other: &'a Self,
    ) -> Option<(GpuTensorView<'a>, GpuTensorView<'a>)> {

        //
        let mut current_shape = DimStride{
            shape: self.shape().clone(),
            stride: self.strides().clone()
        };

        let mut target_shape = DimStride{
            shape: other.shape().clone(),
            stride: other.strides().clone()
        };
        // let (larger_rank, smaller_rank) = if current_shape.shape.len() > target_shape.shape.len(){
        //     (&mut current_shape, &mut target_shape)
        // }else{
        //     (&mut target_shape, &mut current_shape)
        // };
        //
        // // make sure shapes have the same rank by adding 1s to the front of the shorter one
        // let rank_diff = larger_rank.shape.len() - smaller_rank.shape.len();
        // for _i in 0..rank_diff{
        //     smaller_rank.shape.extend_from_slice(&[1]);
        //     // this is a "fake" dimension
        //     smaller_rank.stride = super::utils::strides_from_shape(&smaller_rank.shape);
        // }
        // assert_eq!(smaller_rank.shape.len(), larger_rank.shape.len());
        //
        //
        //
        // // Now for each dimension:
        // let (curr_shape, curr_stride) = (&mut current_shape.shape, &mut current_shape.stride);
        // let (target_shape, target_stride) = (&mut target_shape.shape, &mut target_shape.stride);
        // for (id, (current_dim, target_dim)) in curr_shape.iter_mut().zip(target_shape.iter_mut()).enumerate(){
        //     // IF they are equal -> do nothing
        //     if current_dim == target_dim{
        //         continue;
        //     }
        //     // IF one of them is 1 and the other is DIM (DIM!=1) -> change dimension 1 into DIM, and
        //     // change strides to 0. This is OK since a stride of 0 means we will keep hitting the same
        //     // memory location when indexing into the expanded "fake" dimension increase
        //     match (&current_dim, &target_dim){
        //         (1, targ) =>{
        //             *current_dim = **targ;
        //             curr_stride[id] = 0;
        //         },
        //         (curr, 1) =>{
        //             *target_dim = **curr;
        //             target_stride[id] = 0;
        //         },
        //         // IF they are different and neither is 1, they are not broadcastable
        //         _ => {
        //             panic!(
        //                 "Cant broadcast between dims {} and {} .",
        //                 current_dim, target_dim
        //             );
        //         }
        //     }
        // }

        broadcast_shape_and_stripe(&mut current_shape, &mut target_shape);
        Some((
            GpuTensorView::new(self, current_shape.shape.clone(), current_shape.stride.clone()),
            GpuTensorView::new(other, target_shape.shape.clone(), target_shape.stride.clone()),
        ))

    }

    pub async fn transpose(&self) -> GpuTensor{
        use crate::tensors::gpu_tensor::gpu_ops::transpose;
        transpose(self.get_gpu(), &self).await
    }
}

fn broadcast_shape_and_stripe(current: &mut DimStride, target: &mut DimStride){
    let mut current_shape = (*current).clone();
    let mut target_shape = (*target).clone();
    let (larger_rank, smaller_rank) = if current_shape.shape.len() > target_shape.shape.len(){
        (&mut current_shape, &mut target_shape)
    }else{
        (&mut target_shape, &mut current_shape)
    };

    // make sure shapes have the same rank by adding 1s to the front of the shorter one
    let rank_diff = larger_rank.shape.len() - smaller_rank.shape.len();
    for _i in 0..rank_diff{
        smaller_rank.shape.push_front(1);
        // this is a "fake" dimension
        smaller_rank.stride = super::utils::strides_from_deque_shape(&smaller_rank.shape);
    }
    assert_eq!(smaller_rank.shape.len(), larger_rank.shape.len());
    println!("{:?}", smaller_rank);


    // Now for each dimension:
    let (curr_shape, curr_stride) = (&mut current_shape.shape, &mut current_shape.stride);
    let (target_shape, target_stride) = (&mut target_shape.shape, &mut target_shape.stride);
    for (id, (current_dim, target_dim)) in curr_shape.iter_mut().zip(target_shape.iter_mut()).enumerate(){
        // IF they are equal -> do nothing
        if current_dim == target_dim{
            continue;
        }
        // IF one of them is 1 and the other is DIM (DIM!=1) -> change dimension 1 into DIM, and
        // change strides to 0. This is OK since a stride of 0 means we will keep hitting the same
        // memory location when indexing into the expanded "fake" dimension increase
        match (&current_dim, &target_dim){
            (1, targ) => {
                *current_dim = **targ;
                curr_stride[id] = 0;
            },
            (curr, 1) => {
                *target_dim = **curr;
                target_stride[id] = 0;
            },
            // IF they are different and neither is 1, they are not broadcastable
            _ => {
                panic!(
                    "Cant broadcast between dims {} and {} .",
                    current_dim, target_dim
                );
            }
        }
    }
    current.shape = curr_shape.clone();
    current.stride = curr_stride.clone();
    target.shape = target_shape.clone();
    target.stride = target_stride.clone();
}


#[test]
pub fn broadcast_works(){
    let a_shape = VecDeque::from(vec![2, 2]);
    let mut a = DimStride{
        shape: a_shape.clone(),
        stride: crate::utils::strides_from_deque_shape(&a_shape)
    };
    let b_shape = VecDeque::from(vec![2, 2, 2]);
    let mut b = DimStride{
        shape: b_shape.clone(),
        stride: crate::utils::strides_from_deque_shape(&b_shape)
    };
    broadcast_shape_and_stripe(&mut a, &mut b);
    println!("{:?}", a);
    println!("{:?}", b);
}