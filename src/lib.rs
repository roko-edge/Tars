pub mod data;
pub mod export;
pub mod math;
pub mod modules;
pub mod optim;
pub mod view;

pub use data::*;
pub use export::*;
pub use math::*;
pub use modules::*;
pub use optim::*;

#[derive(Debug, Clone)]
pub struct Train {
    pub model: Sequential,
    pub epochs: usize,
    pub lr: f32,
    pub dataset: Vec<Data>,
}
