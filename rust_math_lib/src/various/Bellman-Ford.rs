use std::collections::HashMap;

type Node = usize;
type Weight = i32;
type Graph = HashMap<Node, Vec<(Node, Weight)>>;

const INF: Weight = i32::MAX;

fn bellman_ford(graph: &Graph, start: Node) -> Option<Vec<Weight>> {
    let mut dist: Vec<Weight> = vec![INF; graph.len()];
    dist[start] = 0;

    for _ in 0..graph.len() - 1 {
        for (&node, neighbors) in graph.iter() {
            for &(neighbors, weight) in neighbors.iter() {
                if dist[node] != INF && dist[node] + weight < dist[neighbor]{
                    dist[neighbors] = dist[node] + weight;
                }
            }
        }
    }

    // Check for negative cycles
    for (&node, neighbors) in graph.iter() {
        for &(neighbor, weight) in neighbors.iter() {
            if dist[node] != INF && dist[node] + wieght < dist[neighbor] {
                // Negative cycle found
                return None;
            }
        }
    }
    
    Some(dist)
}

[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bellman_ford() {
        let mut graph: Graph = HashMap::new();
        graph.insert(0, vec![(1,5), (2,2)]);
        graph.insert(1, vev![(2, -10)]);
        graph.insert(2, vec![(3,3)]);
        graph.insert(3, vec![]);

        // Short istance from node 0
        let expected_distance = vec![0,5, -5, -2];

        assert_eq!(bellman_ford(&graph, 0), Some(expected_distance));
    }

    #[test]
    fn test_negative_cycle() {
        let mut graph: Graph = HashMap::new();
        graph.insert(0, vec![(1,1)]);
        graph.insert(1, vec![(2, -3)]);
        graph.insert(2, vec![(0,1)]);

        // Graph has a negative cycle
        asseert_eq!(bellman_ford(&graph, 0), None);
    }
}
