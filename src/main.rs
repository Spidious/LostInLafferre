use petgraph::{Graph, Undirected};
use petgraph::graph::NodeIndex;
use std::f64;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use petgraph::dot::{Dot, Config};


#[derive(Debug, Serialize, Deserialize, Clone)]
struct Coords(f64, f64, f64);

impl Coords{
    //3D euclidean distance function
    fn euc_dist(&self,other: &Coords) -> f64{
        f64::sqrt((self.0 - other.0)*(self.0 - other.0) +  
                (self.1 - other.1)*(self.1 - other.1) + 
                (self.2 - other.2)*(self.2 - other.2))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Node{
    id: usize,
    rooms: Vec<String>,
    coords: Coords,
    adj: Vec<usize>
}

fn read_json() -> Result<Vec<Node>, Box<dyn std::error::Error>> {
    let mut file = File::open("nodes_edges.json")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    // Deserialize the entire JSON array into a Vec<Node>
    let nodes: Vec<Node> = serde_json::from_str(&data)?;

    Ok(nodes)
}

fn create_graph_from_json(deps: &mut Graph<Coords,f64, Undirected>,room_gid: &mut HashMap<String,NodeIndex>) -> Result<(), Box<dyn std::error::Error>>{
    let nodes = read_json()?;

    //nid: node id
    //gid: graph id
    //room: room string

    //creating helper hash map and vector to create the graph
    let mut nid_gid: HashMap<usize, (NodeIndex, &Coords)> = HashMap::new(); //only used for creating edges

    let mut edges: Vec<(usize,usize)> = Vec::new(); //Vector to hold all edges

    for node in &nodes{
        let node_idx = deps.add_node(node.coords.clone()); //adds coords 
        nid_gid.insert(node.id,(node_idx, &node.coords));

        if node.rooms.len() > 0{ 
            for room in &node.rooms{
                room_gid.insert(room.to_string(),node_idx);
            }
        }
        
        for adj in &node.adj{
            if node.id < *adj{ //makes sure each edge is only added once
                edges.push((node.id,*adj));
            }
        }
    }

    for edge in &edges{
        let src_coords = nid_gid[&edge.0].1;
        let dst_coords = nid_gid[&edge.1].1;
        let weigth = src_coords.euc_dist(&dst_coords);
        deps.add_edge(nid_gid[&edge.0].0,nid_gid[&edge.1].0,weigth);
    }

    Ok(())
}

fn main(){

    // Read the file into a string
    let mut deps = Graph::<Coords, f64, Undirected>::new_undirected();
    let mut room_gid: HashMap<String, NodeIndex> = HashMap::new();
    let _ = create_graph_from_json(&mut deps, &mut room_gid);

    println!("{:?}", Dot::with_config(&deps, &[Config::EdgeNoLabel]));

}