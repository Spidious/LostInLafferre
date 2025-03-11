use petgraph::{Graph, Undirected};
use petgraph::graph::NodeIndex;
use std::f64;
use std::collections::HashMap;
use std::env;
use graph_library::{Coords,create_graph_from_json};
use petgraph::dot::{Dot, Config};
use actix_web::{web, App, Responder, post, get, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use graph_library::find_path;


#[derive(Deserialize)] 
struct InputData {
    content: Value, // Accept any JSON structure
}

#[derive(Serialize)]
struct OutputData {
    processed_content: Value, // Processed JSON data
}


#[get("/rooms/{src}/{dst}")]
async fn rooms(path: web::Path<(u32,String)>) -> impl Responder{
    let (src, dst) = path.into_inner();
    
    HttpResponse::Ok().body(format!("Start {}, End {}!", src, dst))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/tst")]//test get request for app data in server
async fn tst(data: web::Data<AppState>) -> impl Responder{
    let app_state = &data.laffere;
    HttpResponse::Ok().body(format!("{:?}", Dot::with_config(app_state, &[Config::EdgeNoLabel])))
}

struct AppState{
    laffere: Graph::<Coords, f64, Undirected>,
    room_hash: HashMap<String, NodeIndex>,
}

#[actix_web::main]
async fn main()->std::io::Result<()>{

    let args: Vec<String> = env::args().collect();

    let ip: Option<&String> = args.get(1);
    let port_arg: Option<&String> = args.get(2);
    let port: u16 = if port_arg.is_none() {0} else {port_arg.unwrap().parse::<u16>().unwrap()};
    let path: Option<&String> = args.get(3);

    if ip.is_none() || port_arg.is_none() || path.is_none(){
        println!("Check command line arguments");
        println!("Command for running with cargo: cargo run -- 127.0.0.1 8080 nodes_edges.json ");
        println!("Command for running inside debug/release: ./LostInLafferre 127.0.0.1 8080 ../../nodes_edges.json ");
        return Ok(());
    }

    // Inititializes graph and room->graph_id hash maps
    //Reads json file and generates graph
    let mut deps = Graph::<Coords, f64, Undirected>::new_undirected();
    let mut room_gid: HashMap<String, NodeIndex> = HashMap::new();
    let _ = create_graph_from_json(&mut deps, &mut room_gid, &path.unwrap().as_str());

    /***********************************************************************************/
    //test of simple search on small graph
    let src = room_gid.get("106"); 
    let dst = room_gid.get("102");

    let path = find_path(&deps, src.expect("Check the file path"), dst.expect("Check the file path")); // call to the search
    println!("{:?}", path); //prints the path found by A* search


    println!("Graph Nodes and edges"); //prints the current graph out to terminal
    println!("{:?}", Dot::with_config(&deps, &[Config::EdgeNoLabel]));
    println!("Hash Maps keys to indices");
    for (key, value) in &room_gid {
        println!("{} => {:?}", key, value);
    }
    /*********************************************************************************/


    let app_state = AppState{ //AppState used to pass data to the HttpServer
        laffere: deps,
        room_hash: room_gid
    };

    let data = web::Data::new(app_state);

    HttpServer::new(move || {
        //App::new().service(route)})
        App::new()
            .app_data(data.clone()) //Data passed to server
            .service(hello) //default hello world response
            .service(tst) //test to see if passing data to server worked
            .service(rooms) //test for getting room numbers from the URL in GET request
    })
    //.bind(("127.0.0.1",8080))? // Exposes this port to allow POST/GET requests
    .bind((ip.unwrap().as_str(),port))?
    .run()
    .await
}