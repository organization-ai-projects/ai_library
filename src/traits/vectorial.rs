/// Trait pour opérations vectorielles.
pub trait Vectorial {
    /// Additionne deux vecteurs.
    fn add(&self, other: &Self) -> Self;
}
