#!/usr/bin/env bash
# Hook commit-msg : vérifie le format du message de commit (EN/FR)

# Sélection DRY de la langue
source "$(git rev-parse --show-toplevel)/scripts/bash/utils/select_lang.sh"

# Centralisation des messages
if [ "$LANG" = "fr" ]; then
  MSG_ERROR="⛔️ Le message de commit doit commencer par feat:, fix:, chore:, etc."
else
  MSG_ERROR="⛔️ Commit message must start with feat:, fix:, chore:, etc."
fi

MSG_FILE=$1
MSG=$(cat "$MSG_FILE")
if ! echo "$MSG" | grep -E '^(feat:|fix:|chore:|docs:|refactor:|test:|style:|perf:|ci:|build:|revert:|BREAKING CHANGE:)' > /dev/null; then
  echo "$MSG_ERROR"
  exit 1
fi
