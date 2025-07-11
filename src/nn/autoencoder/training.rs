use crate::nn::autoencoder::{decoder::Decoder, encoder::Encoder};

pub fn train_autoencoder_step(
    encoder: &mut Encoder,
    decoder: &mut Decoder,
    input: &ndarray::Array1<f32>,
    learning_rate: f32,
) -> f32 {
    // Place ton algo de training ici (forward, backprop, update poids…)
    // Retourne la loss pour tracking
    todo!("train_autoencoder_step à compléter")
}
