#!/bin/bash

set -e

DEFAULT_MAIN="main"

echo "=== Suppression safe d'une branche git + option recréation ==="
echo "Branche à traiter ? (ex: feature_XYZ)"
read BR

if [ -z "$BR" ]; then
    echo "⛔️ Pas de nom de branche fourni. Annulation."
    exit 1
fi

# Quelle branche principale ? (main/master...)
if git show-ref --quiet refs/heads/main; then
    MAIN="$DEFAULT_MAIN"
elif git show-ref --quiet refs/heads/master; then
    MAIN="master"
else
    echo "⛔️ Impossible de détecter main/master."
    exit 1
fi

git fetch origin

echo "Vérification : commits de \"$BR\" non présents dans origin/$MAIN..."
if git log origin/$MAIN.."$BR" | grep .; then
    echo "⛔️ La branche \"$BR\" n'est PAS mergée dans origin/$MAIN."
    echo "Suppression refusée."
    exit 1
fi

echo "✅ La branche \"$BR\" a bien été mergée dans $MAIN."
echo "Suppression locale..."
git branch -d "$BR" || git branch -D "$BR"

echo "Suppression côté remote (origin)..."
git push origin --delete "$BR"

echo "✔️ Suppression effectuée avec succès !"

echo " "
echo "🔄 Voulez-vous RECRÉER la branche à partir de $MAIN ? (y/n)"
read REP

if [[ "$REP" == "y" || "$REP" == "Y" ]]; then
    echo "Nom de la nouvelle branche (laisser vide pour '$BR', ou entrer un nouveau nom ex: ${BR}_xxx) :"
    read NEW_BR
    if [ -z "$NEW_BR" ]; then
        NEW_BR="$BR"
    fi
    git checkout $MAIN
    git pull origin $MAIN
    git checkout -b $NEW_BR
    echo "✅ Branche '$NEW_BR' recréée à partir de $MAIN."
else
    echo "Fin du script, aucune recréation faite."
fi
