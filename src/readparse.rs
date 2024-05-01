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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufWriter, Write};
    use std::fs::{self, File};

    // Helper to create a test input file
    fn setup_input_file() -> String {
        let path = "test_input.txt";
        let mut file = BufWriter::new(File::create(path).unwrap());
        writeln!(file, "Node1 Node2").unwrap();
        writeln!(file, "1 2").unwrap();
        writeln!(file, "2 3").unwrap();
        writeln!(file, "3 4").unwrap();
        file.flush().unwrap();
        path.to_string()
    }

    #[test]
    fn test_from_file() {
        let path = setup_input_file();
        let graph = from_file(&path).unwrap();
        assert_eq!(graph.adjacency_list.len(), 3);
        assert!(graph.adjacency_list.contains_key(&1));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_to_csv() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);

        let path = "test_output.csv";
        to_csv(&graph, path).unwrap();

        let contents = fs::read_to_string(path).unwrap();
        assert!(contents.contains("1,2"));
        assert!(contents.contains("1,3"));
        assert!(contents.contains("2,4"));
        fs::remove_file(path).unwrap();
    }
}
