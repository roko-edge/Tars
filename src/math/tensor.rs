use core::panic;
use std::{sync::Arc, vec};

#[derive(Debug)]
pub enum DType {
    F32,
    I8,
    T3,
}

#[derive(Debug, Clone)]
pub enum Storage {
    F32(Arc<Vec<f32>>),
    I8 { data: Arc<Vec<i8>>, scale: f32 },
    T3 { data: Arc<Vec<u8>>, scale: f32 },
}

#[derive(Debug, Clone)]
pub struct Tensor {
    storage: Storage,
    shape: Vec<usize>,
    strides: Vec<usize>,
    offset: usize,
}

impl Tensor {
    pub fn new(storage: Storage, shape: Vec<usize>) -> Self {
        Self::from(storage, shape, 0)
    }

    fn from(storage: Storage, shape: Vec<usize>, offset: usize) -> Self {
        let strides = Self::contiguous_strides(&shape);
        Self {
            storage,
            shape,
            strides,
            offset,
        }
    }
    pub fn from_vec(data: Vec<f32>, shape: Vec<usize>) -> Self {
        assert_eq!(data.len(), shape.iter().product(), "");
        Self::new(Storage::F32(Arc::new(data)), shape)
    }
    pub fn from_slice(data: &[f32], shape: Vec<usize>) -> Self {
        Self::from_vec(data.to_vec(), shape)
    }
    fn contiguous_strides(shape: &[usize]) -> Vec<usize> {
        let mut strides = vec![0; shape.len()];
        let mut actual_stride = 1;
        for i in (0..shape.len()).rev() {
            strides[i] = actual_stride;
            actual_stride *= shape[i];
        }
        strides
    }

    fn step_logical(shape: &Vec<usize>, v: &mut Vec<usize>) {
        v[shape.len() - 1] += 1;

        let mut i = shape.len() - 1;

        while v[i] >= shape[i] {
            v[i] = 0;

            if i > 0 {
                v[i - 1] += 1;
                i -= 1;
            }
        }
    }

    pub fn make_contiguous(&self) -> Self {
        if self.is_contiguous() {
            self.clone()
        } else {
            let mut tensor = Self::zeros(self.shape.clone(), self.dtype());
            let mut indices = vec![0usize; self.shape.len()];

            for _ in 0..self.numel() {
                tensor.set(&indices, self.get(&indices));
                Self::step_logical(&self.shape, &mut indices);
            }

            tensor
        }
    }

    // fn make_contiguous_mut(&self) -> Self {}

    pub fn is_contiguous(&self) -> bool {
        self.strides == Self::contiguous_strides(&self.shape)
            && self.offset == 0
            && match &self.storage {
                Storage::F32(storage) => storage.len() != 0,
                Storage::I8 { .. } => panic!(),
                Storage::T3 { .. } => panic!(),
            }
    }

    pub fn zeros(shape: Vec<usize>, dtype: DType) -> Self {
        let numel = shape.iter().product();

        let storage = match dtype {
            DType::F32 => Storage::F32(Arc::new(vec![0.0; numel])),
            DType::I8 => Storage::I8 {
                data: Arc::new(vec![0i8; numel]),
                scale: 0.0,
            },
            DType::T3 => Storage::T3 {
                data: Arc::new(vec![0u8; (numel + 3).div_ceil(4)]),
                scale: 0.0,
            },
        };

        Self::from(storage, shape, 0)
    }

    fn storage_index(&self, indices: &[usize]) -> usize {
        assert_eq!(
            indices.len(),
            self.shape.len(),
            "indices must match with shape len"
        );
        let idx: usize = indices
            .iter()
            .zip(self.shape.iter())
            .zip(self.strides.iter())
            .map(|((&index, &dim), &stride)| {
                assert!(index < dim, "tensor index out of bounds!");
                index * stride
            })
            .sum();
        self.offset + idx
    }

    pub fn get(&self, index: &[usize]) -> f32 {
        let idx = self.storage_index(index);
        match &self.storage {
            Storage::F32(storage) => storage[idx],
            Storage::I8 { .. } => panic!(),
            Storage::T3 { .. } => panic!(),
        }
    }
    pub fn set(&mut self, index: &[usize], value: f32) {
        let idx = self.storage_index(index);
        let storage = match &mut self.storage {
            Storage::F32(storage) => Arc::make_mut(storage),
            Storage::I8 { .. } => panic!(),
            Storage::T3 { .. } => panic!(),
        };
        storage[idx] = value;
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

    pub fn numel(&self) -> usize {
        self.shape.iter().product()
    }
    pub fn dtype(&self) -> DType {
        match &self.storage {
            Storage::F32(_) => DType::F32,
            Storage::I8 { .. } => DType::I8,
            Storage::T3 { .. } => DType::T3,
        }
    }

    pub fn storage(&self) -> Arc<Vec<f32>> {
        match &self.storage {
            Storage::F32(storage) => storage.clone(),
            Storage::I8 { .. } => panic!(),
            Storage::T3 { .. } => panic!(),
        }
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
            "dimension/s out of bounds"
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
    pub fn add(&self, other: &Self) -> Self {
        assert_eq!(self.shape, other.shape, "");
        let mut out = Tensor::zeros(self.shape.clone(), self.dtype());
        let mut idx = vec![0usize; self.shape.len()];
        for _ in 0..self.numel() {
            out.set(&idx, self.get(&idx) + other.get(&idx));
            Self::step_logical(&self.shape, &mut idx);
        }
        out
    }
    pub fn add_mut(&mut self, other: &Self) {
        assert_eq!(self.shape, other.shape, "");
        let mut idx = vec![0usize; self.shape.len()];
        for _ in 0..self.numel() {
            self.set(&idx, self.get(&idx) + other.get(&idx));
            Self::step_logical(&self.shape, &mut idx);
        }
    }
    pub fn sub(&self, other: &Self) -> Self {
        assert_eq!(self.shape, other.shape, "");
        let mut out = Tensor::zeros(self.shape.clone(), self.dtype());
        let mut idx = vec![0usize; self.shape.len()];
        for _ in 0..self.numel() {
            out.set(&idx, self.get(&idx) - other.get(&idx));
            Self::step_logical(&self.shape, &mut idx);
        }
        out
    }
    pub fn sub_mut(&mut self, other: &Self) {
        assert_eq!(self.shape, other.shape, "");
        let mut idx = vec![0usize; self.shape.len()];
        for _ in 0..self.numel() {
            self.set(&idx, self.get(&idx) - other.get(&idx));
            Self::step_logical(&self.shape, &mut idx);
        }
    }
    pub fn mul(&self, other: &Self) -> Self {
        assert_eq!(self.shape, other.shape, "");
        let mut out = Tensor::zeros(self.shape.clone(), self.dtype());
        let mut idx = vec![0usize; self.shape.len()];
        for _ in 0..self.numel() {
            out.set(&idx, self.get(&idx) * other.get(&idx));
            Self::step_logical(&self.shape, &mut idx);
        }
        out
    }
    pub fn mul_mut(&mut self, other: &Self) {
        assert_eq!(self.shape, other.shape, "");
        let mut idx = vec![0usize; self.shape.len()];
        for _ in 0..self.numel() {
            self.set(&idx, self.get(&idx) * other.get(&idx));
            Self::step_logical(&self.shape, &mut idx);
        }
    }
    pub fn div(&self, other: &Self) -> Self {
        assert_eq!(self.shape, other.shape, "");
        let mut out = Tensor::zeros(self.shape.clone(), self.dtype());
        let mut idx = vec![0usize; self.shape.len()];
        for _ in 0..self.numel() {
            out.set(&idx, self.get(&idx) / other.get(&idx));
            Self::step_logical(&self.shape, &mut idx);
        }
        out
    }
    pub fn div_mut(&mut self, other: &Self) {
        assert_eq!(self.shape, other.shape, "");
        let mut idx = vec![0usize; self.shape.len()];
        for _ in 0..self.numel() {
            self.set(&idx, self.get(&idx) / other.get(&idx));
            Self::step_logical(&self.shape, &mut idx);
        }
    }
    pub fn mul_escalar(&self, s: f32) -> Self {
        let mut out = Tensor::zeros(self.shape.clone(), self.dtype());
        let mut idx = vec![0usize; self.shape.len()];
        for _ in 0..self.numel() {
            out.set(&idx, self.get(&idx) * s);
            Self::step_logical(&self.shape, &mut idx);
        }
        out
    }
    pub fn mul_escalar_mut(&mut self, s: f32) {
        let mut idx = vec![0usize; self.shape.len()];
        for _ in 0..self.numel() {
            self.set(&idx, self.get(&idx) * s);
            Self::step_logical(&self.shape, &mut idx);
        }
    }
    pub fn div_escalar(&self, s: f32) -> Self {
        let mut out = Tensor::zeros(self.shape.clone(), self.dtype());
        let mut idx = vec![0usize; self.shape.len()];
        for _ in 0..self.numel() {
            out.set(&idx, self.get(&idx) / s);
            Self::step_logical(&self.shape, &mut idx);
        }
        out
    }
    pub fn div_escalar_mut(&mut self, s: f32) {
        let mut idx = vec![0usize; self.shape.len()];
        for _ in 0..self.numel() {
            self.set(&idx, self.get(&idx) / s);
            Self::step_logical(&self.shape, &mut idx);
        }
    }
    pub fn map(&self, f: impl Fn(f32) -> f32) -> Self {
        let mut out = Tensor::zeros(self.shape.clone(), self.dtype());
        let mut idx = vec![0usize; self.shape.len()];
        for _ in 0..self.numel() {
            out.set(&idx, f(self.get(&idx)));
            Self::step_logical(&self.shape, &mut idx);
        }
        out
    }
    pub fn map_mut(&mut self, f: impl Fn(f32) -> f32) {
        let mut idx = vec![0usize; self.shape.len()];
        for _ in 0..self.numel() {
            self.set(&idx, f(self.get(&idx)));
            Self::step_logical(&self.shape, &mut idx);
        }
    }
    //
    // pub fn dot_product(&self, other: &Tensor) -> f32 {
    //     assert_eq!(self.shape, other.shape, "tensors has not the same shape");
    //
    //     let mut sum = 0.0;
    //     let rank = self.shape.len();
    //
    //     match rank {
    //         1 => {
    //             for i in 0..self.shape[0] {
    //                 sum += self.get(&[i]) * other.get(&[i])
    //             }
    //         }
    //         2 => {
    //             for i in 0..self.shape[0] {
    //                 for j in 0..self.shape[1] {
    //                     sum += self.get(&[i, j]) * other.get(&[i, j]);
    //                 }
    //             }
    //         }
    //         _ => {
    //             panic!("Only implemented to 1 and 2 dimensions")
    //         }
    //     }
    //
    //     sum
    // }
    //
    // pub fn add(&self, tensor: &Tensor) -> Self {
    //     assert_eq!(self.shape, tensor.shape, "shape doesn't match");
    //
    //     let mut new_tensor = Tensor::zeros(self.shape.clone());
    //     let rank = self.shape.len();
    //
    //     match rank {
    //         1 => {
    //             for i in 0..self.shape[0] {
    //                 new_tensor.set(&[i], self.get(&[i]) + tensor.get(&[i]));
    //             }
    //         }
    //         2 => {
    //             for i in 0..self.shape[0] {
    //                 for j in 0..self.shape[1] {
    //                     new_tensor.set(&[i, j], self.get(&[i, j]) + tensor.get(&[i, j]));
    //                 }
    //             }
    //         }
    //         _ => {
    //             panic!("Only implemented to 1 and 2 dimensions")
    //         }
    //     }
    //
    //     new_tensor
    // }
    //
    // pub fn add_mut(&mut self, tensor: &Tensor) {
    //     assert_eq!(self.shape, tensor.shape, "shape doesn't match");
    //
    //     let storage = Arc::make_mut(&mut self.storage);
    //
    //     for i in self.offset..storage.len() {
    //         storage[i] += tensor.storage[i];
    //     }
    // }
    //
    // pub fn sub(&self, tensor: &Tensor) -> Self {
    //     assert_eq!(self.shape, tensor.shape, "shape doesn't match");
    //     assert_eq!(self.offset, tensor.offset, "offset doesn't match");
    //
    //     let mut new_storage = self.storage.clone();
    //     let storage = Arc::make_mut(&mut new_storage);
    //
    //     for i in self.offset..storage.len() {
    //         storage[i] -= tensor.storage[i];
    //     }
    //
    //     Self {
    //         storage: new_storage,
    //         shape: self.shape.clone(),
    //         strides: self.strides.clone(),
    //         offset: 0,
    //     }
    // }
    //
    // pub fn sub_mut(&mut self, tensor: &Tensor) {
    //     assert_eq!(self.shape, tensor.shape, "shape doesn't match");
    //     assert_eq!(self.offset, tensor.offset, "offset doesn't match");
    //
    //     let storage = Arc::make_mut(&mut self.storage);
    //
    //     for i in self.offset..storage.len() {
    //         storage[i] -= tensor.storage[i];
    //     }
    // }
    //
    // pub fn mut_scalar(&self, s: f32) -> Self {
    //     let mut new_storage = self.storage.clone();
    //     let storage = Arc::make_mut(&mut new_storage);
    //
    //     for i in self.offset..self.storage.len() {
    //         storage[i] *= s;
    //     }
    //
    //     Self {
    //         storage: new_storage,
    //         shape: self.shape.clone(),
    //         strides: self.strides.clone(),
    //         offset: 0,
    //     }
    // }
    //
    // pub fn mut_scalar_mut(&mut self, s: f32) {
    //     let storage = Arc::make_mut(&mut self.storage);
    //
    //     for i in self.offset..storage.len() {
    //         storage[i] *= s;
    //     }
    // }
    //
    // pub fn div_scalar(&self, s: f32) -> Self {
    //     let mut new_storage = self.storage.clone();
    //     let storage = Arc::make_mut(&mut new_storage);
    //
    //     for i in self.offset..self.storage.len() {
    //         storage[i] /= s;
    //     }
    //
    //     Self {
    //         storage: new_storage,
    //         shape: self.shape.clone(),
    //         strides: self.strides.clone(),
    //         offset: 0,
    //     }
    // }
    //
    // pub fn div_scalar_mut(&mut self, s: f32) {
    //     let storage = Arc::make_mut(&mut self.storage);
    //
    //     for i in self.offset..storage.len() {
    //         storage[i] *= s;
    //     }
    // }
    //
    // // Entrega uma matriz e um vetor coluna, como no foward z = (W*x + b), onde z é um tensor
    // // coluna, W uma matriz, x um tensor coluna e b tbm.
    // pub fn matvec(&self, tensor: &Tensor) -> Self {
    //     assert_eq!(self.shape.len(), 2, "self is not a matrix");
    //     assert_eq!(tensor.shape.len(), 1, "tensor is not a vector");
    //     assert_eq!(
    //         self.shape[1], tensor.shape[0],
    //         "The collum of the matrix is different of the rows of the vector"
    //     );
    //
    //     let mut new_tensor = Tensor::zeros(vec![self.shape[0]]);
    //
    //     for i in 0..self.shape[0] {
    //         for j in 0..self.shape[1] {
    //             new_tensor.set(
    //                 &[i],
    //                 new_tensor.get(&[i]) + self.get(&[i, j]) * tensor.get(&[j]),
    //             );
    //         }
    //     }
    //
    //     new_tensor
    // }
    //
    // pub fn outer(&self, tensor: &Tensor) -> Self {
    //     assert_eq!(self.shape.len(), 1, "self doesn't have 1 dimension");
    //     assert_eq!(tensor.shape.len(), 1, "self doesn't have 1 dimension");
    //
    //     let mut new_tensor = Tensor::zeros(vec![self.shape[0], tensor.shape[0]]);
    //
    //     for i in 0..self.shape[0] {
    //         for j in 0..tensor.shape[0] {
    //             new_tensor.set(&[i, j], self.get(&[i]) * tensor.get(&[j]));
    //         }
    //     }
    //
    //     new_tensor
    // }
    //
    // fn map(&self, f: fn(f32) -> f32) -> Tensor {
    //     let mut new_tensor = Tensor::zeros(self.shape.clone());
    //
    //     for i in 0..self.storage.len() {
    //         new_tensor.storage[i] = f(self.get(i));
    //     }
    //
    //     for i in self.storage() {}
    //
    //     new_tensor
    // }
    //
    // fn map_mut(&mut self, f: fn(f32) -> f32) {
    //     todo!("Ain't done yet!");
    // }
}
