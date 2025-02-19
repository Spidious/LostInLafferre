use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use petgraph::{Graph, Undirected};
use petgraph::graph::Node;



// State will be pushed to the open list
struct State{
    node : Node,
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
    start_node: &Node,
    goal_node: &Node,
    nid_gid: &HashMap<usize, (NodeIndex, &Coords)>, 
) -> Option<Vec<NodeIndex>> {
    
    // Get room ids from node references
    let start = nid_gid.get(&start_node.id)?.0;
    let goal = nid_gid.get(&goal_node.id)?.0;

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
    f_score.insert(start, nid_gid[&start_node.id].1.euc_dist(nid_gid[&goal_node.id].1));

    // push start node into open set
    open.push(State { 
        node: start, 
        f: f_score[&start], 
        g: 0.0 
    });

    // A* loop
    while let Some(current) = open.pop() {
        if current.node == goal {
            return Some(reconstruct_path(came_from, goal));
        }

        // Iterate over neighbors
        for edge in graph.edges(current.node) {
            let neighbor = edge.target();
            let cost = *edge.weight();
            let tentative_g_score = g_score[&current.node] + cost;

            // tentative score is better than current score
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f64::INFINITY) {
                came_from.insert(neighbor, current.node);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + nid_gid[&goal_node.id].1.euc_dist(nid_gid[&neighbor.index()].1));

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