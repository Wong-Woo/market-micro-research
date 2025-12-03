use axum::{
    routing::get,
    Router,
    response::Json,
};
use serde_json::json;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello",get(hello));

    let addr = SocketAddr::from(([127,0,0,1],3000));
    println!("listening on {}",addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn hello() -> Json<serde_json::Value> {
     Json(json!({ "message": "Hello World! " }))
}
