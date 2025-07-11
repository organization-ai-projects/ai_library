# Guide des meilleures pratiques Git & GitHub (DevOps)

## Règles fondamentales

- **Interdiction de commit direct sur `main` ou `master`** : Toujours passer par une branche de développement et une Pull Request (PR).
- **Avant de créer une nouvelle branche de dev** : Toujours faire `git pull origin main` pour être à jour et éviter les divergences.
- **Pourquoi utiliser `merge` et pas `rebase` ?**
  - `merge` conserve l’historique complet et la traçabilité des intégrations, ce qui est essentiel en équipe.
  - `rebase` réécrit l’historique, ce qui peut poser problème si plusieurs personnes travaillent sur la même branche.
  - En DevOps, la clarté et la traçabilité priment sur l’histoire linéaire.

---

## Workflow recommandé

### 1. Travailler dans ta branche de développement
- Crée une branche de développement (ex : `dev`).
- Effectue tes modifications et commits localement.
- Utilise le script :
  - `./auto_commit.sh` pour ajouter (avec `git add -A`) et committer tes changements.
  - `./auto_push.sh` pour envoyer ta branche sur GitHub.
  - **Ou** `./auto_send_github.sh` pour faire l’ajout, le commit et le push en une seule commande.

### 2. Créer une Pull Request (PR)
- Sur GitHub, crée une PR de ta branche de développement (`dev`) vers la branche principale (`main` ou `master`).
- Attends la revue et la fusion de la PR.

### 3. Après la fusion de la PR
- Utilise le script :
  - `./manager_after_pr_github.sh`
    - Met à jour ta branche principale locale (`main` ou `master`).
    - Synchronise ta branche de développement locale avec la branche principale **en utilisant `merge`** (meilleure pratique DevOps).
    - Pousse la branche de développement synchronisée sur GitHub.

### 4. Recommencer le cycle
- Continue à travailler sur ta branche de développement pour les prochaines évolutions.

---

## Scripts à utiliser

- **auto_commit.sh** : Ajoute et commit les modifications (utilise `git add -A` pour tout prendre en compte).
- **auto_push.sh** : Pousse la branche courante sur GitHub.
- **auto_send_github.sh** : Combine ajout, commit et push en une seule commande.
- **manager_after_pr_github.sh** : Synchronise ta branche de développement après fusion d'une PR (utilise `merge`).
- **safe_delete_and_recreate_branch.sh** : Permet de supprimer une branche locale et distante **seulement si elle a été mergée**, et propose de la recréer proprement à partir de la branche principale.

---

## Scripts DevOps présents

- **auto_commit.sh** : Ajout et commit (meilleure pratique : `git add -A`)
- **auto_push.sh** : Push sur la branche courante
- **auto_send_github.sh** : Commit + push en une commande
- **manager_after_pr_github.sh** : Synchronisation post-PR (merge recommandé)
- **safe_delete_and_recreate_branch.sh** : Suppression sécurisée et recréation de branche après PR

---

## Scripts complémentaires possibles (optionnel)

- **git_status_all.sh** : Affiche le statut de toutes les branches locales et distantes
- **git_cleanup_local.sh** : Supprime toutes les branches locales déjà mergées dans main/master
- **git_stash_save.sh / git_stash_pop.sh** : Sauvegarde et restaure les modifications non committées
- **git_tag_release.sh** : Crée un tag de version et le pousse sur le remote
- **git_log_pr.sh** : Affiche l’historique des PR fusionnées

---

## Conseils DevOps

- Toujours synchroniser ta branche de développement avec la branche principale après chaque fusion de PR.
- Utiliser `merge` pour garder l’historique et faciliter la collaboration.
- Faire des commits clairs et suivre la convention (`feat:`, `fix:`, `chore:`...).
- Supprimer les branches locales inutiles après fusion.

---

## Exemple de cycle complet

1. `git pull origin main` (avant toute nouvelle branche de dev)
2. `git checkout -b dev_ma_feature`
3. Travailler, puis :

## Scripts à utiliser (nouvelle organisation)

- **scripts/bash/git/commit/add_commit.sh** : Ajoute et commit les modifications (utilise `git add -A` pour tout prendre en compte).
- **scripts/bash/git/branch/create_branch.sh** : Crée une nouvelle branche à partir d'une branche source.
- **scripts/bash/git/branch/delete_branch.sh** : Supprime une branche locale et distante si elle a été mergée.
- **scripts/bash/git/status/status_all.sh** : Affiche le statut de toutes les branches locales et distantes.
- **scripts/bash/pipelines/after_pr_pipeline.sh** : Pipeline post-PR (synchro, suppression, recréation).
- **scripts/bash/pipelines/create_and_commit.sh** : Crée une branche puis commit.
- **scripts/bash/pipelines/full_push.sh** : Crée une branche, commit et push.
- **scripts/bash/pipelines/pr_simulation.sh** : Simule le workflow PR sans API GitHub.


## Exemple de cycle complet

1. `git pull origin main` (avant toute nouvelle branche de dev)
2. `scripts/bash/git/branch/create_branch.sh main dev_ma_feature`
3. Travailler, puis :
   - `scripts/bash/git/commit/add_commit.sh`  
   - `git push origin dev_ma_feature`
4. Créer la PR sur GitHub
5. Après fusion : `scripts/bash/pipelines/after_pr_pipeline.sh`


## Nettoyage des branches après PR

Après la fusion et synchronisation, il est recommandé de supprimer les branches de feature/de développement devenues inutiles.  
Utilise le script :

```bash
scripts/bash/git/branch/delete_branch.sh <nom_branche>
```

Ce script vérifie que la branche a bien été mergée avant suppression, supprime localement et sur le remote.


Pour toute nouvelle fonctionnalité, recommence à partir de ta branche de développement.
Pour toute nouvelle fonctionnalité, recommence à partir de ta branche de développement.
