use core::panic;
use std::sync::Arc;

#[derive(Debug)]
pub enum DType {
    F32(f32),
    T3,
    I8(i8),
}

#[derive(Debug, Clone)]
pub struct Tensor {
    storage: Arc<Vec<DType>>,
    shape: Vec<usize>,
    strides: Vec<usize>,
    offset: usize,
}

// This enum defines all types that a tensor can store. functions that implemments a tensor operator should override each operator based on the type you're working with.

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
        assert_eq!(self.shape, other.shape, "tensors has not the same shape");

        let mut sum = 0.0;
        let rank = self.shape.len();

        match rank {
            1 => {
                for i in 0..self.shape[0] {
                    sum += self.get(&[i]) * other.get(&[i])
                }
            }
            2 => {
                for i in 0..self.shape[0] {
                    for j in 0..self.shape[1] {
                        sum += self.get(&[i, j]) * other.get(&[i, j]);
                    }
                }
            }
            _ => {
                panic!("Only implemented to 1 and 2 dimensions")
            }
        }

        sum
    }

    pub fn add(&self, tensor: &Tensor) -> Self {
        assert_eq!(self.shape, tensor.shape, "shape doesn't match");

        let mut new_tensor = Tensor::zeros(self.shape.clone());
        let rank = self.shape.len();

        match rank {
            1 => {
                for i in 0..self.shape[0] {
                    new_tensor.set(&[i], self.get(&[i]) + tensor.get(&[i]));
                }
            }
            2 => {
                for i in 0..self.shape[0] {
                    for j in 0..self.shape[1] {
                        new_tensor.set(&[i, j], self.get(&[i, j]) + tensor.get(&[i, j]));
                    }
                }
            }
            _ => {
                panic!("Only implemented to 1 and 2 dimensions")
            }
        }

        new_tensor
    }

    pub fn add_mut(&mut self, tensor: &Tensor) {
        assert_eq!(self.shape, tensor.shape, "shape doesn't match");

        let storage = Arc::make_mut(&mut self.storage);

        for i in self.offset..storage.len() {
            storage[i] += tensor.storage[i];
        }
    }

    pub fn sub(&self, tensor: &Tensor) -> Self {
        assert_eq!(self.shape, tensor.shape, "shape doesn't match");
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

    // Entrega uma matriz e um vetor coluna, como no foward z = (W*x + b), onde z é um tensor
    // coluna, W uma matriz, x um tensor coluna e b tbm.
    pub fn matvec(&self, tensor: &Tensor) -> Self {
        assert_eq!(self.shape.len(), 2, "self is not a matrix");
        assert_eq!(tensor.shape.len(), 1, "tensor is not a vector");
        assert_eq!(
            self.shape[1], tensor.shape[0],
            "The collum of the matrix is different of the rows of the vector"
        );

        let mut new_tensor = Tensor::zeros(vec![self.shape[0]]);

        for i in 0..self.shape[0] {
            for j in 0..self.shape[1] {
                new_tensor.set(
                    &[i],
                    new_tensor.get(&[i]) + self.get(&[i, j]) * tensor.get(&[j]),
                );
            }
        }

        new_tensor
    }

    pub fn outer(&self, tensor: &Tensor) -> Self {
        assert_eq!(self.shape.len(), 1, "self doesn't have 1 dimension");
        assert_eq!(tensor.shape.len(), 1, "self doesn't have 1 dimension");

        let mut new_tensor = Tensor::zeros(vec![self.shape[0], tensor.shape[0]]);

        for i in 0..self.shape[0] {
            for j in 0..tensor.shape[0] {
                new_tensor.set(&[i, j], self.get(&[i]) * tensor.get(&[j]));
            }
        }

        new_tensor
    }

    fn map(&self, f: fn(f32) -> f32) -> Tensor {
        let mut new_tensor = Tensor::zeros(self.shape.clone());

        for i in 0..self.storage.len() {
            new_tensor.storage[i] = f(self.get(i));
        }

        for i in self.storage() {}

        new_tensor
    }

    fn map_mut(&mut self, f: fn(f32) -> f32) {
        todo!("Ain't done yet!");
    }
}
