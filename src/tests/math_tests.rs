use crate::math::*;

// NOTE:
// The highest precision we've got was:
// mean => 3 points precision,
// variation => 8 points precision.

#[test]
fn prng_test() {
    let n = 100000;

    let values: Vec<f32> = (0..n).map(|_| prng(0.0, 1.0)).collect();

    let mean = values.iter().sum::<f32>() / n as f32;

    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / n as f32;

    assert!(mean.abs() < 0.02);

    assert!((variance - 1.0).abs() < 0.02);

    let limit = 3.0_f32.sqrt();

    assert!(values.iter().all(|x| *x >= -limit && *x < limit));
}

#[test]
fn xavier_test() {
    let n = 100000;

    let values = xavier(n);

    let mean = values.iter().sum::<f32>() / n as f32;

    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / n as f32;

    let expected_variance = 1.0 / n as f32;

    let limit = (3.0 / n as f32).sqrt();

    assert_eq!(values.len(), n);

    assert!(mean.abs() < 0.01);

    assert!((variance - expected_variance).abs() < expected_variance * 0.05);

    assert!(values.iter().all(|x| *x >= -limit && *x < limit));
}
