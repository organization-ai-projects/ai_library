/// Trait commun pour tous les loaders vidéo
pub trait VideoLoader {
    type Metadata: std::fmt::Debug + Clone;
    fn format_name(&self) -> &'static str;
    fn load_videos(&self, folder: &str) -> Result<Vec<(Vec<u8>, Option<Self::Metadata>)>, Box<dyn std::error::Error>>;
}

/// Exemple d'utilisation :
/// let loader = Mp4Loader;
/// let videos = loader.load_videos("./videos");
