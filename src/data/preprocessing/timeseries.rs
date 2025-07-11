//! Fonctions de prétraitement pour les datasets de séries temporelles.
//!
//! - Normalisation min-max
//! - Lissage
//! - Extraction de features

/// Normalise chaque série temporelle entre 0 et 1.
pub fn preprocess_timeseries(series: &[Vec<f32>]) -> Vec<Vec<f32>> {
    series
        .iter()
        .map(|s| {
            let min = s.iter().cloned().fold(f32::INFINITY, f32::min);
            let max = s.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            if max > min {
                s.iter().map(|&v| (v - min) / (max - min)).collect()
            } else {
                s.clone()
            }
        })
        .collect()
}

/// Applique un lissage par moyenne glissante (window=3) sur chaque série temporelle.
pub fn smooth_timeseries(series: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let window = 3;
    series
        .iter()
        .map(|s| {
            let len = s.len();
            if len == 0 {
                return vec![];
            }
            (0..len)
                .map(|i| {
                    let start = if i >= window - 1 { i + 1 - window } else { 0 };
                    let end = i + 1;
                    let slice = &s[start..end];
                    slice.iter().copied().sum::<f32>() / slice.len() as f32
                })
                .collect()
        })
        .collect()
}

/// Extrait des features simples (moyenne, min, max) pour chaque série temporelle.
pub fn extract_features(series: &[Vec<f32>]) -> Vec<Vec<f32>> {
    series
        .iter()
        .map(|s| {
            if s.is_empty() {
                vec![0.0, 0.0, 0.0]
            } else {
                let mean = s.iter().copied().sum::<f32>() / s.len() as f32;
                let min = s.iter().copied().fold(f32::INFINITY, f32::min);
                let max = s.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                vec![mean, min, max]
            }
        })
        .collect()
}
