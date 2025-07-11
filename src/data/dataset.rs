//! Module central pour gérer les datasets multi-types et multi-fichiers.

/// Enum représentant les différents types de datasets.
#[derive(Clone, Debug)]
pub enum DatasetType {
    /// Dataset d’images, chaque image est un Vec<u8> (pixels ou raw)
    Image(Vec<Vec<u8>>),
    /// Dataset texte, chaque sample est une String
    Text(Vec<String>),
    /// Dataset séries temporelles, chaque série est un Vec<f32>
    Timeseries(Vec<Vec<f32>>),
    /// Dataset audio, chaque sample est un Vec<f32>
    Audio(Vec<Vec<f32>>),
    /// Dataset vidéo, chaque vidéo est un Vec<u8> (frames ou raw)
    Video(Vec<Vec<u8>>),
    /// Dataset tabulaire, chaque ligne est un Vec<f32>
    Tabular(Vec<Vec<f32>>),
    /// Dataset de graphes, chaque graphe est un Vec<(usize, usize, f32)>
    Graph(Vec<Vec<(usize, usize, f32)>>),
    /// Pour les datasets multi-types
    Mixed(Vec<DatasetType>),
}

/// Structure représentant un dataset multi-fichiers.
pub struct MultiFileDataset {
    pub datasets: Vec<DatasetType>,
    pub metadata: Option<crate::data::metadata::DatasetMetadata>,
}

impl MultiFileDataset {
    /// Ajoute un dataset.
    pub fn add(&mut self, dataset: DatasetType) {
        self.datasets.push(dataset);
    }

    /// Définit les métadonnées du dataset.
    pub fn set_metadata(&mut self, metadata: crate::data::metadata::DatasetMetadata) {
        self.metadata = Some(metadata);
    }

    /// Applique le prétraitement adapté à chaque dataset et retourne le résultat.
    pub fn preprocess_all(&self) -> Vec<DatasetType> {
        self.datasets
            .iter()
            .map(|ds| match ds {
                DatasetType::Image(images) => {
                    DatasetType::Image(crate::data::preprocessing::image::preprocess_images(images))
                }
                DatasetType::Text(texts) => DatasetType::Text(
                    crate::data::preprocessing::text::preprocess_texts(texts)
                        .into_iter()
                        .map(|v| v.iter().map(|f| f.to_string()).collect::<String>())
                        .collect(),
                ),
                DatasetType::Timeseries(series) => DatasetType::Timeseries(
                    crate::data::preprocessing::timeseries::preprocess_timeseries(series),
                ),
                DatasetType::Audio(audio) => {
                    DatasetType::Audio(crate::data::preprocessing::audio::preprocess_audio(audio))
                }
                DatasetType::Video(videos) => DatasetType::Video(
                    crate::data::preprocessing::video::preprocess_video(videos)
                        .into_iter()
                        .map(|v| v.iter().map(|f| (*f * 255.0) as u8).collect())
                        .collect(),
                ),
                DatasetType::Tabular(table) => DatasetType::Tabular(
                    crate::data::preprocessing::tabular::preprocess_tabular(table),
                ),
                DatasetType::Graph(graphs) => {
                    DatasetType::Graph(crate::data::preprocessing::graph::preprocess_graph(graphs))
                }
                DatasetType::Mixed(mixed) => {
                    let processed = mixed
                        .iter()
                        .flat_map(|sub_ds| {
                            MultiFileDataset {
                                datasets: vec![sub_ds.clone()],
                                metadata: None,
                            }
                            .preprocess_all()
                        })
                        .collect();
                    DatasetType::Mixed(processed)
                }
            })
            .collect()
    }
}
