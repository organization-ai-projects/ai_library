/// Normalise les données entre 0 et 1.
pub fn normalize(data: &[f32]) -> Vec<f32> {
    let min = data.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    if max > min {
        data.iter().map(|&v| (v - min) / (max - min)).collect()
    } else {
        data.to_vec()
    }
}
