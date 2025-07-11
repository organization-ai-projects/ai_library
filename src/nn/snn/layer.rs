use super::neuron::SpikingNeuron;

/// Structure représentant une couche de neurones spiking.
pub struct SpikingLayer {
    pub neurons: Vec<SpikingNeuron>,
}

impl SpikingLayer {
    /// Met à jour tous les neurones de la couche.
    pub fn forward(&mut self, inputs: &[f32]) -> Vec<bool> {
        self.neurons
            .iter_mut()
            .zip(inputs.iter())
            .map(|(n, &inp)| n.update(inp))
            .collect()
    }
}
