# 🗣️ Sélection de la langue centralisée (DRY)

Tous les scripts bash principaux utilisent un mécanisme centralisé pour la sélection de la langue via `scripts/bash/utils/select_lang.sh`.

**Fonctionnement :**
- Au début de chaque script, on utilise :
  ```bash
  source "$(dirname "$0")/../utils/select_lang.sh"
  ```
- Si la variable `LANG` n’est pas définie, l’utilisateur est invité (en anglais) à choisir sa langue (`en` ou `fr`).
- Le script adapte alors tous les messages et prompts.
- Pour ajouter une nouvelle langue, il suffit de mettre à jour `select_lang.sh` et les blocs de messages dans chaque script.

**Avantages :**
- DRY : aucune duplication de la logique de sélection de langue.
- Facile à maintenir et à étendre pour de nouvelles langues.
- Expérience utilisateur cohérente dans tous les scripts.

**Exemple :**
```bash
#!/bin/bash
source "$(dirname "$0")/../utils/select_lang.sh"
if [ "$LANG" = "fr" ]; then
  echo "Bonjour !"
else
  echo "Hello!"
fi
```

# ai_library

> Librairie AI minimaliste, modulaire et 100% Rust pour usage partagé.
>
> - Backend CPU natif (pas ndarray, pas torch, pur Vec<f32>).
> - Tensor ND minimal (à venir).
> - Hooks pour CUDA, autograd, graph, etc. (prévu).
> - API extensible, open-source, "torch-like" mais full maison.

## Exemple

```rust
use ai_library::hello_ai;

fn main() {
    println!("{}", hello_ai());
}
```

## Installation rapide

1. Clone le repo
2. Lance `bash install_hooks.sh` pour installer les hooks Git
3. Utilise les scripts dans `scripts/bash/` pour toutes les actions Git et pipelines

## Bonnes pratiques
- Ne jamais travailler sur main/dev directement
- Toujours passer par une branche de feature et une PR
- Utilise les hooks et le CI pour garantir la qualité

## Structure des scripts
- Voir `scripts/bash/` pour tous les scripts atomiques et pipelines
- Voir `scripts/git_hooks/` pour les hooks personnalisés

## Multi-langue
Ce projet propose aussi une documentation en anglais : voir `README.en.md`
