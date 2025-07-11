use std::fs;

/// Charge tous les fichiers TXT d’un dossier.
pub fn load_txt_files(folder: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut texts = Vec::new();
    for entry in fs::read_dir(folder)? {
        let path = entry?.path();
        if path.is_file() && path.extension().map_or(false, |e| e == "txt") {
            let content = std::fs::read_to_string(&path)?;
            texts.push(content);
        }
    }
    Ok(texts)
}
