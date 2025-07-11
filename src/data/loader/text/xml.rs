use std::fs;

/// Charge tous les fichiers XML d’un dossier.
pub fn load_xml_files(folder: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut xmls = Vec::new();
    for entry in fs::read_dir(folder)? {
        let path = entry?.path();
        if path.is_file() && path.extension().map_or(false, |e| e == "xml") {
            let content = std::fs::read_to_string(&path)?;
            xmls.push(content);
        }
    }
    Ok(xmls)
}
