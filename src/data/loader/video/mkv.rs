use std::fs;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct MkvMetadata {
    // Pas d'extraction avancée pour l'instant
}

pub struct MkvLoader;

impl super::video_loader::VideoLoader for MkvLoader {
    type Metadata = MkvMetadata;
    fn format_name(&self) -> &'static str {
        "mkv"
    }
    fn load_videos(
        &self,
        folder: &str,
    ) -> Result<Vec<(Vec<u8>, Option<MkvMetadata>)>, Box<dyn std::error::Error>> {
        let mut videos = Vec::new();
        for entry in std::fs::read_dir(folder)? {
            let path = entry?.path();
            if path.is_file() && path.extension().map_or(false, |e| e == "mkv") {
                let mut file = std::fs::File::open(&path)?;
                let mut buf = Vec::new();
                file.read_to_end(&mut buf)?;
                videos.push((buf, Some(MkvMetadata {})));
            }
        }
        Ok(videos)
    }
}
