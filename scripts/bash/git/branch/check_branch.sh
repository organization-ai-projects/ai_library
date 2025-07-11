#!/usr/bin/env bash
# Vérifie que la branche passée en argument n'est pas main ou dev
# Usage : check_branch.sh <branch_name>

BRANCH="$1"

if [ -z "$BRANCH" ]; then
  echo "[check_branch] Erreur : aucun nom de branche fourni."
  exit 2
fi

if [[ "$BRANCH" == "main" || "$BRANCH" == "dev" ]]; then
  # Autorise le merge via PR en CI (GitHub Actions)
  if [ -n "$GITHUB_ACTIONS" ]; then
    echo "[check_branch] Merge PR autorisé sur $BRANCH en CI."
    exit 0
  else
    echo "⛔️ Interdiction d'opérer sur la branche $BRANCH ! Utilisez une branche de feature."
    exit 1
  fi
fi

# Si tout est ok
exit 0
