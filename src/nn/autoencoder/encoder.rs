pub struct Encoder {
    pub weights: ndarray::Array2<f32>,
    pub bias: ndarray::Array1<f32>,
}

impl Encoder {
    pub fn encode(&self, input: &ndarray::Array1<f32>) -> ndarray::Array1<f32> {
        &self.weights.dot(input) + &self.bias
    }
}
