use std::fs::File;
use std::io::{BufRead, BufReader};

/// Charge un dataset de graphes depuis un fichier edge list (from,to,weight par ligne).
pub fn load_edge_list_graphs(path: &str) -> Result<Vec<Vec<(usize, usize, f32)>>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut graphs = Vec::new();
    let mut current_graph = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            if !current_graph.is_empty() {
                graphs.push(current_graph.clone());
                current_graph.clear();
            }
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            let from = parts[0].trim().parse().unwrap_or(0);
            let to = parts[1].trim().parse().unwrap_or(0);
            let weight = parts[2].trim().parse().unwrap_or(0.0);
            current_graph.push((from, to, weight));
        }
    }
    if !current_graph.is_empty() {
        graphs.push(current_graph);
    }
    Ok(graphs)
}
