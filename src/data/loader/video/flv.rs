use std::fs;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct FlvMetadata {
    // Pas d'extraction avancée pour l'instant
}

pub struct FlvLoader;

impl super::video_loader::VideoLoader for FlvLoader {
    type Metadata = FlvMetadata;
    fn format_name(&self) -> &'static str {
        "flv"
    }
    fn load_videos(
        &self,
        folder: &str,
    ) -> Result<Vec<(Vec<u8>, Option<FlvMetadata>)>, Box<dyn std::error::Error>> {
        let mut videos = Vec::new();
        for entry in std::fs::read_dir(folder)? {
            let path = entry?.path();
            if path.is_file() && path.extension().map_or(false, |e| e == "flv") {
                let mut file = std::fs::File::open(&path)?;
                let mut buf = Vec::new();
                file.read_to_end(&mut buf)?;
                videos.push((buf, Some(FlvMetadata {})));
            }
        }
        Ok(videos)
    }
}
