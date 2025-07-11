/// Structure représentant un cache mémoire.
pub struct MemoryCache {
    pub cache: std::collections::HashMap<usize, f32>,
}

impl MemoryCache {
    /// Récupère une valeur du cache.
    pub fn get(&self, addr: usize) -> Option<f32> {
        self.cache.get(&addr).copied()
    }
    /// Insère une valeur dans le cache.
    pub fn insert(&mut self, addr: usize, value: f32) {
        self.cache.insert(addr, value);
    }
}
