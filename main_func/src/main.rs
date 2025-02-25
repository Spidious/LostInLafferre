use petgraph::{Graph, Undirected};
use petgraph::graph::NodeIndex;
use std::f64;
use std::collections::HashMap;
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
/* 
Notes for input data 
Ways to take input data
    - take input data from the url like in the rooms funciton below
    e.x /rooms/{src}/{dst}

*/

#[derive(Serialize)]
struct OutputData {
    processed_content: Value, // Processed JSON data
}


#[get("/route")]
async fn route(input : web::Json<InputData>)-> impl Responder{

    "Routing"

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

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/tst")]
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

    let path = "nodes_edges.json";
    // Read the file into a string
    let mut deps = Graph::<Coords, f64, Undirected>::new_undirected();
    let mut room_gid: HashMap<String, NodeIndex> = HashMap::new();
    let _ = create_graph_from_json(&mut deps, &mut room_gid, &path);

    //let test_coords = deps.node_weight(room_gid["105"]);
    //println!("{:?}",test_coords.unwrap());

    let src = room_gid["106"];
    let dst = room_gid["102"];

    let path = find_path(&deps, &src, &dst);

    println!("{:?}", path);




    println!("Graph Nodes and edges");
    println!("{:?}", Dot::with_config(&deps, &[Config::EdgeNoLabel]));
    println!("Hash Maps keys to indices");
    for (key, value) in &room_gid {
        println!("{} => {:?}", key, value);
    }



    let app_state = AppState{
        laffere: deps,
        room_hash: room_gid
    };

    let data = web::Data::new(app_state);

    HttpServer::new(move || {
        //App::new().service(route)})
        App::new()
            .app_data(data.clone())
            .service(hello)
            .service(tst)
            .service(echo)
            .service(route)
            .service(rooms)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1",8080))? // Exposes this port to allow POST/GET requests
    .run()
    .await
}