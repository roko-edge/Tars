// use std::sync::Arc;
//
// #[derive(Debug, Clone)]
// pub struct Tensor {
//     storage: Arc<Vec<f32>>,
//     shape: Vec<usize>,
//     strides: Vec<usize>,
//     offset: usize,
// }
//
// impl Tensor {
//     pub fn new(storage: Vec<f32>, shape: Vec<usize>, offset: usize) -> Self {
//         let mut strides = vec![0; shape.len()];
//         let mut actual_stride = 1;
//
//         for i in (0..shape.len()).rev() {
//             strides[i] = actual_stride;
//             actual_stride *= shape[i];
//         }
//
//         Self {
//             storage: Arc::new(storage),
//             shape,
//             strides,
//             offset,
//         }
//     }
//
//     pub fn number_elements(&self) -> usize {
//         self.shape.iter().product()
//     }
//
//     pub fn transposed(&mut self) {
//         self.shape.reverse();
//         self.strides.reverse();
//     }
//
//     pub fn transposed_assign(&self) -> Self {
//         let mut output = self.clone();
//
//         output.shape.reverse();
//         output.strides.reverse();
//
//         output
//     }
//
//     pub fn mult_scale(&mut self, x: f32) {
//         for elements in Arc::make_mut(&mut self.storage) {
//             *elements *= x;
//         }
//     }
//
//     pub fn mult_scale_assign(&self, x: f32) -> Self {
//         let mut output = self.clone();
//
//         for elements in Arc::make_mut(&mut output.storage) {
//             *elements *= x;
//         }
//
//         output
//     }
//
//     pub fn div_scale(&mut self, x: f32) {
//         for elements in Arc::make_mut(&mut self.storage) {
//             *elements /= x;
//         }
//     }
//
//     pub fn div_scale_assign(&self, x: f32) -> Self {
//         let mut output = self.clone();
//
//         for elements in Arc::make_mut(&mut output.storage) {
//             *elements /= x;
//         }
//
//         output
//     }
//
//     pub fn add(&mut self, tensor: &Tensor) {
//         let data = Arc::make_mut(&mut self.storage);
//
//         for i in 0..tensor.storage.len() {
//             data[i] += tensor.storage[i];
//         }
//     }
//
//     pub fn add_assign(&self, tensor: &Tensor) -> Self {
//         let mut output = self.clone();
//         let data = Arc::make_mut(&mut output.storage);
//
//         for i in 0..tensor.storage.len() {
//             data[i] += tensor.storage[i];
//         }
//
//         output
//     }
//
//     pub fn hadamard(&mut self, tensor: &Tensor) {
//         let data = Arc::make_mut(&mut self.storage);
//
//         for i in 0..tensor.storage.len() {
//             data[i] *= tensor.storage[i];
//         }
//     }
//
//     pub fn hadamard_assign(&self, tensor: &Tensor) -> Self {
//         let mut output = self.clone();
//         let data = Arc::make_mut(&mut output.storage);
//
//         for i in 0..tensor.storage.len() {
//             data[i] *= tensor.storage[i];
//         }
//
//         output
//     }
//
//     // Defiimos self = A e tensor = B, ou seja, C = AB';
//     pub fn outer_product_assign(&mut self, tensor: &Tensor) -> Self {
//         tensor.transposed();
//     }
// }
