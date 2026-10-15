use crate::math::tensor::{Storage, Tensor};
use std::sync::Arc;

#[test]
fn contiguous_strides_test() {
    let storage = Storage::F32(Arc::new(vec![1.0, 2.0, 3.0]));
    let shape = vec![1, 2];
    let t = Tensor::new(storage, shape);
    assert!(t.is_contiguous(), "Tensor strides aren't contiguous");
}
