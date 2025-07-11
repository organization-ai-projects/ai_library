#!/bin/bash
# Create a new branch from a source branch (EN/FR)

# Sélection DRY de la langue
source "$(dirname "$0")/../utils/select_lang.sh"

# Centralisation des messages
if [ "$LANG" = "fr" ]; then
  MSG_USAGE="Usage : $0 <branche_source> <nouvelle_branche>"
  MSG_SRC_NOT_FOUND="Branche source '$SRC_BRANCH' non trouvée, tentative de fetch..."
  MSG_SRC_SWITCH_FAIL="⛔️ Impossible de basculer sur '$SRC_BRANCH'."
else
  MSG_USAGE="Usage: $0 <source_branch> <new_branch>"
  MSG_SRC_NOT_FOUND="Source branch '$SRC_BRANCH' not found locally, trying to fetch..."
  MSG_SRC_SWITCH_FAIL="⛔️ Unable to switch to '$SRC_BRANCH'."
fi
if [ "$#" -ne 2 ]; then
  echo "$MSG_USAGE"
  exit 1
fi
SRC_BRANCH="$1"
NEW_BRANCH="$2"
if ! git show-ref --verify --quiet refs/heads/"$SRC_BRANCH"; then
  echo "$MSG_SRC_NOT_FOUND"
  git fetch origin "$SRC_BRANCH":"$SRC_BRANCH"
fi
if ! git checkout "$SRC_BRANCH"; then
  echo "$MSG_SRC_SWITCH_FAIL"
  exit 1
fi
if git show-ref --verify --quiet refs/heads/"$NEW_BRANCH"; then
  if [ "$LANG" = "fr" ]; then
    echo "⛔️ La branche '$NEW_BRANCH' existe déjà !"
  else
    echo "⛔️ Branch '$NEW_BRANCH' already exists!"
  fi
  exit 1
fi
git checkout -b "$NEW_BRANCH"
if [ "$LANG" = "fr" ]; then
  echo "✅ Branche '$NEW_BRANCH' créée à partir de '$SRC_BRANCH'."
else
  echo "✅ Branch '$NEW_BRANCH' created from '$SRC_BRANCH'."
fi
