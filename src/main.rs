mod dfsbfs;
mod readparse;

use crate::readparse::{from_file, to_csv};
use std::io;

fn main() -> io::Result<()> {
    let input_path = "roadNet-PA.txt";
    let output_csv_path = "roadNet-PA.csv";
    let start_node = 0;
    let target_node = 6354;

    let graph = from_file(input_path)?;
    to_csv(&graph, output_csv_path)?;

    let bfs_distances = graph.bfs(start_node);
    if let Some(distance) = bfs_distances.get(&target_node) {
        println!("BFS: Shortest path from node {} to node {} is {} edges.", start_node, target_node, distance);
    } else {
        println!("BFS: No path found from node {} to node {} using BFS.", start_node, target_node);
    }

    let dfs_result = graph.dfs(start_node, target_node);
    if let Some(depth) = dfs_result {
        println!("DFS: Path found from node {} to node {} with depth {} using DFS.", start_node, target_node, depth);
    } else {
        println!("DFS: No path found from node {} to node {} using DFS.", start_node, target_node);
    }

    // Select the top 100 nodes based on degree
    let center_nodes = graph.top_degree_nodes(100);
    let average_path_length = graph.average_path_length_subset(&center_nodes);

    println!("Average path length from the top 100 center nodes: {:.2}", average_path_length);

    Ok(())
}