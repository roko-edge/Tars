pub fn random_vec(len: usize) -> Vec<f32> {
    let mut vector = vec![0.0; len];
    for e in &mut vector {
        *e = rand::random();
    }
    vector
}
pub fn random_mat(lenx: usize, leny: usize) -> Vec<Vec<f32>> {
    let mut mat = vec![vec![0.0; lenx]; leny];
    for l in &mut mat {
        for c in l {
            *c = rand::random();
        }
    }
    mat
}

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}
pub fn relu(x: f32) -> f32 {
    x.max(0.0)
}
