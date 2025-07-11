# Dossier `preprocessing`

Ce dossier regroupe les fonctions de prétraitement pour chaque type de données :
- `image/` : normalisation, conversion, augmentation, etc.
- `audio/` : normalisation, extraction de features, etc.
- `video/` : normalisation, extraction de frames, etc.
- `text/` : tokenisation, vectorisation, nettoyage, etc.
- `tabular/` : normalisation, imputation, encodage, etc.
- `graph/` : normalisation des poids, extraction de features, etc.
- `timeseries/` : normalisation, lissage, extraction de features.

## Utilisation

```rust
use ai_library::data::preprocessing::image::preprocess_images;
let images_f32 = preprocess_images(&images_u8);
```

## À compléter

- Ajouter de nouveaux prétraitements ou types en créant un fichier et en l’exposant dans le `mod.rs` du sous-dossier concerné.
- Ajouter des exemples pour chaque prétraitement.

## À faire

- Ajouter des benchmarks pour les fonctions de prétraitement.
- Ajouter la gestion des pipelines de prétraitement complexes.
