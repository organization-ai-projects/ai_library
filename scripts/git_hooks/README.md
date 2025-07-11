# Git Hooks personnalisés

Placez ces scripts dans `.git/hooks/` et rendez-les exécutables (`chmod +x`).

- **pre-commit.sh** : Bloque les commits sur main/dev et impose un message de commit conventionné.
- **pre-push.sh** : Bloque le push direct sur main/dev.

Pour installer un hook :
```bash
cp scripts/git_hooks/pre-commit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```
