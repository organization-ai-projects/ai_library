# Dossier `image`

Chargement des images selon leur format :
- `png.rs` : fonction `load_png_images`
- `jpg.rs` : fonction `load_jpg_images`
- (ajouter d’autres formats si besoin)

## Utilisation

```rust
use ai_library::data::loader::image::load_png_images;
let images = load_png_images("chemin/vers/dossier")?;
```

## À compléter

- Ajouter d’autres formats (BMP, TIFF…) en créant un fichier et en l’exposant dans `mod.rs`.
- Ajouter des exemples pour chaque format.

## À faire

- Ajouter la gestion des métadonnées d’image.
- Ajouter la conversion automatique vers différents types (RGB, grayscale…).
