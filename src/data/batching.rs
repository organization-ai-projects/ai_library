/// Découpe un dataset en batchs de taille fixe.
pub fn create_batches<T: Clone>(data: &[T], batch_size: usize) -> Vec<Vec<T>> {
    data.chunks(batch_size).map(|c| c.to_vec()).collect()
}

/// Sample aléatoire d’un batch.
pub fn random_batch<T: Clone>(data: &[T], batch_size: usize) -> Vec<T> {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    data.choose_multiple(&mut rng, batch_size)
        .cloned()
        .collect()
}
