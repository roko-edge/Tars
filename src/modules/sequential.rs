use crate::Activation;
use crate::AnyModule;
use crate::Linear;
use crate::Module;

#[derive(Clone, Debug)]
pub struct Sequential {
    pub modules: Vec<AnyModule>,
    input_size: usize,
}

impl Sequential {
    pub fn new(input_size: usize) -> Self {
        Self {
            modules: Vec::new(),
            input_size,
        }
    }

    pub fn linear(mut self, output_size: usize) -> Self {
        let linear = Linear::random(self.input_size, output_size);
        self.modules.push(AnyModule::Linear(linear));
        self.input_size = output_size;
        self
    }

    pub fn relu(mut self) -> Self {
        //TODO: apply random function in previous linear
        self.modules.push(AnyModule::Activation(Activation::Relu));
        self
    }

    pub fn sigmoid(mut self) -> Self {
        //TODO: apply random function in previou:s linear
        self.modules
            .push(AnyModule::Activation(Activation::Sigmoid));
        self
    }
}

impl Module for Sequential {
    fn forward(&self, input: &[f32]) -> Vec<f32> {
        let mut output = input.to_vec();
        for module in &self.modules {
            output = module.forward(&output);
        }
        output
    }
}
