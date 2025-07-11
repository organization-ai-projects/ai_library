use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

/// Charge un dataset tabulaire depuis un fichier CSV.
pub fn load_csv(path: &str) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut data = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let row = record
            .iter()
            .filter_map(|s| s.trim().parse::<f32>().ok())
            .collect::<Vec<f32>>();
        data.push(row);
    }
    Ok(data)
}
