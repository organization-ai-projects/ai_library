/// Structure pour stocker les métadonnées d’un dataset.
#[derive(Clone, Debug)]
pub struct DatasetMetadata {
    pub labels: Option<Vec<String>>,
    pub split: Option<DatasetSplit>,
    pub description: Option<String>,
    pub source_files: Option<Vec<String>>,
    pub num_samples: Option<usize>,
    pub num_features: Option<usize>,
}

/// Enum pour les splits train/test/val.
#[derive(Clone, Debug)]
pub enum DatasetSplit {
    Train,
    Test,
    Validation,
}
