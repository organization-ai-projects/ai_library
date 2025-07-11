//! Fonctions de prétraitement pour les datasets de séries temporelles.
//!
//! - Normalisation min-max
//! - Lissage (à venir)
//! - Extraction de features (à venir)

/// Normalise chaque série temporelle entre 0 et 1.
///
/// # Exemple
/// ```
/// let series = vec![vec![1.0, 2.0, 3.0]];
/// let norm = preprocess_timeseries(&series);
/// assert_eq!(norm[0][0], 0.0);
/// assert_eq!(norm[0][2], 1.0);
/// ```
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

/// Applique un lissage sur les séries temporelles (stub).
pub fn smooth_timeseries(_series: &[Vec<f32>]) -> Vec<Vec<f32>> {
    // À implémenter : moyenne glissante, etc.
    todo!("smooth_timeseries à compléter")
}

/// Extrait des features des séries temporelles (stub).
pub fn extract_features(_series: &[Vec<f32>]) -> Vec<Vec<f32>> {
    // À implémenter : mocker ou utiliser une lib
    todo!("extract_features à compléter")
}
