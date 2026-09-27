use crate::AnyModule;
use crate::Data;
use crate::Sequential;
use crate::cost;
#[derive(Clone, Debug)]
pub struct Grad {
    pub modules: Vec<AnyModule>,
}

pub fn num_grad(model: &Sequential, data: &[Data]) -> Grad {
    let mut temp_model = model.clone();
    let mut grad = Grad {
        modules: model.modules.clone(),
    };
    let h = 1e-3;
    for m in 0..model.modules.len() {
        match &model.modules[m] {
            AnyModule::Linear(linear) => {
                for o in 0..linear.out_sz {
                    for w in 0..linear.in_sz {
                        temp_model.modules[m].as_mut_linear().weights[o][w] += h;
                        let costp = cost(&temp_model, data);
                        temp_model.modules[m].as_mut_linear().weights[o][w] -= 2.0 * h;
                        let costm = cost(&temp_model, data);
                        grad.modules[m].as_mut_linear().weights[o][w] = (costp - costm) / (2.0 * h);
                        temp_model.modules[m].as_mut_linear().weights[o][w] += h;
                    }

                    temp_model.modules[m].as_mut_linear().bias[o] += h;
                    let costp = cost(&temp_model, data);
                    temp_model.modules[m].as_mut_linear().bias[o] -= 2.0 * h;
                    let costm = cost(&temp_model, data);
                    grad.modules[m].as_mut_linear().bias[o] = (costp - costm) / (2.0 * h);
                    temp_model.modules[m].as_mut_linear().bias[o] += h;
                }
            }
            AnyModule::Activation(_) => {}
        }
    }
    grad
}
