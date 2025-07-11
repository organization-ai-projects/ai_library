use mp4parse::{read_mp4, MediaContext};
use std::fs;
use std::io::Read;

/// Métadonnées extraites d'un fichier MP4
#[derive(Debug, Clone)]
pub struct Mp4Metadata {
    pub duration: Option<u64>,
    pub width: Option<u16>,
    pub height: Option<u16>,
    pub timescale: Option<u32>,
}

/// Charge toutes les vidéos MP4 d’un dossier (lecture brute du fichier + métadonnées).
/// Retourne le contenu binaire et les métadonnées de chaque vidéo MP4.
pub fn load_mp4_videos(
    folder: &str,
) -> Result<Vec<(Vec<u8>, Option<Mp4Metadata>)>, Box<dyn std::error::Error>> {
    let mut videos = Vec::new();
    for entry in fs::read_dir(folder)? {
        let path = entry?.path();
        if path.is_file() && path.extension().map_or(false, |e| e == "mp4") {
            let mut file = std::fs::File::open(&path)?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;

            // Extraction des métadonnées avec mp4parse
            let meta = match read_mp4(&mut &buf[..]) {
                Ok(ctx) => {
                    let mut duration = None;
                    let mut width = None;
                    let mut height = None;
                    let mut timescale = None;
                    for track in &ctx.tracks {
                        if track.track_type == mp4parse::TrackType::Video {
                            if let Some(ref tkhd) = track.tkhd {
                                width = Some((tkhd.width >> 16) as u16);
                                height = Some((tkhd.height >> 16) as u16);
                            }
                            // mp4parse ne fournit pas toujours duration/timescale par track
                            // On laisse à None si non disponible
                        }
                    }
                    Some(Mp4Metadata {
                        duration,
                        width,
                        height,
                        timescale,
                    })
                }
                Err(_) => None,
            };
            videos.push((buf, meta));
        }
    }
    Ok(videos)
}
