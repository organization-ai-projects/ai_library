//! Fonctions de prétraitement pour les datasets d’images.
//!
//! - Conversion u8 -> f32
//! - Normalisation [0,1]
//! - Redimensionnement (à venir)
//! - Augmentation de données (à venir)

/// Convertit et normalise un vecteur d’images (u8 -> f32, [0,1]).
/// Chaque image est un Vec<u8> représentant les pixels.
pub fn preprocess_images(images: &[Vec<u8>]) -> Vec<Vec<f32>> {
    // Conversion simple u8 -> f32 normalisé
    images
        .iter()
        .map(|img| img.iter().map(|&px| px as f32 / 255.0).collect())
        .collect()
}

/// Redimensionne toutes les images à une taille cible (stub).

/// Redimensionne toutes les images à une taille cible (mock : crop ou pad à la taille demandée).
pub fn resize_images(
    images: &[Vec<u8>],
    target_width: usize,
    target_height: usize,
) -> Vec<Vec<u8>> {
    // Mock : on tronque ou complète chaque image à target_width * target_height
    let target_size = target_width * target_height;
    images
        .iter()
        .map(|img| {
            let mut out = img.clone();
            out.resize(target_size, 0);
            out.truncate(target_size);
            out
        })
        .collect()
}

/// Applique une augmentation de données sur les images (stub).

/// Applique une augmentation de données simple : flip horizontal.
pub fn augment_images(images: &[Vec<u8>]) -> Vec<Vec<u8>> {
    images
        .iter()
        .map(|img| {
            let mut out = img.clone();
            out.reverse();
            out
        })
        .collect()
}
