#!/bin/bash
 # Simulate PR workflow without GitHub API (EN/FR)

# Sélection DRY de la langue
source "$(dirname "$0")/../utils/select_lang.sh"

# Centralisation des messages
if [ "$LANG" = "fr" ]; then
  MSG_USAGE="Usage : $0 <branche_source> <nouvelle_branche> <message_commit>"
  MSG_PUSHED="✅ Branche poussée."
  MSG_PR="➡️ Crée maintenant ta Pull Request sur GitHub depuis '$NEW_BRANCH' vers 'main' ou 'dev'."
else
  MSG_USAGE="Usage: $0 <source_branch> <new_branch> <commit_message>"
  MSG_PUSHED="✅ Branch pushed."
  MSG_PR="➡️ Now create your Pull Request on GitHub from '$NEW_BRANCH' to 'main' or 'dev'."
fi
 # 1. Create branch, commit, push
 # 2. Show instructions to create PR on GitHub
if [ "$#" -ne 3 ]; then
  echo "$MSG_USAGE"
  exit 1
fi
SRC_BRANCH="$1"
NEW_BRANCH="$2"
COMMIT_MSG="$3"
bash ../git/branch/create_branch.sh "$SRC_BRANCH" "$NEW_BRANCH"
bash ../git/commit/add_commit.sh "$COMMIT_MSG"
git push origin "$NEW_BRANCH"
echo "$MSG_PUSHED"
echo "$MSG_PR"
