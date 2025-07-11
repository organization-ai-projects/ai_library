# 🗣️ Multilingual & DRY Language Selection

All main bash scripts use a centralized mechanism for language selection via `scripts/bash/utils/select_lang.sh`.

**How it works:**
- At the start of each script, the following line is used:
  ```bash
  source "$(dirname "$0")/../utils/select_lang.sh"
  ```
- If the `LANG` variable is not set, the user is prompted in English to choose their language (`en` or `fr`).
- The script then adapts all messages and prompts accordingly.
- To add a new language, simply update `select_lang.sh` and the message blocks in each script.

**Advantages:**
- DRY: No duplicated language selection logic.
- Easy to maintain and extend for new languages.
- Consistent user experience across all scripts.

**Example:**
```bash
#!/bin/bash
source "$(dirname "$0")/../utils/select_lang.sh"
if [ "$LANG" = "fr" ]; then
  echo "Bonjour !"
else
  echo "Hello!"
fi
```
# ai_library

**[Français](README.fr.md)**

> Minimalist, modular and 100% Rust AI library for shared usage.
>
> - Native CPU backend (no ndarray, no torch, pure Vec<f32>).
> - Minimal ND tensor (coming soon).
> - Hooks for CUDA, autograd, graph, etc. (planned).
> - Extensible, open-source API, "torch-like" but fully homemade.

## Example

```rust
use ai_library::hello_ai;

fn main() {
    println!("{}", hello_ai());
}
```

## Quick setup

1. Clone the repo
2. Run `bash install_hooks.sh` to install Git hooks
3. Use scripts in `scripts/bash/` for all Git actions and pipelines

## Best practices
- Never work directly on main/dev
- Always use a feature branch and a PR
- Use hooks and CI to ensure quality

## Script structure
- See `scripts/bash/` for all atomic scripts and pipelines
- See `scripts/git_hooks/` for custom hooks

## Multi-language
This project also provides documentation in French: see `README.fr.md`
