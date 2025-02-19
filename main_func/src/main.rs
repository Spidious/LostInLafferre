use petgraph::{Graph, Undirected};
use petgraph::graph::NodeIndex;
use std::f64;
use std::collections::HashMap;
use graph_library::Coords;
use graph_library::create_graph_from_json;
use petgraph::dot::{Dot, Config};
use actix_web::{web, App,Responder, post, HttpResponse, HttpServer};
use tokio::fs;

#[derive(Deserialize)]
struct InputData {
    content: Value, // Accept any JSON structure
}

#[derive(Serialize)]
struct OutputData {
    processed_content: Value, // Processed JSON data
}


#[get("/route")]
async fn route(input : web::Json<InputData>)->impl Responder{

    "Routing"

}


#[actix_web::main]
async fn main()->std::io::Result<()>{

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


    HttpServer::new(|| {
        App::new().service(route)})
        
        // Exposes this port to allow POST/GET requests
        .bind(("127.0.0.1",8080))?
        .run()
        .await

}