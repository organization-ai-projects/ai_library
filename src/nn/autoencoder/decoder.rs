pub struct Decoder {
    pub weights: ndarray::Array2<f32>,
    pub bias: ndarray::Array1<f32>,
}

impl Decoder {
    pub fn decode(&self, latent: &ndarray::Array1<f32>) -> ndarray::Array1<f32> {
        &self.weights.dot(latent) + &self.bias
    }
}
