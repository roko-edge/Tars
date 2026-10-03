use crate::Tensor;
use std::sync::Arc;

// Stride means how much you should walk to change direction

#[test]
pub fn numel_test() {
    let storage: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let shape: Vec<usize> = vec![2, 3];
    let tensor: Tensor = Tensor::new(storage, shape, 0);

    assert_eq!(tensor.numel(), 6);
}

#[test]
fn transpose_test() {
    let storage: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let shape: Vec<usize> = vec![2, 3];
    let mut tensor = Tensor::new(storage.clone(), shape, 0);

    tensor.transpose();

    assert_eq!(tensor.shape, vec![3, 2]);
    assert_eq!(tensor.strides, vec![1, 3]);
}

#[test]
fn transpose_assign_test() {
    let storage: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let shape: Vec<usize> = vec![2, 3];
    let mut tensor = Tensor::new(storage.clone(), shape, 0);

    let assigned_tensor = tensor.transposed_assign();
    tensor.transposed();

    assert_eq!(assigned_tensor.strides, tensor.strides);
}

#[test]
pub fn mult_scale_test() {
    let storage: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let shape: Vec<usize> = vec![2, 3];
    let mut tensor = Tensor::new(storage.clone(), shape, 0);

    tensor.mult_scale(10.0);

    let expected = Arc::new(vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0]);

    assert_eq!(tensor.storage, expected);
}
