/// Structure représentant la RAM.
pub struct Ram {
    pub storage: Vec<f32>,
}

impl Ram {
    /// Lit une valeur à une adresse donnée.
    pub fn read(&self, addr: usize) -> Option<f32> {
        self.storage.get(addr).copied()
    }
    /// Écrit une valeur à une adresse donnée.
    pub fn write(&mut self, addr: usize, value: f32) {
        if addr < self.storage.len() {
            self.storage[addr] = value;
        }
    }
}
