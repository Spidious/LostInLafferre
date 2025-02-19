use petgraph::{Graph, Undirected};
use petgraph::graph::NodeIndex;
use std::f64;
use std::collections::HashMap;
use graph_library::Coords;
use graph_library::create_graph_from_json;
use petgraph::dot::{Dot, Config};
use actix_web;



async fn main(){

    let path = "nodes_edges.json";
    // Read the file into a string
    let mut deps = Graph::<Coords, f64, Undirected>::new_undirected();
    let mut room_gid: HashMap<String, NodeIndex> = HashMap::new();
    let _ = create_graph_from_json(&mut deps, &mut room_gid, &path);

    println!("Graph Nodes and edges");
    println!("{:?}", Dot::with_config(&deps, &[Config::EdgeNoLabel]));
    println!("Hash Maps keys to indices");
    for (key, value) in &room_gid {
        println!("{} => {:?}", key, value);
    }

}