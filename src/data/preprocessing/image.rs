//! Fonctions de prétraitement pour les datasets d’images.
//!
//! - Conversion u8 -> f32
//! - Normalisation [0,1]
//! - Redimensionnement (à venir)
//! - Augmentation de données (à venir)

/// Convertit et normalise un vecteur d’images (u8 -> f32, [0,1]).
/// Chaque image est un Vec<u8> représentant les pixels.
///
/// # Exemple
/// ```
/// let images = vec![vec![0, 128, 255]];
/// let norm = preprocess_images(&images);
/// assert_eq!(norm[0][0], 0.0);
/// assert_eq!(norm[0][2], 1.0);
/// ```
pub fn preprocess_images(images: &[Vec<u8>]) -> Vec<Vec<f32>> {
    // Conversion simple u8 -> f32 normalisé
    images
        .iter()
        .map(|img| img.iter().map(|&px| px as f32 / 255.0).collect())
        .collect()
}

/// Redimensionne toutes les images à une taille cible (stub).
pub fn resize_images(
    _images: &[Vec<u8>],
    _target_width: usize,
    _target_height: usize,
) -> Vec<Vec<u8>> {
    // À implémenter : utiliser une lib d’image ou mocker
    todo!("resize_images à compléter")
}

/// Applique une augmentation de données sur les images (stub).
pub fn augment_images(_images: &[Vec<u8>]) -> Vec<Vec<u8>> {
    // À implémenter : flip, rotation, etc.
    todo!("augment_images à compléter")
}
