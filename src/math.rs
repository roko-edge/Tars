pub mod tensor;
use rand::{RngExt, random};
pub use tensor::*;

pub fn random_vec(len: usize) -> Vec<f32> {
    let mut vector = vec![0.0; len];
    for e in &mut vector {
        *e = rand::random::<f32>() * 2.0 - 1.0;
    }
    vector
}

// NOTE:
// This function produces a random value with mean = sqrt(3) * (low + high - 1) and variance = ((low + high)^2) / 12.
// =>  mean(prng(0, 1)) = 0, var(prng(0, 1)) = 1. Exactly what we want to initialize our network.

pub fn prng(low: f32, high: f32) -> f32 {
    let u: f32 = rand::random_range(low..high);
    3.0_f32.sqrt() * (2.0 * u - 1.0)
}

// NOTE:
// This function returns a vector of n random variables variables as (prng(0, 1) / sqrt(n))
// This is nescessary beacuse variance has this prop: var(aX) = a^2var(x)
// => var(X/sqrt(n)) = 1/x * var(X) => var(1/x * var(X)) = 1/n
// The variance of this whole expression should be 1/n according to Xavier Glorot - check [https://arxiv.org/abs/1704.08863]

pub fn xavier(n: usize) -> Vec<f32> {
    (0..n).map(|_| prng(0.0, 1.0) / (n as f32).sqrt()).collect()
}

// NOTE:
// This function returns a random number between from the discrete interval [-1, 0, 1]
// By standard, this strategy generates a distribution with with mean 0 and variance = 2/3.
// The mean is ok, but we need a variance = 1. To fix that, we use the fact that var(a*x) = a^2 *
// var(x). => c^2*(2/3) = 1 => c = sqrt(3/2). It means that we should multi the rng by the factor
// a = sqrt(3/2)

// Basis used rn: (Consider sqrt() as s())
// {-s(3/2), 0, s(3, 2)}

pub fn ternary_rng() -> f32 {
    let mut rng = rand::rng();
    let u = rng.random_range(0..3);

    let scale = (3.0_f32 / 2.0).sqrt();

    match u {
        0 => -scale,
        1 => 0.0,
        _ => scale,
    }
}

pub fn random_mat(lenx: usize, leny: usize) -> Vec<Vec<f32>> {
    let mut mat = vec![vec![0.0; lenx]; leny];
    for l in &mut mat {
        *l = random_vec(lenx);
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
