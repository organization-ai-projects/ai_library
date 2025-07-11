use opencv::{core, imgcodecs, prelude::*, videoio};
use std::fs;

/// Charge toutes les vidéos MP4 d’un dossier (extrait la première frame).
pub fn load_mp4_videos(folder: &str) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    let mut videos = Vec::new();
    for entry in fs::read_dir(folder)? {
        let path = entry?.path();
        if path.is_file() && path.extension().map_or(false, |e| e == "mp4") {
            let mut cap =
                videoio::VideoCapture::from_file(path.to_str().unwrap(), videoio::CAP_ANY)?;
            let mut frame = core::Mat::default();
            if cap.read(&mut frame)? && !frame.empty()? {
                let mut buf = opencv::types::VectorOfu8::new();
                imgcodecs::imencode(".png", &frame, &mut buf, &core::Vector::new())?;
                videos.push(buf.to_vec());
            }
        }
    }
    Ok(videos)
}
