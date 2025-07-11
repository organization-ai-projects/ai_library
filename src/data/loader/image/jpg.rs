use image::io::Reader as ImageReader;
use std::fs;

/// Charge toutes les images JPG d’un dossier.
pub fn load_jpg_images(folder: &str) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    let mut images = Vec::new();
    for entry in fs::read_dir(folder)? {
        let path = entry?.path();
        if path.is_file() && path.extension().map_or(false, |e| e == "jpg") {
            let img = ImageReader::open(&path)?.decode()?.to_luma8();
            images.push(img.into_raw());
        }
    }
    Ok(images)
}
