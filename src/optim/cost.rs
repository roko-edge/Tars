use crate::data::Data;
use crate::modules::Module;
use crate::sequential::Sequential;
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
