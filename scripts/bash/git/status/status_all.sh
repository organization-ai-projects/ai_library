#!/bin/bash
# Show status of all local and remote branches (EN/FR)

# Sélection DRY de la langue
source "$(dirname "$0")/../utils/select_lang.sh"

# Centralisation des messages
if [ "$LANG" = "fr" ]; then
  MSG_BRANCHES="Branches locales et distantes :"
  MSG_STATUS="Statut du dépôt :"
else
  MSG_BRANCHES="Local and remote branches:"
  MSG_STATUS="Repository status:"
fi
git fetch --all
echo "$MSG_BRANCHES"
git branch -a
echo "$MSG_STATUS"
git status
