/// Vérifie la cohérence d’un dataset (valeurs manquantes, etc.).
pub fn validate_dataset<T: AsRef<[f32]>>(data: &[T]) -> bool {
    if data.is_empty() {
        return false;
    }
    let len = data[0].as_ref().len();
    data.iter()
        .all(|row| row.as_ref().len() == len && row.as_ref().iter().all(|v| !v.is_nan()))
}
