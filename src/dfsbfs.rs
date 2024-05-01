use std::collections::{HashMap, VecDeque};

/// A Graph structure with adjacency list representation
pub struct Graph {
    pub adjacency_list: HashMap<i32, Vec<i32>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, src: i32, dst: i32) {
        self.adjacency_list.entry(src).or_insert_with(Vec::new).push(dst);
    }

    /// Breadth-First Search to find shortest paths from start node
    pub fn bfs(&self, start_node: i32) -> HashMap<i32, i32> {
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

    /// Depth-First Search to find paths from start node (non-shortest)
    pub fn dfs(&self, start_node: i32, target_node: i32) -> Option<i32> {
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

    /// Finds the top `n` nodes with the highest degree.
    pub fn top_degree_nodes(&self, n: usize) -> Vec<i32> {
        let mut node_degrees: Vec<(i32, usize)> = self.adjacency_list.iter()
            .map(|(&node, neighbors)| (node, neighbors.len()))
            .collect();
        
        // Sort by degree descending, then take the top `n`
        node_degrees.sort_by(|a, b| b.1.cmp(&a.1));
        node_degrees.into_iter().take(n).map(|(node, _)| node).collect()
    }

    /// Calculate the average path length from a subset of nodes to all other reachable nodes --> data set is too big
    pub fn average_path_length_subset(&self, nodes: &[i32]) -> f64 {
        let mut total_average = 0.0;
        let mut count = 0;

        for &node in nodes {
            let distances = self.bfs(node);
            let total: i32 = distances.values().sum();
            let num_paths = (distances.len() - 1) as i32; // Exclude start node 

            if num_paths > 0 {
                total_average += total as f64 / num_paths as f64;
                count += 1;
            }
        }

        if count > 0 {
            total_average / count as f64
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        let distances = graph.bfs(1);
        assert_eq!(distances.get(&5), Some(&3));
    }

    #[test]
    fn test_dfs() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);
        graph.add_edge(4, 5);

        let path_depth = graph.dfs(1, 5);
        assert_eq!(path_depth, Some(3));
    }

    // Helper function to create a graph for testing average path length
    fn setup_complex_graph() -> Graph {
        let mut graph = Graph::new();
        // Example complex graph structure
        for i in 1..=10 {
            for j in 1..=10 {
                if i != j {
                    graph.add_edge(i, j);  // Full connectivity among 10 nodes
                }
            }
        }
        graph
    }

    #[test]
    fn test_top_degree_nodes() {
        let graph = setup_complex_graph();
        let top_nodes = graph.top_degree_nodes(5);
        assert_eq!(top_nodes.len(), 5);
        // Since all nodes have equal degree, the specific top nodes can vary
        // but should all have the maximum degree of 9
        for node in top_nodes {
            assert_eq!(graph.adjacency_list.get(&node).unwrap().len(), 9);
        }
    }

    #[test]
    fn test_average_path_length_subset() {
        let graph = setup_complex_graph();
        let top_nodes = graph.top_degree_nodes(5); // Ensure this method itself is correct
        let avg_length = graph.average_path_length_subset(&top_nodes);

        // Diagnostic output if test fails
        if avg_length != 1.0 {
            for node in &top_nodes {
                let distances = graph.bfs(*node);
                let total: i32 = distances.values().sum();
                let num_paths = distances.len() as i32;
                println!("Node {}: Total distances sum = {}, Number of paths = {}, Average = {}", 
                         node, total, num_paths, total as f64 / num_paths as f64);
            }
            panic!("Expected average path length of 1.0, got {}", avg_length);
        }
        assert_eq!(avg_length, 1.0);
    }
}
