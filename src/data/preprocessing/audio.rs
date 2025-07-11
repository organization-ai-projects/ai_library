//! Fonctions de prétraitement pour les datasets audio.

/// Normalise un vecteur d’audio.
pub fn preprocess_audio(audio: &[Vec<f32>]) -> Vec<Vec<f32>> {
    audio
        .iter()
        .map(|sample| {
            let min = sample.iter().cloned().fold(f32::INFINITY, f32::min);
            let max = sample.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            if max > min {
                sample.iter().map(|&v| (v - min) / (max - min)).collect()
            } else {
                sample.clone()
            }
        })
        .collect()
}
