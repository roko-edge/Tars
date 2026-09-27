#[derive(Clone, Debug)]
pub struct Data {
    pub input: Vec<f32>,
    pub target: Vec<f32>,
}
impl Data {
    pub fn new(input: &[f32], target: &[f32]) -> Self {
        Self {
            input: input.to_vec(),
            target: target.to_vec(),
        }
    }
}
