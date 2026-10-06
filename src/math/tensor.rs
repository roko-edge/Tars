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

    pub fn is_contiguous(&self) -> bool {
        self.strides == Self::contiguous_strides(&self.shape)
    }

    fn from(storage: Vec<f32>, shape: Vec<usize>, offset: usize) -> Self {
        let strides = Self::contiguous_strides(&shape);

        Self {
            storage: Arc::new(storage),
            shape,
            strides,
            offset,
        }
    }

    pub fn new(storage: Vec<f32>, shape: Vec<usize>) -> Self {
        let numel = shape.iter().product();
        assert_eq!(
            storage.len(),
            numel,
            "Storage received must match with shape calc of elements"
        );
        Self::from(storage, shape, 0)
    }

    pub fn zeros(shape: Vec<usize>) -> Self {
        let numel = shape.iter().product();

        let storage = vec![0.0; numel];

        Self::from(storage, shape, 0)
    }

    pub fn random_uniform(&mut self, low: f32, high: f32) {
        let storage =
            Arc::get_mut(&mut self.storage).expect("***N sei o que faz, boto o erro dps***");

        for i in 0..storage.len() {
            storage[i] = low + (high - low) * rand::random::<f32>();
        }
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

    pub fn storage(&self) -> Arc<Vec<f32>> {
        self.storage.clone()
    }

    pub fn shape(&self) -> &[usize] {
        &self.shape
    }

    pub fn strides(&self) -> &[usize] {
        &self.strides
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn set(&mut self, index: &[usize], value: f32) {
        let idx = self.storage_index(index);
        let storage =
            Arc::get_mut(&mut self.storage).expect("cannot mutate tensor with shared storage");
        storage[idx] = value;
    }

    pub fn numel(&self) -> usize {
        self.shape.iter().product()
    }

    pub fn transpose_mut(&mut self, dim0: usize, dim1: usize) {
        assert!(
            dim0 < self.shape.len() && dim1 < self.shape.len(),
            "dimension/s out of bounds"
        );

        self.shape.swap(dim0, dim1);
        self.strides.swap(dim0, dim1);
    }

    pub fn transpose(&self, dim0: usize, dim1: usize) -> Self {
        assert!(
            dim0 < self.shape.len() && dim1 < self.shape.len(),
            "dimension/s out of bouns"
        );

        let mut new_shape = self.shape.clone();
        let mut new_strides = self.strides.clone();

        new_shape.swap(dim0, dim1);
        new_strides.swap(dim0, dim1);

        Self {
            storage: self.storage.clone(),
            shape: new_shape,
            strides: new_strides,
            offset: self.offset,
        }
    }

    pub fn dot_product(&self, other: &Tensor) -> f32 {
        assert_eq!(
            self.storage.len(),
            other.storage.len(),
            "length of storage is different"
        );

        let mut sum = 0.0;

        for i in 0..self.storage.len() {
            sum += self.storage[i] * other.storage[i];
        }

        sum
    }

    pub fn add(&self, tensor: &Tensor) -> Self {
        assert_eq!(self.shape, tensor.shape, "shape doesn't match");
        assert_eq!(self.offset, tensor.offset, "offset doesn't match");

        let mut new_storage = self.storage.clone();
        let storage = Arc::make_mut(&mut new_storage);

        for i in self.offset..storage.len() {
            storage[i] += tensor.storage[i];
        }

        Self {
            storage: new_storage,
            shape: self.shape.clone(),
            strides: self.strides.clone(),
            offset: 0,
        }
    }

    pub fn add_mut(&mut self, tensor: &Tensor) {
        assert_eq!(self.shape, tensor.shape, "shape doesn't match");
        assert_eq!(self.offset, tensor.offset, "offset doesn't match");

        let storage = Arc::make_mut(&mut self.storage);

        for i in self.offset..storage.len() {
            storage[i] += tensor.storage[i];
        }
    }

    pub fn sub(&self, tensor: &Tensor) -> Self {
        assert_eq!(self.offset, tensor.offset, "offset doesn't match");

        let mut new_storage = self.storage.clone();
        let storage = Arc::make_mut(&mut new_storage);

        for i in self.offset..storage.len() {
            storage[i] -= tensor.storage[i];
        }

        Self {
            storage: new_storage,
            shape: self.shape.clone(),
            strides: self.strides.clone(),
            offset: 0,
        }
    }

    pub fn sub_mut(&mut self, tensor: &Tensor) {
        assert_eq!(self.shape, tensor.shape, "shape doesn't match");
        assert_eq!(self.offset, tensor.offset, "offset doesn't match");

        let storage = Arc::make_mut(&mut self.storage);

        for i in self.offset..storage.len() {
            storage[i] -= tensor.storage[i];
        }
    }

    pub fn mut_scalar(&self, s: f32) -> Self {
        let mut new_storage = self.storage.clone();
        let storage = Arc::make_mut(&mut new_storage);

        for i in self.offset..self.storage.len() {
            storage[i] *= s;
        }

        Self {
            storage: new_storage,
            shape: self.shape.clone(),
            strides: self.strides.clone(),
            offset: 0,
        }
    }

    pub fn mut_scalar_mut(&mut self, s: f32) {
        let storage = Arc::make_mut(&mut self.storage);

        for i in self.offset..storage.len() {
            storage[i] *= s;
        }
    }

    pub fn div_scalar(&self, s: f32) -> Self {
        let mut new_storage = self.storage.clone();
        let storage = Arc::make_mut(&mut new_storage);

        for i in self.offset..self.storage.len() {
            storage[i] /= s;
        }

        Self {
            storage: new_storage,
            shape: self.shape.clone(),
            strides: self.strides.clone(),
            offset: 0,
        }
    }

    pub fn div_scalar_mut(&mut self, s: f32) {
        let storage = Arc::make_mut(&mut self.storage);

        for i in self.offset..storage.len() {
            storage[i] *= s;
        }
    }
}
