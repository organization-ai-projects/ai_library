//! Fonctions de prétraitement pour les datasets texte.
//!
//! - Tokenisation
//! - Vectorisation
//! - Nettoyage

/// Tokenise et vectorise un vecteur de textes.
/// Ici, chaque mot est représenté par sa longueur (stub).

pub fn preprocess_texts(texts: &[String]) -> Vec<Vec<f32>> {
    texts
        .iter()
        .map(|txt| txt.split_whitespace().map(|w| w.len() as f32).collect())
        .collect()
}

/// Nettoie le texte : suppression de la ponctuation et passage en minuscules.
pub fn clean_texts(texts: &[String]) -> Vec<String> {
    texts
        .iter()
        .map(|txt| {
            txt.to_lowercase()
                .replace(|c: char| !c.is_alphanumeric() && !c.is_whitespace(), "")
        })
        .collect()
}

/// Vectorise le texte via embeddings (mock : somme des longueurs des mots).
pub fn embed_texts(texts: &[String]) -> Vec<Vec<f32>> {
    texts
        .iter()
        .map(|txt| {
            let sum = txt.split_whitespace().map(|w| w.len() as f32).sum();
            vec![sum]
        })
        .collect()
}
