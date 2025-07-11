# Dossier `data`

Ce dossier gère toute la préparation des datasets pour l’IA :
- Chargement de tous les formats courants (CSV, JSON, images, audio, vidéo, etc.)
- Prétraitement, validation, batching, streaming, interopérabilité
- Gestion des métadonnées et structure centralisée

## Modules exposés

- `loader/` : chargement des fichiers/dossiers selon le type et le format
- `preprocessing/` : fonctions de prétraitement pour chaque type
- `batching.rs` : découpage en batchs et sampling
- `validation.rs` : contrôle qualité des données
- `interop.rs` : conversion vers formats externes (ndarray, tch, etc.)
- `metadata.rs` : gestion des labels, splits, description
- `dataset.rs` : structure centralisée multi-type/multi-fichier

## Utilisation

Voir chaque README de sous-dossier pour les détails d’utilisation.

## À compléter

- Ajouter des exemples d’utilisation pour chaque module.
- Documenter les structures et fonctions publiques.

## À faire

- Ajouter des tests unitaires pour chaque module.
- Ajouter des benchmarks pour les fonctions critiques.
