//! Fonctions de prétraitement pour les datasets tabulaires.

/// Prétraite un dataset tabulaire (normalisation min-max).
pub fn preprocess_tabular(table: &[Vec<f32>]) -> Vec<Vec<f32>> {
    table
        .iter()
        .map(|row| {
            let min = row.iter().cloned().fold(f32::INFINITY, f32::min);
            let max = row.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            if max > min {
                row.iter().map(|&v| (v - min) / (max - min)).collect()
            } else {
                row.clone()
            }
        })
        .collect()
}
