# Dossier `graph`

Chargement des graphes selon leur format :
- `edge_list.rs` : fonction `load_edge_list_graphs`
- (ajouter d’autres formats comme adjacency_matrix…)

## Utilisation

```rust
use ai_library::data::loader::graph::load_edge_list_graphs;
let graphs = load_edge_list_graphs("chemin/vers/fichier.txt")?;
```

## À compléter

- Ajouter d’autres formats (adjacency matrix…) en créant un fichier et en l’exposant dans `mod.rs`.
- Ajouter des exemples pour chaque format.

## À faire

- Ajouter la gestion des attributs de noeuds et d’arêtes.
- Ajouter la conversion entre formats de graphes.
