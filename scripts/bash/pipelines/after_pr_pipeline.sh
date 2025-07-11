#!/bin/bash
# Post-PR pipeline: sync main, delete dev branch, propose recreation (EN/FR)

# Sélection DRY de la langue
source "$(dirname "$0")/../utils/select_lang.sh"

# Centralisation des messages
if [ "$LANG" = "fr" ]; then
  MSG_SYNC_MISSING="⛔️ Le script sync_main_with_remote.sh est manquant !"
  MSG_DELETE_MISSING="⛔️ Le script safe_delete_and_recreate_branch.sh est manquant !"
  MSG_DONE="✅ Pipeline post-PR terminé."
else
  MSG_SYNC_MISSING="⛔️ sync_main_with_remote.sh script is missing!"
  MSG_DELETE_MISSING="⛔️ safe_delete_and_recreate_branch.sh script is missing!"
  MSG_DONE="✅ Post-PR pipeline completed."
fi
if [ -f ./sync_main_with_remote.sh ]; then
  bash ./sync_main_with_remote.sh
else
  echo "$MSG_SYNC_MISSING"
  exit 1
fi
if [ -f ./safe_delete_and_recreate_branch.sh ]; then
  bash ./safe_delete_and_recreate_branch.sh
else
  echo "$MSG_DELETE_MISSING"
  exit 1
fi
echo "$MSG_DONE"
