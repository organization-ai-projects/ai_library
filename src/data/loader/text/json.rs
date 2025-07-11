use std::fs;
use serde_json::Value;

/// Charge tous les fichiers JSON d’un dossier.
pub fn load_json_files(folder: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let mut jsons = Vec::new();
    for entry in fs::read_dir(folder)? {
        let path = entry?.path();
        if path.is_file() && path.extension().map_or(false, |e| e == "json") {
            let content = std::fs::read_to_string(&path)?;
            let json: Value = serde_json::from_str(&content)?;
            jsons.push(json);
        }
    }
    Ok(jsons)
}
