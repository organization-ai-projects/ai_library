use hound;
use std::fs;

/// Charge tous les fichiers WAV d’un dossier.
pub fn load_wav_audio(folder: &str) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
    let mut audios = Vec::new();
    for entry in fs::read_dir(folder)? {
        let path = entry?.path();
        if path.is_file() && path.extension().map_or(false, |e| e == "wav") {
            let mut reader = hound::WavReader::open(&path)?;
            let samples: Vec<f32> = reader
                .samples::<i16>()
                .filter_map(Result::ok)
                .map(|s| s as f32 / i16::MAX as f32)
                .collect();
            audios.push(samples);
        }
    }
    Ok(audios)
}
