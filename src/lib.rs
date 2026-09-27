pub mod data;
pub use data::*;
pub mod layer;
pub use layer::*;
pub mod optimizer;
pub use optimizer::*;
pub mod grad;
pub use grad::*;
pub mod model;
pub use model::*;
pub mod math;
pub use math::*;
pub mod view;

pub fn cost(model: &Sequential, data: &[Data]) -> f32 {
    let mut loss = 0.0;
    for sample in data {
        let mut erro = 0.0;
        let target = &sample.target;
        let pred = model.forward(&sample.input);
        for i in 0..sample.target.len() {
            erro += (target[i] - pred[i]) * (target[i] - pred[i]);
        }
        loss += erro / (sample.target.len() as f32);
    }
    loss / data.len() as f32
}

#[derive(Debug, Clone)]
pub struct Train {
    pub model: Sequential,
    pub epochs: usize,
    pub lr: f32,
    pub dataset: Vec<Data>,
}
