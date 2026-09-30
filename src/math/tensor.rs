use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Tensor {
    data: Arc<Vec<f32>>,
    dimensions: Vec<usize>,
    strides: Vec<usize>,
}

impl Tensor {
    pub fn new(data: Vec<f32>, dimensions: Vec<usize>) -> Self {
        let mut strides = vec![0; dimensions.len()];
        let mut actual_stride = 1;

        for i in (0..dimensions.len() - 1).rev() {
            strides[i] = actual_stride;
            actual_stride *= dimensions[i];
        }

        Self {
            data: Arc::new(data),
            dimensions,
            strides,
        }
    }
}
