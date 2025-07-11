/// Trait pour un système IA.
pub trait System {
    /// Initialise le système.
    fn initialize(&mut self);
    /// Exécute le système.
    fn execute(&self, input: &[f32]) -> Vec<f32>;
}
