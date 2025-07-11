
#!/bin/bash
# Crée une branche, commit et push (EN/FR)

# Sélection DRY de la langue
source "$(dirname "$0")/../utils/select_lang.sh"

# Centralisation des messages
if [ "$LANG" = "fr" ]; then
  MSG_USAGE="Usage : $0 <branche_source> <nouvelle_branche> <message_commit>"
  MSG_PUSHED="✅ Branche poussée sur le remote."
else
  MSG_USAGE="Usage: $0 <source_branch> <new_branch> <commit_message>"
  MSG_PUSHED="✅ Branch pushed to remote."
fi

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
