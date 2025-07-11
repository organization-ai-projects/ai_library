/// Structure représentant un neurone spiking.
pub struct SpikingNeuron {
    pub threshold: f32,
    pub potential: f32,
}

impl SpikingNeuron {
    /// Met à jour le potentiel et retourne vrai si le neurone spike.
    pub fn update(&mut self, input: f32) -> bool {
        self.potential += input;
        if self.potential >= self.threshold {
            self.potential = 0.0;
            true
        } else {
            false
        }
    }
}
