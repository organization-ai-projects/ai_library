# Dossier `loader`

Ce dossier gère le chargement des datasets selon leur type et format :
- Organisation par sous-dossier : `image/`, `audio/`, `video/`, `text/`, `tabular/`, `graph/`
- Chaque sous-dossier expose les fonctions pour charger chaque format (ex : PNG, JPG, WAV, MP4, CSV…)

## Utilisation

```rust
use ai_library::data::loader::image::load_png_images;
let images = load_png_images("chemin/vers/dossier")?;
```

## À compléter

- Ajouter de nouveaux formats en créant un fichier et en l’exposant dans le `mod.rs` du sous-dossier concerné.
- Ajouter des exemples pour chaque format.

## À faire

- Ajouter la gestion des erreurs avancée.
- Ajouter la détection automatique du format.
