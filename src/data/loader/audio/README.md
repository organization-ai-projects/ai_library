# Dossier `audio`

Chargement des fichiers audio selon leur format :
- `wav.rs` : fonction `load_wav_audio`
- (ajouter d’autres formats comme mp3, flac…)

## Utilisation

```rust
use ai_library::data::loader::audio::load_wav_audio;
let audios = load_wav_audio("chemin/vers/dossier")?;
```

## À compléter

- Ajouter d’autres formats (MP3, FLAC…) en créant un fichier et en l’exposant dans `mod.rs`.
- Ajouter des exemples pour chaque format.

## À faire

- Ajouter la gestion des métadonnées audio.
- Ajouter la conversion automatique vers différents taux d’échantillonnage.
