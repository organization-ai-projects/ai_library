#!/bin/bash

read -p "Nom de la branche principale (défaut : main) : " main_branch
if [ -z "$main_branch" ]; then
    main_branch="main"
fi
git checkout "$main_branch"
git pull origin "$main_branch"
echo "✅ Branche principale locale synchronisée avec le distant."
