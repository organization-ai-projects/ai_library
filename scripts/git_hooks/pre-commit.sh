#!/usr/bin/env bash
# Hook pre-commit : refuse les commits sur main/dev et vérifie la présence d'un message (EN/FR)

# Sélection DRY de la langue
source "$(git rev-parse --show-toplevel)/scripts/bash/utils/select_lang.sh"

# Centralisation des messages
if [ "$LANG" = "fr" ]; then
  MSG_BRANCH="⛔️ Interdiction de commit direct sur main ou dev ! Utilisez une branche de feature."
else
  MSG_BRANCH="⛔️ Direct commit to main or dev is forbidden! Use a feature branch."
fi

BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [[ "$BRANCH" == "main" || "$BRANCH" == "dev" ]]; then
  echo "$MSG_BRANCH"
  exit 1
fi
