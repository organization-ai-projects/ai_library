//! Fonctions de prétraitement pour les datasets vidéo.

/// Prétraite un vecteur de vidéos (normalisation u8 -> f32).
pub fn preprocess_video(videos: &[Vec<u8>]) -> Vec<Vec<f32>> {
    videos
        .iter()
        .map(|vid| vid.iter().map(|&b| b as f32 / 255.0).collect())
        .collect()
}
