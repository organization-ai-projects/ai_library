/// Iterator pour streamer un dataset volumineux.
pub struct DatasetStreamer<T> {
    pub source: Box<dyn Iterator<Item = T>>,
}

impl<T> Iterator for DatasetStreamer<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.source.next()
    }
}

impl<T> DatasetStreamer<T> {
    /// Crée un streamer à partir d’un vecteur.
    pub fn from_vec(data: Vec<T>) -> Self
    where
        T: 'static,
    {
        DatasetStreamer {
            source: Box::new(data.into_iter()),
        }
    }
}
