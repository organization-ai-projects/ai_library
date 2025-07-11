use std::fs;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct WebmMetadata {
    // Pas d'extraction avancée pour l'instant
}

pub struct WebmLoader;

impl super::video_loader::VideoLoader for WebmLoader {
    type Metadata = WebmMetadata;
    fn format_name(&self) -> &'static str {
        "webm"
    }
    fn load_videos(
        &self,
        folder: &str,
    ) -> Result<Vec<(Vec<u8>, Option<WebmMetadata>)>, Box<dyn std::error::Error>> {
        let mut videos = Vec::new();
        for entry in fs::read_dir(folder)? {
            let path = entry?.path();
            if path.is_file() && path.extension().map_or(false, |e| e == "webm") {
                let mut file = fs::File::open(&path)?;
                let mut buf = Vec::new();
                file.read_to_end(&mut buf)?;
                videos.push((buf, Some(WebmMetadata {})));
            }
        }
        Ok(videos)
    }
}
