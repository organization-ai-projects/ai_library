#!/usr/bin/env bash
# Hook post-push : affiche un récapitulatif et propose d'automatiser des actions post-push (EN/FR)

# Sélection DRY de la langue
source "$(git rev-parse --show-toplevel)/scripts/bash/utils/select_lang.sh"

# Centralisation des messages
if [ "$LANG" = "fr" ]; then
  MSG_PUSH="✅ Push effectué !"
  MSG_RECAP="Récapitulatif :"
  MSG_NEXT="Tu peux maintenant :"
  MSG_PR="- Créer une Pull Request sur GitHub si besoin."
  MSG_PIPELINE="- Lancer le pipeline post-PR : scripts/bash/pipelines/after_pr_pipeline.sh"
  MSG_STATUS="- Vérifier le statut des branches : scripts/bash/git/status/status_all.sh"
else
  MSG_PUSH="✅ Push done!"
  MSG_RECAP="Recap:"
  MSG_NEXT="You can now:"
  MSG_PR="- Create a Pull Request on GitHub if needed."
  MSG_PIPELINE="- Run the post-PR pipeline: scripts/bash/pipelines/after_pr_pipeline.sh"
  MSG_STATUS="- Check branch status: scripts/bash/git/status/status_all.sh"
fi

echo "$MSG_PUSH"
echo "$MSG_RECAP"
git log -1 --oneline
echo
echo "$MSG_NEXT"
echo "$MSG_PR"
echo "$MSG_PIPELINE"
echo "$MSG_STATUS"
