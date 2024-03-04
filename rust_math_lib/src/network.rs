use crate::matrix::Matrix;

pub struct Network {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    bias: Vec<Matrix>,
    data: Vec<Matrix>,
}

impl Network {
    pub fn new(&self, layers: Vec<usize>) -> Network {
        let mut weights = vec![];
    }
}
