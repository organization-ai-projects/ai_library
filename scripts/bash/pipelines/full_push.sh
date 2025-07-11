#!/bin/bash
# Add, commit et push sur la branche courante (EN/FR)

source "$(dirname "$0")/../utils/select_lang.sh"


if [ -f ./scripts/bash/git/commit/add_commit.sh ]; then
  bash ./scripts/bash/git/commit/add_commit.sh
else
  if [ "$LANG" = "fr" ]; then
    echo "⛔️ Fichier add_commit.sh introuvable. Arrêt du script."
  else
    echo "⛔️ add_commit.sh file not found. Stopping script."
  fi
  exit 1
fi
git push origin HEAD

if [ "$LANG" = "fr" ]; then
  echo "✅ Commit et push effectués sur la branche courante."
else
  echo "✅ Commit and push done on current branch."
fi