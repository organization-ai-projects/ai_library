#!/bin/bash
# select_lang.sh - Sélectionne la langue et exporte LANG

if [ -z "$LANG" ]; then
  echo -e "Select language: [en/fr] (default: en)"
  read USER_LANG
  case "$USER_LANG" in
    fr|FR|Fr)
      export LANG="fr"
      ;;
    en|EN|En)
      export LANG="en"
      ;;
    *)
      export LANG="en"
      ;;
  esac
fi
