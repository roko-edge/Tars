use tars::*;

pub fn train() -> Train {
    Train {
        model: Sequential::new(2).linear(1).sigmoid(),

        epochs: 100_000,
        lr: 20.0,

        dataset: vec![
            Data::new(&[0.0, 0.0], &[0.0]),
            Data::new(&[0.0, 1.0], &[1.0]),
            Data::new(&[1.0, 0.0], &[1.0]),
            Data::new(&[1.0, 1.0], &[1.0]),
        ],
    }
}
