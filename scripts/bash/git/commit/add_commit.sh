#!/bin/bash
# Add all files and commit with a message (EN/FR)

# Sélection DRY de la langue
source "$(dirname "$0")/../utils/select_lang.sh"

# Centralisation des messages
if [ "$LANG" = "fr" ]; then
  MSG_NO_CHANGES="Aucune modification à committer."
  MSG_COMMIT_PROMPT="Message de commit : "
  MSG_COMMIT_DONE="✅ Commit effectué."
else
  MSG_NO_CHANGES="No changes to commit."
  MSG_COMMIT_PROMPT="Commit message: "
  MSG_COMMIT_DONE="✅ Commit done."
fi

git add -A
if git diff --cached --quiet; then
  echo "$MSG_NO_CHANGES"
  exit 0
fi
read -p "$MSG_COMMIT_PROMPT" msg
git commit -m "$msg"
<<<<<<< HEAD
echo "$MSG_COMMIT_DONE"
=======
echo "$MSG_COMMIT_DONE"
>>>>>>> 01a3a24b2c1078c862b77eb7e462fe6d47fe6fc0
