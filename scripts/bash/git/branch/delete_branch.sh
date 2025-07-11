#!/bin/bash
# Delete a local and remote branch if merged (EN/FR)

# Sélection automatique de la langue si non définie


# Sélection DRY de la langue
source "$(dirname "$0")/../utils/select_lang.sh"

# Centralisation des messages
if [ "$LANG" = "fr" ]; then
  MSG_NO_BRANCH="⛔️ Pas de nom de branche fourni. Annulation."
  MSG_NOT_MERGED="⛔️ La branche $BR n'est PAS mergée dans origin/main. Suppression refusée."
  MSG_DELETED="✔️ Branche $BR supprimée localement et sur le remote."
else
  MSG_NO_BRANCH="⛔️ No branch name provided. Aborting."
  MSG_NOT_MERGED="⛔️ Branch $BR is NOT merged into origin/main. Deletion refused."
  MSG_DELETED="✔️ Branch $BR deleted locally and on remote."
fi

BR="$1"
if [ -z "$BR" ]; then
  echo "$MSG_NO_BRANCH"
  exit 1
fi
git fetch origin
if git log origin/main..$BR | grep .; then
  echo "$MSG_NOT_MERGED"
  exit 1
fi
git branch -d $BR || git branch -D $BR
git push origin --delete $BR
echo "$MSG_DELETED"
