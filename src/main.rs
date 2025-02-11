use actix_web::{web, App,Responder, post, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::Value;




#[derive(Deserialize)]
struct InputData {
    content: Value, // Accept any JSON structure
}

#[derive(Serialize)]
struct OutputData {
    processed_content: Value, // Processed JSON data
}



//process the request
#[post("/route")]
async fn route(input : web::Json<InputData>)->impl Responder{


    let json_coordinates = &input.content;
    

    HttpResponse::Ok().await
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new().service(route)})
        
        // Exposes this port to allow POST/GET requests
        .bind(("127.0.0.1",8080))?
        .run()
        .await
}

