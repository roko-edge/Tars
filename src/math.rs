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

pub fn sigmoidf(z: f32) -> f32 {
    1.0 / (1.0 + (-z).exp())
}

pub fn sigmoidf_deriv_from_a(a: f32) -> f32 {
    a * (1.0 - a)
}

pub fn cost_derivative(p: f32, t: f32, out_sz: usize) -> f32 {
    2.0 * (p - t) / (out_sz as f32)
}

pub fn relu(x: f32) -> f32 {
    if x > 0.0 { x } else { 0.0 }
}

pub fn relu_deriv_from_z(z: f32) -> f32 {
    if z > 0.0 { 1.0 } else { 0.0 }
}
