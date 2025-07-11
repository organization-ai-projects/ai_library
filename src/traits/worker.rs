/// Trait pour un worker IA.
pub trait Worker {
    /// Exécute une tâche IA.
    fn run(&self, input: &[f32]) -> Vec<f32>;
}
