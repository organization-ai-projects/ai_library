#!/bin/bash
# Installe automatiquement tous les hooks personnalisés dans .git/hooks/
HOOKS_DIR="scripts/git_hooks"
GIT_HOOKS=".git/hooks"

if [ ! -d "$HOOKS_DIR" ]; then
  echo "⛔️ Le dossier $HOOKS_DIR n'existe pas. Abandon."
  exit 1
fi

# Sélection DRY de la langue
if [ -f "$HOOKS_DIR/../bash/utils/select_lang.sh" ]; then
  source "$HOOKS_DIR/../bash/utils/select_lang.sh"
else
  LANG="en"
fi

if [ "$LANG" = "fr" ]; then
  MSG_DONE="✅ Tous les hooks personnalisés ont été installés dans .git/hooks/"
else
  MSG_DONE="✅ All custom hooks have been installed in .git/hooks/"
fi

HOOK_FOUND=0
for hook in "$HOOKS_DIR"/*.sh; do
  [ -e "$hook" ] || continue
  HOOK_FOUND=1
  name=$(basename "$hook" .sh)
  chmod +x "$hook"
  cp "$hook" "$GIT_HOOKS/$name"
  chmod +x "$GIT_HOOKS/$name"
  ls -l "$GIT_HOOKS/$name"
done

if [ $HOOK_FOUND -eq 0 ]; then
  echo "⚠️  Aucun hook trouvé dans $HOOKS_DIR/*.sh"
fi

# Vérifie et corrige les droits d’exécution sur tous les hooks installés
for hook in pre-commit pre-push commit-msg post-push; do
  if [ -f "$GIT_HOOKS/$hook" ]; then
    chmod +x "$GIT_HOOKS/$hook"
  fi
done

echo "$MSG_DONE"
