use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use crate::dfsbfs::Graph;

/// Reads from a TXT file and populates the graph
pub fn from_file(file_path: &str) -> io::Result<Graph> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut graph = Graph::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if index > 0 { // Skip header line
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                match (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    (Ok(src), Ok(dst)) => graph.add_edge(src, dst),
                    _ => eprintln!("Warning: Invalid line format: {}", line),
                }
            }
        }
    }
    Ok(graph)
}

/// Converts the graph to a CSV file
pub fn to_csv(graph: &Graph, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    writeln!(file, "FromNodeId,ToNodeId")?;
    for (src, destinations) in &graph.adjacency_list {
        for dst in destinations {
            writeln!(file, "{},{}", src, dst)?;
        }
    }
    Ok(())
}