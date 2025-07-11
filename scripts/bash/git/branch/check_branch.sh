#!/usr/bin/env bash
# Vérifie que la branche passée en argument n'est pas main ou dev
# Usage : check_branch.sh <branch_name>

BRANCH="$1"

if [ -z "$BRANCH" ]; then
  echo "[check_branch] Erreur : aucun nom de branche fourni."
  exit 2
fi

if [[ "$BRANCH" == "main" || "$BRANCH" == "dev" ]]; then
  echo "⛔️ Interdiction d'opérer sur la branche $BRANCH ! Utilisez une branche de feature."
  exit 1
fi

# Si tout est ok
exit 0
