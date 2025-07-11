# Dossier `video`

Chargement des vidéos selon leur format :
- `mp4.rs` : fonction `load_mp4_videos`
- (ajouter d’autres formats comme avi, mov…)

## Utilisation

```rust
use ai_library::data::loader::video::load_mp4_videos;
let videos = load_mp4_videos("chemin/vers/dossier")?;
```

## À compléter

- Ajouter d’autres formats (AVI, MOV…) en créant un fichier et en l’exposant dans `mod.rs`.
- Ajouter des exemples pour chaque format.

## À faire

- Ajouter l’extraction de plusieurs frames par vidéo.
- Ajouter la gestion des métadonnées vidéo.
