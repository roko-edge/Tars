use std::sync::Arc;
#[derive(Debug, Clone)]
pub struct Tensor {
    storage: Arc<Vec<f32>>,
    shape: Vec<usize>,
    strides: Vec<usize>,
    offset: usize,
}
impl Tensor {
    fn contiguous_strides(shape: &[usize]) -> Vec<usize> {
        let mut strides = vec![0; shape.len()];
        let mut actual_stride = 1;
        for i in (0..shape.len()).rev() {
            strides[i] = actual_stride;
            actual_stride *= shape[i];
        }
        strides
    }
    fn from(storage: Vec<f32>, shape: Vec<usize>, offset: usize) -> Self {
        let strides = Tensor::contiguous_strides(&shape);

        Self {
            storage: Arc::new(storage),
            shape,
            strides,
            offset,
        }
    }
    pub fn numel(&self) -> usize {
        self.shape.iter().product()
    }
    pub fn new(storage: Vec<f32>, shape: Vec<usize>) -> Self {
        let numel = shape.iter().product();
        assert_eq!(
            storage.len(),
            numel,
            "Storage received must match with shape calc of elements"
        );
        Tensor::from(storage, shape, 0)
    }
    fn storage_index(&self, indices: &[usize]) -> usize {
        assert_eq!(
            indices.len(),
            self.shape.len(),
            "indices must match with shape len"
        );
        let mut idx = self.offset;
        for i in 0..indices.len() {
            assert!(indices[i] < self.shape[i], "tensor index out of bounds");
            idx += indices[i] * self.strides[i];
        }
        idx
    }
    pub fn get(&self, index: &[usize]) -> f32 {
        let idx = self.storage_index(index);
        self.storage[idx]
    }
    pub fn set(&mut self, index: &[usize], value: f32) {
        let idx = self.storage_index(index);
        let storage =
            Arc::get_mut(&mut self.storage).expect("cannot mutate tensor with shared storage");
        storage[idx] = value;
    }
}
