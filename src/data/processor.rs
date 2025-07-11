//ai_library/src/data/processor.rs
pub mod image;
pub mod text;
pub mod timeseries;

/// Applique un pré-traitement générique sur les données (normalisation, filtrage, etc.).
pub fn preprocess_data(data: &[f32]) -> Vec<f32> {
    // Exemple d'utilisation du module normalization
    super::normalization::normalize(data)
}

/// Prétraite un dataset image.
pub fn preprocess_image_dataset(images: &[Vec<u8>]) -> Vec<Vec<f32>> {
    image::preprocess_images(images)
}

/// Prétraite un dataset texte.
pub fn preprocess_text_dataset(texts: &[String]) -> Vec<Vec<f32>> {
    text::preprocess_texts(texts)
}

/// Prétraite un dataset de séries temporelles.
pub fn preprocess_timeseries_dataset(series: &[Vec<f32>]) -> Vec<Vec<f32>> {
    timeseries::preprocess_timeseries(series)
}
