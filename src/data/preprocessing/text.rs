//! Fonctions de prétraitement pour les datasets texte.
//!
//! - Tokenisation
//! - Vectorisation
//! - Nettoyage (à venir)

/// Tokenise et vectorise un vecteur de textes.
/// Ici, chaque mot est représenté par sa longueur (stub).
///
/// # Exemple
/// ```
/// let texts = vec![String::from("hello world")];
/// let vecs = preprocess_texts(&texts);
/// assert_eq!(vecs[0], vec![5.0, 5.0]);
/// ```
pub fn preprocess_texts(texts: &[String]) -> Vec<Vec<f32>> {
    texts
        .iter()
        .map(|txt| txt.split_whitespace().map(|w| w.len() as f32).collect())
        .collect()
}

/// Nettoie le texte (stub).
pub fn clean_texts(_texts: &[String]) -> Vec<String> {
    // À implémenter : suppression ponctuation, lowercasing, etc.
    todo!("clean_texts à compléter")
}

/// Vectorise le texte via embeddings (stub).
pub fn embed_texts(_texts: &[String]) -> Vec<Vec<f32>> {
    // À implémenter : mocker avec random ou utiliser une lib
    todo!("embed_texts à compléter")
}
