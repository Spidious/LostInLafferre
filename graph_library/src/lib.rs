use petgraph::{Graph, Undirected};
use petgraph::graph::NodeIndex;
use std::f64;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader,Read};
use petgraph::visit::EdgeRef;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use serde_json::from_reader;


//Consts for if changingthe values between floors
const BASEMENT: f64 = 0.0;
const FIRST: f64 = 50.0;
const SECOND: f64 = 100.0;
const THIRD: f64 = 150.0;

//-----------------------------------------------------------------------------------------------------------
//- Library functions for reading json file into the graph
//
//

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coords{
    x: f64, 
    y: f64, 
    z: f64
}//coords are stored as a struct of x,y,z coordinates

#[derive(Debug, Serialize, Deserialize)]
struct Adj{
    target: usize,
    cost: f64
}

#[derive(Debug, Serialize, Deserialize)]
struct NodeList {
    nodes: Vec<Node>,
}

impl Coords{ //used for calculating heuristic 
    //Note:
    //- currently used when creating edge weights when creating graph (will most likely change depending on how actually graph formatted in json)
    //3D euclidean distance function
    /*fn euc_dist(&self,other: &Coords) -> f64{
        f64::sqrt((self.x - other.x)*(self.x - other.x) +  
                (self.y - other.y)*(self.y - other.y) + 
                (self.z - other.z)*(self.z - other.z))
    }*/

    fn euc_dist(&self,other: &Coords) -> f64{
        f64::sqrt((self.x - other.x)*(self.x - other.x) +  
                (self.y - other.y)*(self.y - other.y)) + 
                f64::abs(self.z - other.z)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Node{ //struct used only for reading json objects into rust
    id: usize,
    coordinates: Coords,
    room_names: Vec<String>,
    connections: Vec<Adj>
}

fn read_json(path: &str) -> Result<NodeList, Box<dyn std::error::Error>> { //json file into a vector of node structs
    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    // Deserialize the entire JSON array into a Vec<Node>
    let nodes: NodeList = serde_json::from_str(&data)?;

    Ok(nodes)
}

//functions for creating graph from json
pub fn create_graph_from_json(
    deps: &mut Graph<Coords,f64, Undirected>,
    room_gid: &mut HashMap<String,NodeIndex>, //hash map Key: Room number Value: NodeIndex
    path: &str
    ) -> Result<(), Box<dyn std::error::Error>>{

    let node_list = read_json(path)?;

    println!("{}",node_list.nodes.len());

    //nid: node id
    //gid: graph id
    //room: room string

    //creating helper hash map and vector to create the graph
    let mut nid_gid: HashMap<usize, NodeIndex> = HashMap::new(); //only used for creating edges

    let mut edges: Vec<(usize,usize,f64)> = Vec::new(); //Vector to hold all edges

    for node in &node_list.nodes{
        let node_idx = deps.add_node(node.coordinates.clone()); //adds coords 
        nid_gid.insert(node.id,node_idx);

        if node.room_names.len() > 0{ 
            for room in &node.room_names{
                room_gid.insert(room.to_string(),node_idx);
            }
        }
        
        for adj in &node.connections{
            if node.id < adj.target{ //makes sure each edge is only added once
                edges.push((node.id, adj.target, adj.cost));
            }
        }
    }

    for edge in &edges{
        deps.add_edge(nid_gid[&edge.0],nid_gid[&edge.1],edge.2);
    }

    Ok(())
}

//-------------------------------------------------------------------------------------\
//- Library functions for A* saerch
//
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
) -> Option<Vec<Coords>> {
    

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
    g_score.insert(*start, 0.0);
    // Get coordinate field for start and goal
    f_score.insert(*start, graph.node_weight(*start).unwrap().euc_dist(graph.node_weight(*goal).unwrap()));

    // push start node into open set
    open.push(State { 
        node: *start, 
        f: f_score[start], 
        g: 0.0 
    });

    // A* loop
    while let Some(current) = open.pop() {
        if current.node == *goal {
            return Some(reconstruct_path(came_from, *goal, graph));
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
                
                f_score.insert(neighbor, tentative_g_score + graph.node_weight(neighbor).unwrap().euc_dist(graph.node_weight(*goal).unwrap()));

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
//
// Reconstruct the path, takes a hashmap that takes a nodeindex, and a standalone node index, returns a vector of NodeIndecies
fn reconstruct_path(came_from: HashMap<NodeIndex, NodeIndex>, mut current: NodeIndex, graph: &Graph<Coords, f64, petgraph::Undirected>) -> Vec<Coords> {
    let mut path = vec![graph.node_weight(current).unwrap().clone()];
    // Get each node and add it to the path
    while let Some(&parent) = came_from.get(&current) {
        path.push(graph.node_weight(parent).unwrap().clone());
        current = parent;
    }
    // Reverse and return the path
    path.reverse();
    path
}