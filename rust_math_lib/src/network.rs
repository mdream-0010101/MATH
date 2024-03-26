use crate::matrix::Matrix;

pub struct Network {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    bias: Vec<Matrix>,
    data: Vec<Matrix>,
}

impl Network {
    pub fn new(layers: Vec<usize>) -> Network {
        let weights = Vec::new();

        Network {
            layers,
            weights,
            bias: Vec::new(),
            data: Vec::new(),
        }
    }
}
