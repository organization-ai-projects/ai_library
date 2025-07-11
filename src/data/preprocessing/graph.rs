//! Fonctions de prétraitement pour les datasets de graphes.

/// Prétraite un dataset de graphes (normalisation des poids).
pub fn preprocess_graph(graphs: &[Vec<(usize, usize, f32)>]) -> Vec<Vec<(usize, usize, f32>>> {
    graphs.iter()
        .map(|g| {
            let min = g.iter().map(|(_, _, w)| *w).fold(f32::INFINITY, f32::min);
            let max = g.iter().map(|(_, _, w)| *w).fold(f32::NEG_INFINITY, f32::max);
            if max > min {
                g.iter().map(|&(from, to, w)| (from, to, (w - min) / (max - min))).collect()
            } else {
                g.clone()
            }
        })
        .collect()
}
