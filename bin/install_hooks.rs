use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[cfg(unix)]
fn make_executable(path: &Path) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms)
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) -> io::Result<()> {
    Ok(()) // Pas de chmod sous Windows, mais la copie suffit
}

fn main() -> io::Result<()> {
    let lang = env::var("LANG").unwrap_or_else(|_| "en".to_string());
    let hooks_dir = Path::new("scripts/git_hooks");
    let git_hooks = Path::new(".git/hooks");

    // Crée le dossier .git/hooks si besoin
    if !git_hooks.exists() {
        fs::create_dir_all(git_hooks)?;
    }

    for entry in fs::read_dir(hooks_dir)? {
        let entry = entry?;
        let path = entry.path();
        // On ne prend que les scripts .sh (ex : pre-commit.sh)
        if path.extension().map_or(false, |ext| ext == "sh") {
            let name = path.file_stem().unwrap(); // "pre-commit"
            let dest = git_hooks.join(name);      // .git/hooks/pre-commit (SANS .sh)
            // Lecture, conversion en LF, écriture
            let content = fs::read_to_string(&path)?
                .replace("\r\n", "\n");
            fs::write(&dest, content)?;
            make_executable(&dest)?;
            println!("✅ Copié: {} -> {}", path.display(), dest.display());
        }
    }

    if lang.starts_with("fr") {
        println!("✅ Tous les hooks personnalisés (sans extension .sh) ont été installés dans .git/hooks/");
    } else {
        println!("✅ All custom hooks (without .sh) have been installed in .git/hooks/");
    }
    Ok(())
}
