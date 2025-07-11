use std::fs;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct MpegMetadata {
    // Pas d'extraction avancée pour l'instant
}

pub struct MpegLoader;

impl super::video_loader::VideoLoader for MpegLoader {
    type Metadata = MpegMetadata;
    fn format_name(&self) -> &'static str {
        "mpeg/mpg"
    }
    fn load_videos(
        &self,
        folder: &str,
    ) -> Result<Vec<(Vec<u8>, Option<MpegMetadata>)>, Box<dyn std::error::Error>> {
        let mut videos = Vec::new();
        for entry in std::fs::read_dir(folder)? {
            let path = entry?.path();
            if path.is_file()
                && (path.extension().map_or(false, |e| e == "mpeg")
                    || path.extension().map_or(false, |e| e == "mpg"))
            {
                let mut file = std::fs::File::open(&path)?;
                let mut buf = Vec::new();
                file.read_to_end(&mut buf)?;
                videos.push((buf, Some(MpegMetadata {})));
            }
        }
        Ok(videos)
    }
}
