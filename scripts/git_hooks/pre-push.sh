#!/usr/bin/env bash
# Hook pre-push : refuse le push sur main/dev si non passé par PR (EN/FR)

# Sélection DRY de la langue
source "$(git rev-parse --show-toplevel)/scripts/bash/utils/select_lang.sh"

# Centralisation des messages
if [ "$LANG" = "fr" ]; then
  MSG_BRANCH="⛔️ Interdiction de push direct sur main ou dev ! Utilisez une PR."
else
  MSG_BRANCH="⛔️ Direct push to main or dev is forbidden! Use a PR."
fi

BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [[ "$BRANCH" == "main" || "$BRANCH" == "dev" ]]; then
  echo "$MSG_BRANCH"
  exit 1
fi
