# Dossier `text`

Chargement des fichiers texte selon leur format :
- `txt.rs` : fonction `load_txt_files`
- `json.rs` : fonction `load_json_files`
- `xml.rs` : fonction `load_xml_files`

## Utilisation

```rust
use ai_library::data::loader::text::load_txt_files;
let texts = load_txt_files("chemin/vers/dossier")?;
```

## À compléter

- Ajouter d’autres formats (YAML, HTML…) en créant un fichier et en l’exposant dans `mod.rs`.
- Ajouter des exemples pour chaque format.

## À faire

- Ajouter la gestion de l’encodage des fichiers texte.
- Ajouter la détection automatique du format.
