use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};


struct Graph {
   adjacency_list: HashMap<i32, Vec<i32>>,
}


impl Graph {
   fn new() -> Self {
       Graph {
           adjacency_list: HashMap::new(),
       }
   }


   fn add_edge(&mut self, src: i32, dst: i32) {
       self.adjacency_list.entry(src).or_insert_with(Vec::new).push(dst);
   }


   // Reads from a TXT file and populates the graph
   fn from_file(file_path: &str) -> io::Result<Self> {
       let file = File::open(file_path)?;
       let reader = BufReader::new(file);
       let mut graph = Graph::new();
  
       for (index, line) in reader.lines().enumerate() {
           let line = line?;
           if index > 0 { // Skip header line
               let parts: Vec<&str> = line.split_whitespace().collect(); // Change this to split on whitespace
               if parts.len() == 2 {
                   match (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                       (Ok(src), Ok(dst)) => graph.add_edge(src, dst),
                       _ => eprintln!("Warning: Invalid line format: {}", line), // Better error handling
                   }
               }
           }
       }
       Ok(graph)
   }
  
   // Converts the graph to a CSV file
   fn to_csv(&self, file_path: &str) -> io::Result<()> {
       let mut file = File::create(file_path)?;
       writeln!(file, "FromNodeId,ToNodeId")?;
       for (src, destinations) in &self.adjacency_list {
           for dst in destinations {
               writeln!(file, "{},{}", src, dst)?;
           }
       }
       Ok(())
   }


   // Breadth-First Search to find shortest paths from start node
   fn bfs(&self, start_node: i32) -> HashMap<i32, i32> {
       let mut distances = HashMap::new();
       let mut queue = VecDeque::new();
       queue.push_back(start_node);
       distances.insert(start_node, 0);


       while let Some(current_node) = queue.pop_front() {
           if let Some(neighbors) = self.adjacency_list.get(&current_node) {
               for &neighbor in neighbors {
                   if !distances.contains_key(&neighbor) {
                       distances.insert(neighbor, distances[&current_node] + 1);
                       queue.push_back(neighbor);
                   }
               }
           }
       }
       distances
   }


   // Depth-First Search to find paths from start node (non-shortest)
   fn dfs(&self, start_node: i32, target_node: i32) -> Option<i32> {
       let mut stack = vec![(start_node, 0)];
       let mut visited = HashMap::new();


       while let Some((current_node, depth)) = stack.pop() {
           if !visited.contains_key(&current_node) {
               if current_node == target_node {
                   return Some(depth);
               }
               visited.insert(current_node, true);
               if let Some(neighbors) = self.adjacency_list.get(&current_node) {
                   for &neighbor in neighbors {
                       if !visited.contains_key(&neighbor) {
                           stack.push((neighbor, depth + 1));
                       }
                   }
               }
           }
       }
       None
   }
}
          


fn main() -> io::Result<()> {
   // Specify the path to your road network data
   let input_path = "roadNet-PA.txt";
   let output_csv_path = "roadNet-PA.csv";
   let start_node = 0;  // Example start node
   let target_node = 6354;  // Example target node for DFS


   // Load the graph from the text file --> export to csv
   let graph = Graph::from_file(input_path)?;
   graph.to_csv(output_csv_path)?;


   // Perform BFS from the start node
   let bfs_distances = graph.bfs(start_node);
   if let Some(distance) = bfs_distances.get(&target_node) {
       println!("BFS: Shortest path from node {} to node {} is {} edges.", start_node, target_node, distance);
   } else {
       println!("BFS: No path found from node {} to node {}", start_node, target_node);
   }


   // Perform DFS from the start node to the target node
   let dfs_result = graph.dfs(start_node, target_node);
   if let Some(depth) = dfs_result {
       println!("DFS: Path found from node {} to node {} with depth {}", start_node, target_node, depth);
   } else {
       println!("DFS: No path found from node {} to node {}", start_node, target_node);
   }


   Ok(())
}


#[cfg(test)]
mod tests {
   use super::*;


   fn create_test_graph() -> Graph {
       let mut graph = Graph::new();
       graph.add_edge(0, 1);
       graph.add_edge(0, 2);
       graph.add_edge(1, 2);
       graph.add_edge(2, 0);
       graph.add_edge(2, 3);
       graph.add_edge(3, 3);
       graph
   }


   #[test]
   fn test_graph_creation() {
       let graph = create_test_graph();
       assert_eq!(graph.adjacency_list.len(), 4);
       assert!(graph.adjacency_list.contains_key(&0));
       assert!(graph.adjacency_list.contains_key(&3));
       assert_eq!(graph.adjacency_list[&2].len(), 2);
   }


   #[test]
   fn test_bfs() {
       let graph = create_test_graph();
       let distances = graph.bfs(2);
       assert_eq!(distances[&0], 1); // Distance from 2 to 0
       assert_eq!(distances[&1], 2); // Distance from 2 to 1
       assert_eq!(distances[&3], 1); // Distance from 2 to 3
       assert_eq!(distances.get(&2), Some(&0)); // Distance from 2 to 2 (itself)
   }


   #[test]
   fn test_dfs() {
       let graph = create_test_graph();
       let depth = graph.dfs(2, 3);
       assert_eq!(depth, Some(1)); // Depth from 2 to 3
   }
}


