# Dossier `tabular`

Chargement des fichiers tabulaires selon leur format :
- `csv.rs` : fonction `load_csv`
- (ajouter d’autres formats comme xls, parquet…)

## Utilisation

```rust
use ai_library::data::loader::tabular::load_csv;
let table = load_csv("chemin/vers/fichier.csv")?;
```

## À compléter

- Ajouter d’autres formats (XLS, Parquet…) en créant un fichier et en l’exposant dans `mod.rs`.
- Ajouter des exemples pour chaque format.

## À faire

- Ajouter la gestion des types de colonnes.
- Ajouter la gestion des valeurs manquantes.
