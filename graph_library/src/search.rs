use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use petgraph::{Graph, Undirected};
use petgraph::graph::NodeIndex;



// State will be pushed to the open list
struct State{
    node : NodeIndex,
    f : f64,
    g : f64
}

// Implement ordering for BinaryHeap (min-heap)
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.partial_cmp(&self.f).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl Eq for State {}

// A* path finding, takes a starting node, an end node, a graph, and a room id hashmap
// Returns the full path
pub fn find_path(
    graph: &Graph<Coords, f64, petgraph::Undirected>,
    start: &NodeIndex,
    goal: &NodeIndex, 
) -> Option<Vec<NodeIndex>> {
    

    // Priority queue for open nodes
    let mut open = BinaryHeap::new();

    // Holds the best path we've found, used for reconstruction
    let mut came_from: HashMap<NodeIndex, NodeIndex> = HashMap::new();

    let mut g_score: HashMap<NodeIndex, f64> = HashMap::new();
    let mut f_score: HashMap<NodeIndex, f64> = HashMap::new();

    // Initialize scores for all nodes
    for node in graph.node_indices() {
        g_score.insert(node, f64::INFINITY);
        f_score.insert(node, f64::INFINITY);
    }

    // set initial scores for the start node
    g_score.insert(start, 0.0);
    // Get coordinate field for start and goal
    f_score.insert(start, graph.weight(NodeIndex).unwrap());

    // push start node into open set
    open.push(State { 
        node: start, 
        f: f_score[&start], 
        g: 0.0 
    });

    // A* loop
    while let Some(current) = open.pop() {
        if current == goal {
            return Some(reconstruct_path(came_from, goal));
        }

        // Iterate over neighbors
        for edge in graph.edges(current) {
            let neighbor = edge.target();
            let cost = *edge.weight();
            let tentative_g_score = g_score[&current] + cost;

            // tentative score is better than current score
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f64::INFINITY) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                
                f_score.insert(neighbor, tentative_g_score + graph.weight(NodeIndex).unwrap());

                // Push neighbor onto open list
                open.push(State {
                    node: neighbor,
                    f: f_score[&neighbor],
                    g: tentative_g_score,
                });
            }
        }
    }
    
    None
}

// Reconstruct the path, takes a hashmap that takes a nodeindex, and a standalone node index, returns a vector of NodeIndecies
fn reconstruct_path(came_from: HashMap<NodeIndex, NodeIndex>, mut current: NodeIndex) -> Vec<NodeIndex> {
    let mut path = vec![current];
    // Get each node and add it to the path
    while let Some(&parent) = came_from.get(&current) {
        path.push(parent);
        current = parent;
    }
    // Reverse and return the path
    path.reverse();
    path
}